import { defineConfig } from "@solidjs/start/config";
/* @ts-ignore */
import pkg from "@vinxi/plugin-mdx";
import unocssPlugin from "unocss/vite";

const { default: mdx } = pkg;
export default defineConfig({
  extensions: ["mdx", "md"],
  ssr: true,
  islands: true,
  server: {
    preset: "static",
    prerender: {
      crawlLinks: true,
      urlMap: {
        "/": { type: "page" },
        "/documentation": { type: "page" },
        "/documentation/getting-started/installation": { type: "page" },
        "/playground": { type: "page" },
        "/concepts": { type: "page" },
      },
    },
  },
  vite: {
    plugins: [
      mdx.withImports({})({
        jsx: true,
        jsxImportSource: "solid-js",
        providerImportSource: "solid-mdx",
      }),
      unocssPlugin(),
    ],
  },
});
