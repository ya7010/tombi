import { defineConfig } from "@solidjs/start/config";
/* @ts-ignore */
import pkg from "@vinxi/plugin-mdx";
import unocssPlugin from "unocss/vite";
import remarkGfm from "remark-gfm";

const { default: mdx } = pkg;

export default defineConfig({
  extensions: ["mdx", "md"],
  ssr: true,
  server: {
    preset: "static",
    baseURL: process.env.BASE_URL,
    prerender: {
      crawlLinks: true,
      failOnError: true,
    },
  },
  vite: {
    // @ts-ignore
    base: process.env.BASE_URL,
    plugins: [
      mdx.withImports({})({
        jsx: true,
        jsxImportSource: "solid-js",
        providerImportSource: "solid-mdx",
        remarkPlugins: [[remarkGfm, { tablePipeAlign: false }]],
      }),
      unocssPlugin(),
    ],
    build: {
      minify: true,
    },
  },
});
