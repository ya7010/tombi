import type { RouteSectionProps } from "@solidjs/router";
import { useLocation } from "@solidjs/router";
import { Sidebar } from "~/components/Sidebar";
import { createEffect, onMount } from "solid-js";
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

  onMount(() => {
    // Add anchor links to all headings
    for (const heading of document.querySelectorAll(".heading-with-anchor")) {
      const anchorLink = document.createElement("a");
      anchorLink.textContent = "#";
      anchorLink.className = "anchor-link";
      anchorLink.setAttribute("data-anchor", "true");
      anchorLink.setAttribute("aria-hidden", "true");
      heading.appendChild(anchorLink);
    }

    document.addEventListener("click", (event) => {
      const target = event.target as HTMLElement;

      // Process anchor link clicks
      if (target.closest(".anchor-link")) {
        const heading = target.closest(".heading-with-anchor") as HTMLElement;
        if (heading?.id) {
          event.preventDefault(); // Prevent default behavior

          // Copy URL to clipboard
          const url = `${window.location.href.split("#")[0]}#${heading.id}`;
          navigator.clipboard.writeText(url).then(() => {
            // Display copy notification
            const notification = document.createElement("div");
            notification.textContent = "URL copied to clipboard";
            notification.style.position = "fixed";
            notification.style.bottom = "20px";
            notification.style.right = "20px";
            notification.style.padding = "10px 15px";
            notification.style.backgroundColor = "#333";
            notification.style.color = "#fff";
            notification.style.borderRadius = "4px";
            notification.style.zIndex = "1000";
            document.body.appendChild(notification);

            // Remove notification after 2 seconds
            setTimeout(() => {
              document.body.removeChild(notification);
            }, 2000);
          });
        }
      }
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
