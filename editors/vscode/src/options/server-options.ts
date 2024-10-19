import * as node from "vscode-languageclient/node";
import type * as vscode from "vscode";

export function serverOptions(
  context: vscode.ExtensionContext,
): node.ServerOptions {
  const run = {
    module: context.asAbsolutePath("dist/server.js"),
    transport: node.TransportKind.ipc,
  };

  return {
    run,
    debug: run,
  };
}
