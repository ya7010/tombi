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
    const settings = vscode.workspace.getConfiguration(
      EXTENTION_ID,
    ) as Settings;

    const tombiBin = await bootstrap(context, settings);

    const server = new Server(tombiBin);
    const client = new node.LanguageClient(
      EXTENTION_ID,
      `${EXTENTION_NAME} Language Server`,
      serverOptions(server.tombiBin.path, settings),
      clientOptions(),
      // biome-ignore lint/complexity/useLiteralKeys: <explanation>
      process.env["__TOMBI_LANGUAGE_SERVER_DEBUG"] !== undefined,
    );

    const extenstion = new Extension(context, client, server);

    // NOTE: When VSCode starts, if a TOML document is open in a tab and the focus is not on it,
    //       the Language Server will not start.
    //       Therefore, send the notification to the Language Server for all open TOML documents.
    for (const document of vscode.workspace.textDocuments) {
      await extenstion.onDidOpenTextDocument(document);
    }

    log.info("extension activated");

    return extenstion;
  }

  async deactivate(): Promise<void> {
    await this.client.stop();
    log.info("extension deactivated");
  }

  private registerCommands(): void {
    this.context.subscriptions.push(
      vscode.commands.registerCommand(
        `${EXTENTION_ID}.showLanguageServerVersion`,
        async () => command.showLanguageServerVersion(this.server),
      ),
    );
  }

  private registerEvents(): void {
    vscode.workspace.onDidOpenTextDocument(async (event) =>
      this.onDidOpenTextDocument(event),
    );
  }

  private async onDidOpenTextDocument(
    document: vscode.TextDocument,
  ): Promise<void> {
    if (SUPPORT_LANGUAGES.includes(document.languageId)) {
      await this.client.sendNotification(
        node.DidOpenTextDocumentNotification.type,
        {
          textDocument: node.TextDocumentItem.create(
            document.uri.toString(),
            document.languageId,
            document.version,
            document.getText(),
          ),
        },
      );
    }
  }
}
