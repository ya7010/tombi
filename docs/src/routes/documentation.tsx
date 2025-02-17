import type { RouteSectionProps } from "@solidjs/router";
import { Sidebar } from "~/components/Sidebar";

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
      </main>
    </div>
  );
}
