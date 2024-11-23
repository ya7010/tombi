import * as vscode from "vscode";
import * as os from "node:os";
import type * as extention from "./extention";
import { log } from "@/logging";
import { LANGUAGE_SERVER_BIN_NAME } from "./lsp/server";

export type Env = {
  [name: string]: string;
};

export type TombiBin = {
  source: "bundled" | "local" | "debug" | "settings";
  path: string;
};

export async function bootstrap(
  context: vscode.ExtensionContext,
  settings: extention.Settings,
): Promise<TombiBin> {
  const tombiBin = await getTombiBin(context, settings);
  if (!tombiBin) {
    throw new Error("tombi Language Server is not available.");
  }

  log.info("Using Language Server binary at", tombiBin.path);

  return tombiBin;
}

export async function getTombiBin(
  context: vscode.ExtensionContext,
  settings: extention.Settings,
): Promise<TombiBin | undefined> {
  const packageJson: {
    releaseTag: string | null;
  } = context.extension.packageJSON;

  let settingsPath = settings.tombi?.path;
  if (settingsPath) {
    if (settingsPath.startsWith("~/")) {
      settingsPath = os.homedir() + settingsPath.slice("~".length);
    }
    return {
      source: "settings",
      path: settingsPath,
    };
  }

  if (packageJson.releaseTag === null) {
    return {
      source: "local",
      path: LANGUAGE_SERVER_BIN_NAME,
    };
  }

  // biome-ignore lint/complexity/useLiteralKeys: <explanation>
  const debugPath = process.env["__TOMBI_LANGUAGE_SERVER_DEBUG"];
  if (debugPath) {
    return {
      source: "debug",
      path: debugPath,
    };
  }

  // finally, use the bundled one
  const ext = process.platform === "win32" ? ".exe" : "";
  const bundledUri = vscode.Uri.joinPath(
    context.extensionUri,
    "server",
    LANGUAGE_SERVER_BIN_NAME + ext,
  );

  if (await fileExists(bundledUri)) {
    return {
      source: "bundled",
      path: bundledUri.fsPath,
    };
  }

  await vscode.window.showErrorMessage(
    "Unfortunately we don't ship binaries for your platform yet. ",
  );

  return undefined;
}

async function fileExists(uri: vscode.Uri) {
  return await vscode.workspace.fs.stat(uri).then(
    () => true,
    () => false,
  );
}
