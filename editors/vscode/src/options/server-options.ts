import type * as node from "vscode-languageclient/node";

export function serverOptions(serverPath: string): node.ServerOptions {
  const run = {
    command: serverPath,
    args: ["serve"],
  };

  return {
    run,
    debug: run,
  };
}
