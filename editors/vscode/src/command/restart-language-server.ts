import type * as node from "vscode-languageclient/node";

export async function restartLanguageServer(
  client: node.LanguageClient,
): Promise<void> {
  await client.restart();
}
