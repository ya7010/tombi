import type { Settings } from "@/extention";
import type * as node from "vscode-languageclient/node";

export function serverOptions(
  serverPath: string,
  settings: Settings,
): node.ServerOptions {
  let serveArgs: string[] = settings.args ?? [];
  if (settings.tomlVersion && serveArgs.indexOf("--toml-version") === -1) {
    serveArgs = serveArgs.concat(["--toml-version", settings.tomlVersion]);
  }

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
