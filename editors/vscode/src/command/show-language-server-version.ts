import * as vscode from "vscode";
import type { Server } from "../lsp/server";

export async function showLanguageServerVersion(server: Server): Promise<void> {
  const version = await server.showVersion();
  const source = server.tombiBin.source === "bundled" ? " (bundled)" : "";

  vscode.window.showInformationMessage(
    `Tombi Language Server Version: ${version}${source}`,
  );
}
