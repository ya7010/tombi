import * as vscode from "vscode";
import * as node from "vscode-languageclient/node";
import { clientOptions } from "@/options/client-options";
import { serverOptions } from "@/options/server-options";
import { Server } from "@/lsp/server";
import type { Settings } from "./settings";
import * as command from "@/command";
import { bootstrap } from "@/bootstrap";
import { log } from "@/logging";
export type { Settings };

export const EXTENTION_ID = "tombi";
export const EXTENTION_NAME = "Tombi";
export const SUPPORT_LANGUAGES = ["toml", "cargoLock"];

export class Extension {
  constructor(
    private context: vscode.ExtensionContext,
    private client: node.LanguageClient,
    private server: Server,
  ) {
    this.registerEvents();
    this.registerCommands();
  }

  static async activate(context: vscode.ExtensionContext): Promise<Extension> {
    const serverPath = await bootstrap(context, {});
    const server = new Server(serverPath);
    const client = new node.LanguageClient(
      EXTENTION_ID,
      `${EXTENTION_NAME} LSP`,
      serverOptions(server.binPath),
      clientOptions(),
      // biome-ignore lint/complexity/useLiteralKeys: <explanation>
      process.env["__TOMBI_LSP_SERVER_DEBUG"] !== undefined,
    );

    const extenstion = new Extension(context, client, server);
    log.info("extension started");

    return extenstion;
  }

  async deactivate(): Promise<void> {
    await this.client?.stop();
  }

  private registerCommands(): void {
    this.context.subscriptions.push(
      vscode.commands.registerCommand(
        `${EXTENTION_ID}.showServerVersion`,
        async () => {
          await command.showServerVersion(this.server);
        },
      ),
    );
  }

  private registerEvents(): void {
    vscode.workspace.onDidChangeTextDocument(
      async (event) => await this.onDidChangeTextDocument(event),
    );
    vscode.workspace.onDidSaveTextDocument(
      async (event) => await this.onDidSaveTextDocument(event),
    );
    vscode.workspace.onDidChangeConfiguration(
      async (event) => await this.onDidChangeConfiguration(event),
      null,
      this.context.subscriptions,
    );
  }

  private async onDidChangeTextDocument({
    document,
  }: vscode.TextDocumentChangeEvent): Promise<void> {
    if (!SUPPORT_LANGUAGES.includes(document.languageId)) {
      return;
    }
  }

  private async onDidChangeConfiguration(
    _: vscode.ConfigurationChangeEvent,
  ): Promise<void> {
    this.client?.sendNotification(
      node.DidChangeConfigurationNotification.type,
      {
        settings: EXTENTION_ID,
      },
    );
  }

  private async onDidSaveTextDocument(
    document: vscode.TextDocument,
  ): Promise<void> {
    log.info("onDidSaveTextDocument", document.languageId);
    if (SUPPORT_LANGUAGES.includes(document.languageId)) {
    }
  }
}
