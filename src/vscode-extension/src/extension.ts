import * as vscode from "vscode";

import {
  LanguageClient,
  LanguageClientOptions,
  ServerOptions,
  TransportKind,
} from "vscode-languageclient/node";

let client: LanguageClient | undefined;

export async function activate(_context: vscode.ExtensionContext) {
  const output = vscode.window.createOutputChannel("Bonk");
  const traceOutputChannel =
    vscode.window.createOutputChannel("Bonk LSP Trace");

  // TODO: track changes to the Bonk.toml and workspaces

  const serverCommand = getServerCommand();
  if (!serverCommand) {
    output.appendLine(`No path to lsp binary, not activating`);
    return;
  } else {
    output.appendLine(`Using LSP binary: ${JSON.stringify(serverCommand)}`);
  }

  const serverOptions: ServerOptions = {
    run: {
      command: serverCommand.cmd,
      args: serverCommand.args,
      transport: TransportKind.stdio,
    },
    debug: {
      command: serverCommand.cmd,
      args: serverCommand.args,
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

function getServerCommand(): { cmd: string; args: string[] } | undefined {
  const config = vscode.workspace.getConfiguration("bonk");
  const server =
    process.env["__BONK_LSP_SERVER_DEBUG"] ??
    config.get<string | undefined>("server");

  if (server) {
    const [cmd, ...args] = server.split(" ");
    return {
      cmd,
      args,
    };
  }

  return;
}
