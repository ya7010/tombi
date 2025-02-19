import { visit } from "unist-util-visit";
import type { Link, Root, Image } from "mdast";

export function remarkBaseUrl() {
  return (tree: Root) => {
    const processUrl = (url: string): string => {
      if (url.startsWith("/")) {
        let baseUrl = process.env.BASE_URL || import.meta.env.BASE_URL || "/";
        if (baseUrl === "/") {
          baseUrl = "/_build/";
        }
        return `${baseUrl}${url.slice(1)}`;
      }
      return url;
    };

    // Process URLs in links
    visit(tree, "link", (node: Link) => {
      node.url = processUrl(node.url);
    });

    // Process URLs in images
    visit(tree, "image", (node: Image) => {
      node.url = processUrl(node.url);
    });
  };
}
