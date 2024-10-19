import * as vscode from "vscode";
import type { Server } from "../lsp/server";

export async function showVersion(server: Server): Promise<void> {
  const version = await server.showVersion();

  vscode.window.showInformationMessage(`toml-lsp version: ${version}`);
}
