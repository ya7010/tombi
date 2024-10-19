import type * as vscode from "vscode";
import { Extension } from "@/extention";
import { bootstrap } from "@/bootstrap";
import { Server } from "@/lsp/server";
import { showServerVersion } from "./command/show-server-version";

let extention: Extension;

export async function activate(
  context: vscode.ExtensionContext,
): Promise<void> {
  const serverPath = await bootstrap(context, {});
  const server = new Server(serverPath);
  await showServerVersion(server);
  if (!extention) extention = new Extension(context, server);
}

export async function deactivate(): Promise<void> {
  extention?.deactivate();
}
