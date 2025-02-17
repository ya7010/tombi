import { For } from "solid-js";
import { A } from "@solidjs/router";
import docIndex from "../../doc-index.json";
import type { DicIndex } from "~/utils/doc-index";

const docIndexs: DicIndex[] = docIndex;

const TreeItem = (props: { item: DicIndex; level: number }) => {
  return (
    <div class={`my-2 pl-${props.level}`}>
      <A
        href={props.item.path}
        class="text-[--color-text] no-underline text-sm block py-1 hover:text-[--color-primary]"
      >
        {props.item.title}
      </A>
      {props.item.children && (
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
