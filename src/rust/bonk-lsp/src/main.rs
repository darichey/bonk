mod diagnostic;
mod go_to_def;
mod state;
mod util;

use std::error::Error;

use lsp_types::notification::{DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument};
use lsp_types::request::{DocumentDiagnosticRequest, GotoDefinition};
use lsp_types::{
    DiagnosticOptions, DiagnosticServerCapabilities, DocumentDiagnosticReport,
    FullDocumentDiagnosticReport, GotoDefinitionResponse, OneOf,
    RelatedFullDocumentDiagnosticReport, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncOptions, WorkDoneProgressOptions,
};
use lsp_types::{InitializeParams, ServerCapabilities};

use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId, Response};

use crate::diagnostic::get_diagnostics;
use crate::go_to_def::get_go_to_def_result;
use crate::state::State;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    eprintln!("Starting...");

    // Create the transport. Includes the stdio (stdin and stdout) versions but this could
    // also be implemented to use sockets or HTTP.
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    let server_capabilities = serde_json::to_value(ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(
            TextDocumentSyncOptions {
                open_close: Some(true),
                change: Some(TextDocumentSyncKind::INCREMENTAL),
                will_save: Some(false),
                will_save_wait_until: Some(false),
                save: None,
            },
        )),
        diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
            identifier: Some("bonk".to_string()),
            inter_file_dependencies: false,
            workspace_diagnostics: false,
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(false),
            },
        })),
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    })
    .unwrap();
    let initialization_params = match connection.initialize(server_capabilities) {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down...");
    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();

    let mut state = State::new();

    eprintln!("Starting main loop...");
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                eprintln!("Received request {} ({})", req.method, req.id);
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                let req = match cast_req::<DocumentDiagnosticRequest>(req) {
                    Ok((id, params)) => {
                        if let Some("bonk") = params.identifier.as_deref() {
                            let doc = state
                                .get_ledger(&params.text_document.uri)
                                .expect("we don't know about this file");

                            let result = DocumentDiagnosticReport::Full(
                                RelatedFullDocumentDiagnosticReport {
                                    related_documents: None,
                                    full_document_diagnostic_report: FullDocumentDiagnosticReport {
                                        result_id: None,
                                        items: get_diagnostics(
                                            &doc.ledger,
                                            &doc.src,
                                            params
                                                .text_document
                                                .uri
                                                .to_file_path()
                                                .as_deref()
                                                .unwrap(),
                                        ),
                                    },
                                },
                            );
                            let result = serde_json::to_value(&result).unwrap();
                            let resp = Response {
                                id,
                                result: Some(result),
                                error: None,
                            };
                            connection.sender.send(Message::Response(resp))?;
                        }
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };

                let _req = match cast_req::<GotoDefinition>(req) {
                    Ok((id, params)) => {
                        let params = params.text_document_position_params;
                        let doc = state
                            .get_ledger(&params.text_document.uri)
                            .expect("we don't know about this file");

                        let result = get_go_to_def_result(
                            &doc.ledger,
                            &doc.src,
                            params.text_document.uri,
                            params.position,
                        )
                        .map(GotoDefinitionResponse::Scalar);
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
            }
            Message::Response(resp) => {
                eprintln!("Received response {}", resp.id);
            }
            Message::Notification(not) => {
                eprintln!("Received notification {}", not.method);

                let not = match cast_not::<DidOpenTextDocument>(not) {
                    Ok(params) => {
                        state.on_open(params.text_document.uri, params.text_document.text);
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(not)) => not,
                };

                let not = match cast_not::<DidChangeTextDocument>(not) {
                    Ok(params) => {
                        state.on_change(&params.text_document.uri, params.content_changes);
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(not)) => not,
                };

                let _not = match cast_not::<DidCloseTextDocument>(not) {
                    Ok(params) => {
                        state.on_close(&params.text_document.uri);
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(not)) => not,
                };
            }
        }
    }
    Ok(())
}

fn cast_req<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}

fn cast_not<N>(not: Notification) -> Result<N::Params, ExtractError<Notification>>
where
    N: lsp_types::notification::Notification,
    N::Params: serde::de::DeserializeOwned,
{
    not.extract(N::METHOD)
}
