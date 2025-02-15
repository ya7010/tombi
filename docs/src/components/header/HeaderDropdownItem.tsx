import { A } from "@solidjs/router";
import type { ParentProps } from "solid-js";

interface HeaderDropdownItemProps {
  href: string;
  hasBorder?: boolean;
  onSelect: () => void;
}

export function HeaderDropdownItem(props: ParentProps<HeaderDropdownItemProps>) {
  return (
    <A
      href={props.href}
      class={`flex items-center p-x4 p-y2 m-2 h-8 text-white text-lg font-medium no-underline
              hover:bg-white/5 active:bg-white/10 transition-colors
              ${props.hasBorder ? 'border-b border-white/10' : ''}`}
      onClick={props.onSelect}
    >
      {props.children}
    </A>
  );
}
