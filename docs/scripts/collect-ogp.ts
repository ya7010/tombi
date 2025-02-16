import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { type OGPData, OgpUrlMap } from "../src/utils/ogp";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export async function fetchOGP(url: string): Promise<OGPData | null> {
  try {
    const response = await fetch(
      `https://api.allorigins.win/get?url=${encodeURIComponent(url)}`,
    );
    const { contents } = await response.json();

    // Use JSDOM in Node.js environment
    let doc: Document;
    if (typeof window === "undefined") {
      const { JSDOM } = await import("jsdom");
      const dom = new JSDOM(contents);
      doc = dom.window.document;
    } else {
      const parser = new DOMParser();
      doc = parser.parseFromString(contents, "text/html");
    }

    const getMetaContent = (property: string) => {
      const meta = doc.querySelector(`meta[property="${property}"]`);
      return meta?.getAttribute("content") || "";
    };

    return {
      title: getMetaContent("og:title") || doc.title || "",
      description: getMetaContent("og:description") || "",
      image: getMetaContent("og:image") || "",
      url: url,
    };
  } catch (error) {
    console.error("Failed to fetch OGP:", error);
    return null;
  }
}

async function collectOGP() {
  const ogpData: Record<string, OGPData> = {};

  for (const [id, url] of Object.entries(OgpUrlMap)) {
    try {
      const data = await fetchOGP(url);
      if (data && (data.title || data.description || data.image)) {
        ogpData[id] = data;
      }
    } catch (error) {
      console.error(`Failed to fetch OGP for ${id} (${url}):`, error);
    }
  }

  // データディレクトリが存在することを確認
  const dataDir = path.resolve(__dirname, "../src/data");
  if (!fs.existsSync(dataDir)) {
    fs.mkdirSync(dataDir, { recursive: true });
  }

  // OGPデータを JSON ファイルとして保存
  fs.writeFileSync(
    path.resolve(dataDir, "ogp.json"),
    JSON.stringify(ogpData, null, 2),
  );
}

collectOGP().catch(console.error);
