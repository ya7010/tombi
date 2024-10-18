import type * as vscode from "vscode";
import { Extension } from "./extention";

let extention: Extension;

export function activate(context: vscode.ExtensionContext): void {
  if (!extention) extention = new Extension(context);
}

export function deactivate(): Thenable<void> | undefined {
  return extention?.deactivate();
}
