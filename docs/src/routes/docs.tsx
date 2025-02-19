import type { RouteSectionProps } from "@solidjs/router";
import { useLocation } from "@solidjs/router";
import { Sidebar } from "~/components/Sidebar";
import { createEffect } from "solid-js";
import Prism from "prismjs";
import { DocNavigation } from "~/components/DocNavigation";

export default function DocumentationLayout(props: RouteSectionProps) {
  const location = useLocation();

  createEffect(() => {
    // Run whenever location changes
    location.pathname;
    // Apply highlighting in the next frame
    requestAnimationFrame(() => {
      Prism.highlightAll();
    });
  });

  return (
    <div class="flex w-full h-full">
      <Sidebar />
      <main class="flex-1 p-4 mdx-content">
        {props.children}
        <DocNavigation />
      </main>
    </div>
  );
}
