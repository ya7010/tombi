import { defineConfig } from "@solidjs/start/config";
/* @ts-ignore */
import pkg from "@vinxi/plugin-mdx";
import unocssPlugin from "unocss/vite";

const { default: mdx } = pkg;

if (!process.env.BASE_URL) {
  process.env.BASE_URL = "/tombi/";
}

export default defineConfig({
  extensions: ["mdx", "md"],
  ssr: true,
  server: {
    preset: "static",
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
      }),
      unocssPlugin(),
    ],
    build: {
      minify: true,
    },
  },
});
