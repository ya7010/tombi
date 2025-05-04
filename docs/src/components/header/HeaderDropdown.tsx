import type { Accessor } from "solid-js";
import { Show, For } from "solid-js";
import { HeaderDropdownItem } from "./HeaderDropdownItem";
import type { DicIndex } from "~/utils/doc-index";
import docIndex from "../../../doc-index.json";

interface HeaderDropdownProps {
  isOpen: Accessor<boolean>;
  onSelect: () => void;
}

const menuItems: { href: string; label: string; childrenItems?: DicIndex[] }[] =
  [
    { href: "/", label: "Home" },
    { href: "/docs", label: "Docs", childrenItems: docIndex },
    { href: "/playground", label: "Playground" },
  ];

export function HeaderDropdown(props: HeaderDropdownProps) {
  return (
    <Show when={props.isOpen()}>
      <aside class="fixed inset-x-0 top-20 bottom-0 bg-tombi-primary shadow-lg z-40 md:hidden overflow-y-auto">
        <nav class="flex flex-col p-4 gap-y-2">
          <For each={menuItems}>
            {(item, idx) => (
              <HeaderDropdownItem
                href={item.href}
                onSelect={props.onSelect}
                childrenItems={item.childrenItems}
                level={0}
                hasBorder={idx() < menuItems.length - 1}
              >
                {item.label}
              </HeaderDropdownItem>
            )}
          </For>
        </nav>
      </aside>
    </Show>
  );
}
