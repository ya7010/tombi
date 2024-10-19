import type * as vscode from "vscode";
import { Extension } from "@/extention";

let extention: Extension;

export async function activate(
  context: vscode.ExtensionContext,
): Promise<void> {
  if (!extention) {
    extention = await Extension.activate(context);
  }
}

export async function deactivate(): Promise<void> {
  extention?.deactivate();
}
