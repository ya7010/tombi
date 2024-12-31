import type { Settings } from "@/extention";
import type * as node from "vscode-languageclient/node";

export function serverOptions(
  serverPath: string,
  settings: Settings,
): node.ServerOptions {
  const serveArgs: string[] = settings.args ?? [];

  const run = {
    command: serverPath,
    args: ["serve", ...serveArgs],
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
