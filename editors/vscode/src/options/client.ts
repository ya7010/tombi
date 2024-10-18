import * as vscode from "vscode";
import type * as languageclient from "vscode-languageclient";

export function makeLanguageOptions(
  workspaceFolder?: vscode.WorkspaceFolder,
): languageclient.LanguageClientOptions {
  return {
    documentSelector: [
      {
        language: "toml",
        pattern: "*.toml",
      },
    ],
    workspaceFolder,
    synchronize: {
      // Notify the server about file changes to tomy.toml and JSON files contained in the workspace
      fileEvents: [
        vscode.workspace.createFileSystemWatcher("**/tomy.toml"),
        vscode.workspace.createFileSystemWatcher("**/pyproject.toml"),
      ],
    },
  };
}
