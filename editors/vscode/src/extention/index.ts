import * as vscode from "vscode";
import * as node from "vscode-languageclient/node";
import { clientOptions } from "../options/client-options";
import { serverOptions } from "../options/server-options";
import type { Settings } from "./settings";
export type { Settings };

export class Extension {
  constructor(
    private context: vscode.ExtensionContext,
    private serverPath: string,
    private client?: node.LanguageClient,
  ) {
    for (const document of vscode.workspace.textDocuments) {
      this.onDidOpentextDocument({ document });
    }

    vscode.workspace.onDidChangeTextDocument((event) =>
      this.onDidOpentextDocument(event),
    );

    vscode.workspace.onDidChangeConfiguration(
      async (event) => {
        await this.client?.sendNotification(
          node.DidChangeConfigurationNotification.type,
          {
            settings: "toml-toolkit", // For event test
          },
        );
      },
      null,
      this.context.subscriptions,
    );
  }

  deactivate(): Thenable<void> | undefined {
    return this.client?.stop();
  }

  onDidOpentextDocument({ document }: { document: vscode.TextDocument }): void {
    if (document.languageId !== "toml") return;

    if (!this.client) {
      const workspaceFolder = vscode.workspace.getWorkspaceFolder(document.uri);

      this.client = new node.LanguageClient(
        "tomy",
        serverOptions(this.context),
        clientOptions(workspaceFolder),
      );

      this.client.start();
    }
  }
}
