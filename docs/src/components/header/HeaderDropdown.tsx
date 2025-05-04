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
      <div class="fixed inset-x-0 top-20 bg-tombi-primary shadow-lg z-40 md:hidden max-h-[calc(100vh-5rem)] overflow-y-auto">
        <nav class="flex flex-col p-x-4 p-b-6">
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
      </div>
    </Show>
  );
}
