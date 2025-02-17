import { For } from "solid-js";
import { A } from "@solidjs/router";
import docIndex from "../../doc-index.json";

type MenuItem = {
  title: string;
  path: string;
  children?: MenuItem[];
};

const menuItems: MenuItem[] = docIndex.map((item) => ({
  title: item.title,
  path: item.path,
  children: item.children
    ? item.children.map((child) => ({
        title: child.title,
        path: child.path,
      }))
    : undefined,
}));

const TreeItem = (props: { item: MenuItem; level: number }) => {
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
      <For each={menuItems}>{(item) => <TreeItem item={item} level={0} />}</For>
    </nav>
  );
}
