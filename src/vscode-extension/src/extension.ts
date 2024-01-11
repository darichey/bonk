import * as vscode from "vscode";

export function activate(context: vscode.ExtensionContext) {
  let disposable = vscode.commands.registerCommand("bonk.helloWorld", () => {
    vscode.window.showInformationMessage("Hello World from bonk!");
  });

  context.subscriptions.push(disposable);
}
