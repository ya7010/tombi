import type { Accessor } from "solid-js";
import { For } from "solid-js";
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
    <div
      class="fixed inset-x-0 top-20 bg-tombi-primary shadow-lg z-40 md:hidden overflow-hidden transition-[max-height] duration-500"
      classList={{
        "max-h-0": !props.isOpen(),
        "max-h-[calc(100vh-5rem)]": props.isOpen(),
      }}
    >
      <nav class="flex flex-col px-4 pb-2 max-h-[calc(100vh-5rem)] overflow-y-auto">
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
  );
}
