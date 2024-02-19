use bonk_workspace::Workspace;
use clap::Parser;
use lsp_types::ServerCapabilities;
use lsp_types::{
    CompletionOptions, DiagnosticOptions, DiagnosticServerCapabilities, OneOf,
    TextDocumentSyncCapability, TextDocumentSyncKind, TextDocumentSyncOptions,
    WorkDoneProgressOptions,
};
use std::error::Error;
use std::path::PathBuf;

use lsp_server::Connection;

use crate::main_loop;

#[derive(Parser)]
#[command()]
pub struct Args {
    /// Path to the Bonk workspace config.
    pub cfg_path: PathBuf,

    #[arg(long)]
    pub stdio: bool,
}

pub fn run(args: Args) -> Result<(), Box<dyn Error + Send + Sync>> {
    let Args { cfg_path, .. } = args;

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
