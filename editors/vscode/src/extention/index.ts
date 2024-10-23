import * as vscode from "vscode";
import * as node from "vscode-languageclient/node";
import { clientOptions } from "@/options/client-options";
import { serverOptions } from "@/options/server-options";
import { Server } from "@/lsp/server";
import type { Settings } from "./settings";
import * as command from "@/command";
import { bootstrap } from "@/bootstrap";
export type { Settings };

export const EXTENTION_ID = "tombi";
export const EXTENTION_NAME = "Tombi";

export class Extension {
  constructor(
    private context: vscode.ExtensionContext,
    private server: Server,
    private client?: node.LanguageClient,
  ) {
    this.registerCommands();
    for (const document of vscode.workspace.textDocuments) {
      this.onDidOpentextDocument({ document });
    }

    vscode.workspace.onDidChangeTextDocument((event) =>
      this.onDidOpentextDocument(event),
    );

    vscode.workspace.onDidChangeConfiguration(
      async (_event) => {
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

  static async activate(context: vscode.ExtensionContext): Promise<Extension> {
    const serverPath = await bootstrap(context, {});
    const server = new Server(serverPath);

    return new Extension(context, server);
  }

  deactivate(): Thenable<void> | undefined {
    return this.client?.stop();
  }

  private registerCommands() {
    this.context.subscriptions.push(
      vscode.commands.registerCommand(
        `${EXTENTION_ID}.showServerVersion`,
        async () => {
          await command.showServerVersion(this.server);
        },
      ),
    );
  }

  private onDidOpentextDocument({
    document,
  }: { document: vscode.TextDocument }): void {
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
