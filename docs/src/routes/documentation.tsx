import type { RouteSectionProps } from "@solidjs/router";
import { useLocation, A } from "@solidjs/router";
import { Sidebar } from "~/components/Sidebar";
import docIndex from "../../doc-index.json";
import { flattenDocPages, type FlattenedDocPage } from "~/utils/doc-index";

export default function DocumentationLayout(props: RouteSectionProps) {
  return (
    <div class="flex w-full h-full">
      <Sidebar />
      <main class="flex-1 p-4">
        {props.children}
        <DocNavigation />
      </main>
    </div>
  );
}

function DocNavigation() {
  const location = useLocation();
  const flatPages = flattenDocPages(docIndex);
  const currentIndex = flatPages.findIndex(
    (page) => `${import.meta.env.BASE_URL}${page.path}` === location.pathname,
  );
  let nextPage: FlattenedDocPage | null = null;
  let prevPage: FlattenedDocPage | null = null;
  if (currentIndex === -1) {
    nextPage = null;
    prevPage = null;
  } else {
    prevPage = currentIndex > 0 ? flatPages[currentIndex - 1] : null;
    nextPage =
      currentIndex < flatPages.length - 1 ? flatPages[currentIndex + 1] : null;
  }

  return (
    <div class="mt-8 pt-8 border-t border-gray-200 flex justify-between">
      {prevPage && (
        <A
          href={prevPage.path}
          class="no-underline text-blue-500 hover:text-blue-600"
        >
          ← {prevPage.title}
        </A>
      )}
      {nextPage && (
        <A
          href={nextPage.path}
          class="no-underline text-blue-500 hover:text-blue-600 ml-auto"
        >
          {nextPage.title} →
        </A>
      )}
    </div>
  );
}
