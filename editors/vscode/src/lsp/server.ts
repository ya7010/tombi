import { spawn } from "node:child_process";
import { text } from "node:stream/consumers";

export const LANGUAGE_SERVER_BIN_NAME = "toml-lsp";

export class Server {
  private version?: string;

  constructor(public binPath: string) {}

  async showVersion(): Promise<string> {
    if (this.version === undefined) {
      let version: string;
      try {
        version = await text(
          spawn(this.binPath, ["--version"]).stdout.setEncoding("utf-8"),
        );
        // version の先頭文字が LANGUAGE_SERVER_BIN_NAME で始まる場合は、その文字列を削除し、文字列の先頭の空白も削除する
        if (version.startsWith(LANGUAGE_SERVER_BIN_NAME)) {
          version = version.slice(LANGUAGE_SERVER_BIN_NAME.length).trimStart();
        }
      } catch {
        version = "<unknown>";
      }

      this.version = version;

      return version;
    }

    return this.version;
  }
}
