#!/usr/bin/env node

import { Command } from "commander";
import * as fs from "node:fs";
import * as path from "node:path";

// Get version from package.json
const packageJsonPath = path.join(__dirname, "..", "package.json");
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
const version = packageJson.version;

const program = new Command();

program
  .name("tombi")
  .description("A feature-rich TOML toolkit")
  .version(version);

program
  .command("format")
  .description("Format TOML files")
  .argument("<files...>", "Files to format")
  .option("-i, --in-place", "Edit files in place")
  .action((files: string[], options: { inPlace?: boolean }) => {
    // Future implementation will integrate with tombi core functionality
    console.log("Format command called with files:", files);
    console.log("Options:", options);
  });

program
  .command("lint")
  .description("Lint TOML files")
  .argument("<files...>", "Files to lint")
  .option("--fix", "Automatically fix problems when possible")
  .action((files: string[], options: { fix?: boolean }) => {
    // Future implementation will integrate with tombi core functionality
    console.log("Lint command called with files:", files);
    console.log("Options:", options);
  });

program.parse(process.argv);
