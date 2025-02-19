import { visit } from "unist-util-visit";
import type { Link, Root } from "mdast";

export function remarkBaseUrl() {
  return (tree: Root) => {
    visit(tree, "link", (node: Link) => {
      console.log(`before: ${node.url}`);
      if (node.url.startsWith("/")) {
        let baseUrl = process.env.BASE_URL || import.meta.env.BASE_URL || "/";
        if (baseUrl === "/") {
          baseUrl = "/_build/";
        }
        node.url = `${baseUrl}${node.url.slice(1)}`;
      }
      console.log(`after: ${node.url}`);
    });
  };
}
