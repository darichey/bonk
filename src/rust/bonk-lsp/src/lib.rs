pub mod cli;
mod code_actions;
mod complete;
mod diagnostic;
mod go_to_def;
mod reference;
mod state;
mod util;

use std::path::PathBuf;

use bonk_parse::WorkspaceExt;
use bonk_workspace::Workspace;
use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId, Response};
use lsp_types::notification::{DidChangeTextDocument, DidOpenTextDocument};
use lsp_types::request::{
    CodeActionRequest, Completion, DocumentDiagnosticRequest, GotoDefinition, References,
};
use lsp_types::{
    CodeActionProviderCapability, CompletionList, CompletionOptions, DiagnosticOptions,
    DiagnosticServerCapabilities, DocumentDiagnosticReport, FullDocumentDiagnosticReport,
    GotoDefinitionResponse, RelatedFullDocumentDiagnosticReport, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    WorkDoneProgressOptions,
};
use lsp_types::{InitializeParams, OneOf};

use crate::code_actions::get_code_actions;
use crate::complete::get_completion_results;
use crate::diagnostic::get_doc_diagnostics;
use crate::go_to_def::get_go_to_def_result;
use crate::reference::get_references;
use crate::state::State;

fn run_server(cfg_path: PathBuf) -> anyhow::Result<()> {
    let workspace = Workspace::from_cfg(cfg_path).expect("failed to parse workspace");

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
            inter_file_dependencies: true,
            workspace_diagnostics: true, // FIXME: we should respond to workspace/diagnostic requests
            work_done_progress_options: WorkDoneProgressOptions {
                work_done_progress: Some(false),
            },
        })),
        completion_provider: Some(CompletionOptions {
            ..Default::default()
        }),
        definition_provider: Some(OneOf::Left(true)),
        code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
        references_provider: Some(OneOf::Left(true)),
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
    main_loop(connection, initialization_params, workspace)?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("Shutting down...");

    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
    workspace: Workspace,
) -> anyhow::Result<()> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();

    let workspace = workspace.parse().expect("couldn't read ledgers");
    let mut state = State::new(workspace);

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
                            let items = get_doc_diagnostics(&state, &params.text_document.uri);

                            let result = DocumentDiagnosticReport::Full(
                                RelatedFullDocumentDiagnosticReport {
                                    related_documents: None,
                                    full_document_diagnostic_report: FullDocumentDiagnosticReport {
                                        result_id: None,
                                        items,
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

                let req = match cast_req::<GotoDefinition>(req) {
                    Ok((id, params)) => {
                        let params = params.text_document_position_params;
                        let doc = state
                            .get_ledger(&params.text_document.uri)
                            .expect("we don't know about this file");

                        let result =
                            get_go_to_def_result(&state, &doc.ledger, &doc.src, params.position)
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

                let req = match cast_req::<Completion>(req) {
                    Ok((id, params)) => {
                        let params = params.text_document_position;
                        let doc = state
                            .get_ledger(&params.text_document.uri)
                            .expect("we don't know about this file");

                        let items =
                            get_completion_results(&state, &doc.ledger, &doc.src, params.position);

                        let result = CompletionList {
                            is_incomplete: false,
                            items,
                        };
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

                let req = match cast_req::<CodeActionRequest>(req) {
                    Ok((id, params)) => {
                        let doc = state
                            .get_ledger(&params.text_document.uri)
                            .expect("we don't know about this file");

                        let result = get_code_actions(&state, &doc.ledger, &doc.src, params.range);

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

                let _req = match cast_req::<References>(req) {
                    Ok((id, params)) => {
                        let params = params.text_document_position;
                        let doc = state
                            .get_ledger(&params.text_document.uri)
                            .expect("we don't know about this file");

                        let result = get_references(&state, &doc.ledger, &doc.src, params.position);

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

                let _not = match cast_not::<DidChangeTextDocument>(not) {
                    Ok(params) => {
                        state.on_change(&params.text_document.uri, params.content_changes);
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
