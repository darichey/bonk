mod diagnostic;
mod state;

use std::error::Error;

use lsp_types::notification::{DidChangeTextDocument, DidCloseTextDocument, DidOpenTextDocument};
use lsp_types::request::DocumentDiagnosticRequest;
use lsp_types::{
    Diagnostic, DiagnosticOptions, DiagnosticServerCapabilities, DiagnosticSeverity,
    DocumentDiagnosticReport, FullDocumentDiagnosticReport, Position, Range,
    RelatedFullDocumentDiagnosticReport, TextDocumentSyncCapability, TextDocumentSyncKind,
    TextDocumentSyncOptions, WorkDoneProgressOptions,
};
use lsp_types::{InitializeParams, ServerCapabilities};

use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId, Response};

use crate::state::State;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // Note that  we must have our logging only write out to stderr.
    eprintln!("starting generic LSP server");

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
    eprintln!("shutting down server");
    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();

    let mut state = State::new();

    eprintln!("starting example main loop");
    for msg in &connection.receiver {
        eprintln!("got msg: {msg:?}");
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                eprintln!("got request: {req:?}");
                let _req = match cast_req::<DocumentDiagnosticRequest>(req) {
                    Ok((id, params)) => {
                        if let Some("bonk") = params.identifier.as_deref() {
                            eprintln!("got textDocument/diagnostic request #{id}: {params:?}");
                            let result = DocumentDiagnosticReport::Full(
                                RelatedFullDocumentDiagnosticReport {
                                    related_documents: None,
                                    full_document_diagnostic_report: FullDocumentDiagnosticReport {
                                        result_id: None,
                                        items: vec![],
                                        // items: vec![Diagnostic {
                                        //     range: Range {
                                        //         start: Position {
                                        //             line: 0,
                                        //             character: 0,
                                        //         },
                                        //         end: Position {
                                        //             line: 0,
                                        //             character: 10,
                                        //         },
                                        //     },
                                        //     severity: Some(DiagnosticSeverity::ERROR),
                                        //     code: None,
                                        //     code_description: None,
                                        //     source: Some("bonk".to_string()),
                                        //     message: "this is an error".to_string(),
                                        //     related_information: None,
                                        //     tags: None,
                                        //     data: None,
                                        // }],
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
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                eprintln!("got notification: {not:?}");

                let not = match cast_not::<DidOpenTextDocument>(not) {
                    Ok(params) => {
                        state.on_open(
                            params.text_document.uri.to_string(),
                            params.text_document.text,
                        );
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(not)) => not,
                };

                let not = match cast_not::<DidChangeTextDocument>(not) {
                    Ok(params) => {
                        state.on_change(
                            params.text_document.uri.to_string(),
                            params.content_changes,
                        );
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => panic!("{err:?}"),
                    Err(ExtractError::MethodMismatch(not)) => not,
                };

                let _not = match cast_not::<DidCloseTextDocument>(not) {
                    Ok(params) => {
                        state.on_close(params.text_document.uri.to_string());
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
