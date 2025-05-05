import type { Accessor } from "solid-js";
import { For } from "solid-js";
import { HeaderDropdownItem } from "./HeaderDropdownItem";
import type { DicIndex } from "~/utils/doc-index";
import docIndex from "../../../doc-index.json";

interface HeaderDropdownProps {
  isOpen: Accessor<boolean>;
  onSelect: () => void;
}

const menuItems: DicIndex[] = [
  { title: "Home", path: "/" },
  { title: "Docs", path: "/docs", children: docIndex },
  { title: "Playground", path: "/playground" },
];

export function HeaderDropdown(props: HeaderDropdownProps) {
  return (
    <div
      class="fixed inset-x-0 top-20 bg-tombi-primary shadow-lg z-40 md:hidden overflow-hidden transition-all duration-500 ease-linear"
      classList={{
        "max-h-0": !props.isOpen(),
        "max-h-[calc(100vh-5rem)]": props.isOpen(),
      }}
    >
      <nav class="flex flex-col px-4 pb-2 max-h-[calc(100vh-5rem)] overflow-y-auto">
        <For each={menuItems}>
          {(item, idx) => (
            <HeaderDropdownItem
              href={item.path}
              onSelect={props.onSelect}
              childrenItems={item.children}
              level={0}
              hasBorder={idx() < menuItems.length - 1}
            >
              {item.title}
            </HeaderDropdownItem>
          )}
        </For>
      </nav>
    </div>
  );
}
