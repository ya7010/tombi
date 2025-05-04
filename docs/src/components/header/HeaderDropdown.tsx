import type { Accessor } from "solid-js";
import { For } from "solid-js";
import { HeaderDropdownItem } from "./HeaderDropdownItem";
import docIndex from "../../../doc-index.json";

interface HeaderDropdownProps {
  isOpen: Accessor<boolean>;
  onSelect: () => void;
}

export function HeaderDropdown(props: HeaderDropdownProps) {
  const menuItems = [
    { href: "/", label: "Home", childrenItems: undefined },
    {
      href: "/docs",
      label: "Docs",
      childrenItems: docIndex,
    },
    { href: "/playground", label: "Playground", childrenItems: undefined },
  ];

  return (
    <div
      class={`fixed inset-x-0 top-20 bottom-0 bg-tombi-primary shadow-lg z-40 md:hidden ${
        props.isOpen() ? "block" : "hidden"
      }`}
    >
      <div class="h-full overflow-y-auto p-4 flex flex-col">
        <For each={menuItems}>
          {(item, index) => (
            <HeaderDropdownItem
              href={item.href}
              hasBorder={index() < menuItems.length - 1}
              onSelect={props.onSelect}
              childrenItems={item.childrenItems}
              level={0}
            >
              {item.label}
            </HeaderDropdownItem>
          )}
        </For>
      </div>
    </div>
  );
}
