import * as vscode from "vscode";
import * as node from "vscode-languageclient/node";

import { clientOptions } from "./options/client";
import { serverOptions } from "./options/server-options";

export class Extension {
  constructor(
    private context: vscode.ExtensionContext,
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
            settings: "tomy", // For event test
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
