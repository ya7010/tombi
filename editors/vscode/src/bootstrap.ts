import * as vscode from "vscode";
import * as os from "node:os";
import type * as extention from "./extention";
import { log } from "@/logging";
import { LANGUAGE_SERVER_BIN_NAME } from "./lsp/server";

export type Env = {
  [name: string]: string;
};

export type TombiBin = {
  source: "bundled" | "local";
  path: string;
};

export async function bootstrap(
  context: vscode.ExtensionContext,
  settings: extention.Settings,
): Promise<TombiBin> {
  const tombiBin = await getServerPath(context, settings);
  if (!tombiBin) {
    throw new Error("tombi Language Server is not available.");
  }

  log.info("Using Language Server binary at", tombiBin.path);

  return tombiBin;
}

export async function getServerPath(
  context: vscode.ExtensionContext,
  settings: extention.Settings,
): Promise<TombiBin | undefined> {
  const packageJson: {
    releaseTag: string | null;
  } = context.extension.packageJSON;

  let localPath =
    // biome-ignore lint/complexity/useLiteralKeys: <explanation>
    process.env["__TOMBI_LANGUAGE_SERVER_DEBUG"] ?? settings.tombi?.path;

  if (localPath) {
    if (localPath.startsWith("~/")) {
      localPath = os.homedir() + localPath.slice("~".length);
    }
    return {
      source: "local",
      path: localPath,
    };
  }

  if (packageJson.releaseTag === null)
    return {
      source: "local",
      path: LANGUAGE_SERVER_BIN_NAME,
    };

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
