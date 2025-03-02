import type { APIEvent } from "@solidjs/start/server";
import fs from "node:fs/promises";
import path from "node:path";

export async function GET(event: APIEvent) {
  const scriptPath = path.join(import.meta.dirname, "install.sh");
  const script = await fs.readFile(scriptPath, "utf-8");

  event.response.headers.set("Content-Type", "application/x-sh");
  return new Response(script);
}
