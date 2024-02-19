pub mod cli;
mod complete;
mod diagnostic;
mod go_to_def;
mod state;
mod util;

use anyhow::Result;
use bonk_parse::WorkspaceExt;
use bonk_workspace::Workspace;
use lsp_server::{Connection, ExtractError, Message, Notification, Request, RequestId, Response};
use lsp_types::notification::{DidChangeTextDocument, DidOpenTextDocument};
use lsp_types::request::{Completion, DocumentDiagnosticRequest, GotoDefinition};
use lsp_types::InitializeParams;
use lsp_types::{
    CompletionList, DocumentDiagnosticReport, FullDocumentDiagnosticReport, GotoDefinitionResponse,
    RelatedFullDocumentDiagnosticReport,
};

use crate::complete::get_completion_results;
use crate::diagnostic::get_doc_diagnostics;
use crate::go_to_def::get_go_to_def_result;
use crate::state::State;

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
    workspace: Workspace,
) -> Result<()> {
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

                let _req = match cast_req::<Completion>(req) {
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
