import type * as node from "vscode-languageclient/node";
import type * as vscode from "vscode";

export function serverOptions(
  context: vscode.ExtensionContext,
  serverPath: string,
): node.ServerOptions {
  const run = {
    command: serverPath,
  };

  return {
    run,
    debug: run,
  };
}
