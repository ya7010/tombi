import { A } from "@solidjs/router";
import { createSignal, Show, For } from "solid-js";
import type { ParentProps } from "solid-js";
import { IoChevronDown } from "solid-icons/io";
import type { DicIndex } from "~/utils/doc-index";

interface HeaderDropdownItemProps {
  href: string;
  hasBorder?: boolean;
  onSelect: () => void;
  childrenItems?: DicIndex[];
  level?: number;
}

export function HeaderDropdownItem(
  props: ParentProps<HeaderDropdownItemProps>,
) {
  const [isExpanded, setIsExpanded] = createSignal(false);
  const level = props.level ?? 0;
  const hasChildren = !!props.childrenItems?.length;

  const handleClick = (e: MouseEvent) => {
    if (hasChildren && (e.target as HTMLElement).closest("button")) {
      e.preventDefault();
      return;
    }
    props.onSelect();
  };

  const toggleExpanded = (e: MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsExpanded(!isExpanded());
  };

  return (
    <div
      class={`flex flex-col ${props.hasBorder ? "border-b border-white/10" : ""}`}
    >
      <A
        href={props.href}
        class={`flex items-center px-2 py-1 text-white ${level === 0 ? "text-lg" : "text-base"} font-medium no-underline hover:bg-white/5 active:bg-white/10 transition-colors`}
        onClick={handleClick}
      >
        <span class="flex-grow" style={{ "padding-left": `${level}rem` }}>
          {props.children}
        </span>
        {hasChildren && (
          <button
            type="button"
            onClick={toggleExpanded}
            class="w-5 h-5 flex items-center justify-center text-white/60 hover:text-white focus:outline-none bg-transparent border-none transition-transform duration-300 ease-in-out"
            aria-expanded={isExpanded()}
            aria-label={isExpanded() ? "Collapse section" : "Expand section"}
          >
            <IoChevronDown
              classList={{ "rotate-180": isExpanded() }}
              size={16}
            />
          </button>
        )}
      </A>
      <Show when={hasChildren && isExpanded()}>
        <div class="flex flex-col pl-4 space-y-1">
          <For each={props.childrenItems}>
            {(child) => (
              <HeaderDropdownItem
                href={child.path}
                onSelect={props.onSelect}
                childrenItems={child.children}
                level={level + 1}
                hasBorder={false}
              >
                {child.title}
              </HeaderDropdownItem>
            )}
          </For>
        </div>
      </Show>
    </div>
  );
}
