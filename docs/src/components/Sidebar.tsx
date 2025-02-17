import { For, createSignal } from "solid-js";
import { A } from "@solidjs/router";
import { IoChevronDown, IoChevronForward } from "solid-icons/io";
import docIndex from "../../doc-index.json";
import type { DicIndex } from "~/utils/doc-index";

const docIndexs: DicIndex[] = docIndex;

const TreeItem = (props: { item: DicIndex; level: number }) => {
  const [isExpanded, setIsExpanded] = createSignal(false);
  const hasChildren = props.item.children && props.item.children.length > 0;

  return (
    <div class={`my-2 pl-${props.level}`}>
      <div class="flex items-center">
        {hasChildren && (
          <button
            type="button"
            onClick={() => setIsExpanded(!isExpanded())}
            class="w-5 h-5 mr-1 flex items-center justify-center font-bold text-[--color-primary] hover:text-blue-600 focus:outline-none bg-transparent border-none"
          >
            {isExpanded() ? (
              <IoChevronDown size={16} />
            ) : (
              <IoChevronForward size={16} />
            )}
          </button>
        )}
        <A
          href={props.item.path}
          class="text-[--color-text] no-underline text-sm block py-1 hover:text-[--color-primary] flex-grow"
        >
          {props.item.title}
        </A>
      </div>
      {hasChildren && isExpanded() && (
        <div class="ml-2">
          <For each={props.item.children}>
            {(child) => <TreeItem item={child} level={props.level + 1} />}
          </For>
        </div>
      )}
    </div>
  );
};

export function Sidebar() {
  return (
    <nav class="w-[250px] h-full p-4 bg-[--color-bg-secondary] border-r border-[--color-border] sm:block hidden">
      <For each={docIndexs}>{(item) => <TreeItem item={item} level={0} />}</For>
    </nav>
  );
}
