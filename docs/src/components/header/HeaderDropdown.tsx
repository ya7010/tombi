import type { Accessor } from "solid-js";
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
      class={`fixed inset-x-0 top-20 bg-tombi-primary shadow-lg z-40 md:hidden
              overflow-hidden transition-[max-height] duration-300 ease-out`}
      style={{
        "max-height": props.isOpen() ? "100vh" : "0px",
        height: "auto",
      }}
    >
      <div
        class="flex flex-col overflow-y-auto"
        style={{
          "max-height": "calc(100vh - 5rem)",
          height: "auto",
        }}
      >
        {props.isOpen() &&
          menuItems.map((item, index) => (
            <HeaderDropdownItem
              // @ts-ignore
              key={item.href}
              href={item.href}
              hasBorder={index < menuItems.length - 1}
              onSelect={props.onSelect}
              childrenItems={item.childrenItems}
              level={0}
            >
              {item.label}
            </HeaderDropdownItem>
          ))}
      </div>
    </div>
  );
}
