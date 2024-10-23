import * as vscode from "vscode";
import type { Server } from "../lsp/server";

export async function showServerVersion(server: Server): Promise<void> {
  const version = await server.showVersion();

  vscode.window.showInformationMessage(`tombi-lsp version: ${version}`);
}
