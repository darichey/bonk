import * as vscode from "vscode";

import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export function activate(_context: vscode.ExtensionContext) {
  const output = vscode.window.createOutputChannel("Bonk");
  const traceOutputChannel =
    vscode.window.createOutputChannel("Bonk LSP Trace");

  const serverCommand = process.env["__BONK_LSP_SERVER_DEBUG"];
  if (!serverCommand) {
    output.appendLine(`No path to lsp binary, not activating`);
    return;
  } else {
    output.appendLine(`Using LSP binary: ${serverCommand}`);
  }

  const serverOptions: ServerOptions = {
    run: {
      command: serverCommand,
      transport: TransportKind.stdio,
    },
    debug: {
      command: serverCommand,
      transport: TransportKind.stdio,
    },
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: "file", language: "bonk" }],
    traceOutputChannel,
  };

  client = new LanguageClient(
    "bonk",
    "Bonk Language Server",
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
