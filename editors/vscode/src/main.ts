import type * as vscode from "vscode";
import { Extension } from "./extention";
import { bootstrap } from "./bootstrap";

let extention: Extension;

export async function activate(
  context: vscode.ExtensionContext,
): Promise<void> {
  const serverPath = await bootstrap(context, {});
  if (!extention) extention = new Extension(context, serverPath);
}

export async function deactivate(): Promise<void> {
  extention?.deactivate();
}
