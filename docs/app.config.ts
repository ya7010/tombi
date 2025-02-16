import { defineConfig } from "@solidjs/start/config";
/* @ts-ignore */
import pkg from "@vinxi/plugin-mdx";
import unocssPlugin from "unocss/vite";

const { default: mdx } = pkg;
export default defineConfig({
  extensions: ["mdx", "md"],
  ssr: true,
  server: {
    preset: "static",
    prerender: {
      crawlLinks: true,
      failOnError: true,
      concurrency: 1,
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
