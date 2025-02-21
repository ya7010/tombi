export interface OGPData {
  title: string;
  description: string;
  image: string;
  url: string;
}

export const OgpUrlMap = {
  vscode:
    "https://marketplace.visualstudio.com/items?itemName=tombi-toml.tombi",
} as const;

export type OgpId = keyof typeof OgpUrlMap;
