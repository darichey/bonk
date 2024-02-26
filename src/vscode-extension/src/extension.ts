import * as vscode from "vscode";
import * as fs from "fs/promises";

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
  const bonkCfg = await findBonkCfg();

  const serverCommand = getServerCommand();
  if (!serverCommand) {
    output.appendLine(`No path to lsp binary, not activating`);
    return;
  } else {
    output.appendLine(`Using LSP binary: ${serverCommand}`);
  }

  const serverOptions: ServerOptions = {
    run: {
      command: serverCommand.cmd,
      args: [...serverCommand.args, bonkCfg],
      transport: TransportKind.stdio,
    },
    debug: {
      command: serverCommand.cmd,
      args: [...serverCommand.args, bonkCfg],
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

async function findBonkCfg(): Promise<string> {
  const workspace = vscode.workspace.workspaceFolders?.[0];
  if (!workspace) {
    // TODO: support cases where Bonk.toml is not in root of the current workspace
    throw new Error("must have a workspace with Bonk.toml in root open");
  }

  const bonkCfg = `${workspace.uri.fsPath}/Bonk.toml`;

  await fs.access(bonkCfg, fs.constants.F_OK);

  return bonkCfg;
}

function getServerCommand(): { cmd: string; args: string[] } | undefined {
  const config = vscode.workspace.getConfiguration("bonk");
  const server =
    config.get<string | undefined>("server") ??
    process.env["__BONK_LSP_SERVER_DEBUG"];

  if (server) {
    const [cmd, ...args] = server.split(" ");
    return {
      cmd,
      args,
    };
  }

  return;
}
