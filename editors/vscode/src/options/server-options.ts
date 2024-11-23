import type * as node from "vscode-languageclient/node";

export function serverOptions(serverPath: string): node.ServerOptions {
  const run = {
    command: serverPath,
    args: ["serve"],
    options: {
      env: {
        NO_COLOR: "1",
      },
    },
  };

  return {
    run,
    debug: run,
  };
}
