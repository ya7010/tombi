import type { RouteSectionProps } from "@solidjs/router";
import { useLocation, A } from "@solidjs/router";
import { Sidebar } from "~/components/Sidebar";
import docIndex from "../../doc-index.json";
import { flattenDocPages, type FlattenedDocPage } from "~/utils/doc-index";

export default function DocumentationLayout(props: RouteSectionProps) {
  return (
    <div
      style={{
        display: "flex",
        width: "100%",
        height: "100%",
      }}
    >
      <Sidebar />
      <main
        style={{
          flex: 1,
          padding: "1rem",
        }}
      >
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
    <div
      style={{
        "margin-top": "2rem",
        "padding-top": "2rem",
        "border-top": "1px solid #eaeaea",
        display: "flex",
        "justify-content": "space-between",
      }}
    >
      {prevPage && (
        <A
          href={prevPage.path}
          style={{
            "text-decoration": "none",
            color: "#0070f3",
          }}
        >
          ← {prevPage.title}
        </A>
      )}
      {nextPage && (
        <A
          href={nextPage.path}
          style={{
            "text-decoration": "none",
            color: "#0070f3",
            "margin-left": "auto",
          }}
        >
          {nextPage.title} →
        </A>
      )}
    </div>
  );
}
