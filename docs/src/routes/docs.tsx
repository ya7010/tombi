import type { RouteSectionProps } from "@solidjs/router";
import { useLocation } from "@solidjs/router";
import { Sidebar } from "~/components/Sidebar";
import { createEffect, onMount } from "solid-js";
import Prism from "prismjs";
import { DocNavigation } from "~/components/DocNavigation";
import { setupAnchors } from "~/utils/anchor";

export default function DocumentationLayout(props: RouteSectionProps) {
  const location = useLocation();

  createEffect(() => {
    location.pathname;
    requestAnimationFrame(() => {
      Prism.highlightAll();
      setupAnchors();
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
