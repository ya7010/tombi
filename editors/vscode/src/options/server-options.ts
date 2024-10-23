import type * as node from "vscode-languageclient/node";

export function serverOptions(serverPath: string): node.ServerOptions {
  const run = {
    command: serverPath,
  };

  return {
    run,
    debug: run,
  };
}
