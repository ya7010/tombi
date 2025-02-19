import type { RouteSectionProps } from "@solidjs/router";
import { useLocation, A } from "@solidjs/router";
import { Sidebar } from "~/components/Sidebar";
import docIndex from "../../doc-index.json";
import { flattenDocPages, type FlattenedDocPage } from "~/utils/doc-index";
import { createEffect } from "solid-js";
import Prism from "prismjs";

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

function DocNavigation() {
  const location = useLocation();
  const flatPages = flattenDocPages(docIndex);
  const currentIndex = () =>
    flatPages.findIndex(
      (page) => `${import.meta.env.BASE_URL}${page.path}` === location.pathname,
    );
  const nextPage = () =>
    currentIndex() === -1
      ? null
      : currentIndex() < flatPages.length - 1
        ? flatPages[currentIndex() + 1]
        : null;
  const prevPage = () =>
    currentIndex() === -1
      ? null
      : currentIndex() > 0
        ? flatPages[currentIndex() - 1]
        : null;

  return (
    <div class="mt-8 pt-8 border-t border-gray-200 flex justify-between">
      {(() => {
        const prev = prevPage();
        return (
          prev && (
            <A
              href={prev.path}
              class="no-underline text-blue-500 hover:text-blue-600"
            >
              ← {prev.title}
            </A>
          )
        );
      })()}
      {(() => {
        const next = nextPage();
        return (
          next && (
            <A
              href={next.path}
              class="no-underline text-blue-500 hover:text-blue-600 ml-auto"
            >
              {next.title} →
            </A>
          )
        );
      })()}
    </div>
  );
}
