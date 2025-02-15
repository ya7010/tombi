import type { Accessor } from "solid-js";
import { HeaderDropdownItem } from "./HeaderDropdownItem";

interface HeaderDropdownProps {
  isOpen: Accessor<boolean>;
  onSelect: () => void;
}

export function HeaderDropdown(props: HeaderDropdownProps) {
  const menuItems = [
    { href: "/", label: "Home" },
    { href: "/documentation/concepts", label: "Concepts" },
    { href: "/documentation", label: "Docs" },
    { href: "/playground", label: "Playground" },
  ];

  // h-16 = 64px
  const itemHeight = 64;
  const totalHeight = menuItems.length * itemHeight + 10;

  return (
    <div
      class={`fixed inset-x-0 top-20 bg-tombi-primary shadow-lg z-40 md:hidden
              overflow-hidden transition-[height] duration-300 ease-out`}
      style={{ height: props.isOpen() ? `${totalHeight}px` : "0px" }}
    >
      <div class="flex flex-col">
        {props.isOpen() &&
          menuItems.map((item, index) => (
            <HeaderDropdownItem
              // @ts-ignore
              key={item.href}
              href={item.href}
              hasBorder={index < menuItems.length - 1}
              onSelect={props.onSelect}
            >
              {item.label}
            </HeaderDropdownItem>
          ))}
      </div>
    </div>
  );
}
