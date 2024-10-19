import * as vscode from "vscode";
import type * as languageclient from "vscode-languageclient";

export function clientOptions(
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
      // Notify the server about file changes to tomkit.toml and JSON files contained in the workspace
      fileEvents: [
        vscode.workspace.createFileSystemWatcher("**/tomkit.toml"),
        vscode.workspace.createFileSystemWatcher("**/pyproject.toml"),
      ],
    },
  };
}
