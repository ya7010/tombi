import { For, createSignal, Show, createMemo } from "solid-js";
import { A, useLocation } from "@solidjs/router";
import { IoChevronDown } from "solid-icons/io";
import docIndex from "../../doc-index.json";
import type { DicIndex } from "~/utils/doc-index";

const docIndexs: DicIndex[] = docIndex;

const TreeItem = (props: { item: DicIndex; level: number }) => {
  const location = useLocation();
  const isCurrentPage = createMemo(
    () => location.pathname === `${import.meta.env.BASE_URL}${props.item.path}`,
  );

  const hasChildren = props.item.children && props.item.children.length > 0;
  const shouldBeExpanded = createMemo(
    () =>
      hasChildren &&
      (isCurrentPage() ||
        props.item.children?.some(
          (child) =>
            `${import.meta.env.BASE_URL}${child.path}` === location.pathname,
        )),
  );

  const [isExpanded, setIsExpanded] = createSignal(shouldBeExpanded());

  createMemo(() => {
    if (shouldBeExpanded()) {
      setIsExpanded(true);
    }
  });

  return (
    <div class={`my-2 pl-${props.level * 2}`}>
      <div class="flex items-center">
        {isCurrentPage() ? (
          <span class="font-bold text-tombi-700 dark:text-color-yellow text-sm block py-1 flex-grow">
            {props.item.title}
          </span>
        ) : (
          <A
            href={props.item.path}
            class="text-[--color-text] no-underline text-sm block py-1 hover:text-[--color-primary] flex-grow"
          >
            {props.item.title}
          </A>
        )}
        {hasChildren && (
          <button
            type="button"
            onClick={() => setIsExpanded(!isExpanded())}
            class="w-5 h-5 mr-3 flex items-center justify-center font-bold text-[--color-primary] hover:text-blue-600 focus:outline-none bg-transparent border-none"
          >
            <div
              class="transform transition-transform duration-300"
              classList={{ "rotate-180": isExpanded() }}
            >
              <IoChevronDown size={16} />
            </div>
          </button>
        )}
      </div>
      <div
        class="overflow-hidden transition-all duration-500 ease-in-out"
        style={{
          "max-height": isExpanded() ? "500px" : "0px",
          opacity: isExpanded() ? "1" : "0",
        }}
      >
        <div class="ml-2">
          <For each={props.item.children}>
            {(child) => <TreeItem item={child} level={props.level + 1} />}
          </For>
        </div>
      </div>
    </div>
  );
};

export function Sidebar() {
  return (
    <nav class="w-[200px] h-full p-4 bg-[--color-bg-secondary] border-r border-[--color-border] sm:block hidden">
      <For each={docIndexs}>{(item) => <TreeItem item={item} level={0} />}</For>
    </nav>
  );
}
