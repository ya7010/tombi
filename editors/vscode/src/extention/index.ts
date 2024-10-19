import * as vscode from "vscode";
import * as node from "vscode-languageclient/node";
import { clientOptions } from "@/options/client-options";
import { serverOptions } from "@/options/server-options";
import { Server } from "@/lsp/server";
import type { Settings } from "./settings";
import { showServerVersion } from "@/command/show-server-version";
import { bootstrap } from "@/bootstrap";
export type { Settings };

export const EXTENTION_ID = "toml-toolkit";
export const EXTENTION_NAME = "TOML Toolkit";

export class Extension {
  static async activate(context: vscode.ExtensionContext): Promise<Extension> {
    const serverPath = await bootstrap(context, {});
    const server = new Server(serverPath);
    await showServerVersion(server);

    const extension = new Extension(context, server);

    return extension;
  }
  constructor(
    private context: vscode.ExtensionContext,
    private server: Server,
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
            settings: EXTENTION_ID,
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
        EXTENTION_NAME,
        serverOptions(this.context, this.server.binPath),
        clientOptions(workspaceFolder),
      );

      this.client.start();
    }
  }
}
