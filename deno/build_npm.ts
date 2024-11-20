// ex. scripts/build_npm.ts
import { build, emptyDir } from "@deno/dnt";

await emptyDir("./npm");

await build({
  entryPoints: ["./mod.ts"],
  outDir: "./npm",
  shims: {
    // see JS docs for overview and more options
    deno: true,
  },
  importMap: "./deno.json",
  typeCheck: false,
  package: {
    name: "tombi",
    version: Deno.args[0],
    description: "Reserved package for tombi.",
    license: "MIT",
    repository: {
      type: "git",
      url: "git+https://github.com/yassun7010/tombi.git",
    },
    bugs: {
      url: "https://github.com/yassun7010/tombi/issues",
    },
  },
  postBuild() {
    // steps to run after building and before running the tests
    Deno.copyFileSync("README.md", "npm/README.md");
  },
});
