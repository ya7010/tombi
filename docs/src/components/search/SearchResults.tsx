import { For, Show } from "solid-js";
import type { SearchResult } from "~/utils/search";

interface SearchResultsProps {
  results: SearchResult[];
  isVisible: boolean;
}

export function SearchResults(props: SearchResultsProps) {
  return (
    <Show when={props.isVisible && props.results.length > 0}>
      <div class="fixed left-0 right-0 mt-2 mx-auto max-w-150 bg-white/95 dark:bg-gray-800/95 rounded-lg shadow-lg max-h-[80vh] overflow-y-auto z-[60]" style={{ top: "calc(5rem + 4px)" }}>
        <div class="p-4">
          <For each={props.results}>
            {(result) => (
              <a
                href={result.url}
                class="block p-4 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg mb-2 transition-colors"
              >
                <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-1">
                  {result.title}
                </h3>
                <p class="text-gray-600 dark:text-gray-300">{result.content}</p>
              </a>
            )}
          </For>
        </div>
      </div>
    </Show>
  );
}
