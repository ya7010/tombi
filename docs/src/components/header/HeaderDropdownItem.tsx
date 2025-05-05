import { A } from "@solidjs/router";
import { createSignal, For, createEffect } from "solid-js";
import type { ParentProps } from "solid-js";
import { IoChevronForward } from "solid-icons/io";
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
  const toggleExpanded = (e: MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsExpanded(!isExpanded());
  };

  const level = props.level ?? 0;
  const hasChildren = !!props.childrenItems?.length;

  const handleClick = (e: MouseEvent) => {
    if (hasChildren && (e.target as HTMLElement).closest("button")) {
      e.preventDefault();
      return;
    }
    props.onSelect();
  };

  // Manage max-height for smooth collapse/expand animation
  const [contentEl, setContentEl] = createSignal<HTMLDivElement>();
  createEffect(() => {
    const el = contentEl();
    if (!el) return;
    el.style.maxHeight = isExpanded() ? `${el.scrollHeight}px` : "0px";
  });

  return (
    <div
      class={`flex flex-col ${props.hasBorder ? "border-b border-white/10" : ""}`}
    >
      <A
        href={props.href}
        class={`flex items-center pl-3 h-16 text-white ${level === 0 ? "text-lg" : "text-base"} font-medium no-underline hover:bg-white/5 active:bg-white/10 transition-colors`}
        onClick={handleClick}
      >
        <span class="flex-grow" style={{ "padding-left": `${level}rem` }}>
          {props.children}
        </span>
        {hasChildren && (
          <button
            type="button"
            onClick={toggleExpanded}
            class="w-8 h-16 p-x-8 flex items-center justify-center text-white/60 hover:text-white hover:bg-white/10 focus:outline-none bg-transparent border-none rounded"
            aria-expanded={isExpanded()}
            aria-label={isExpanded() ? "Collapse section" : "Expand section"}
          >
            <IoChevronForward
              class="transition-transform duration-500 ease-in-out"
              classList={{ "rotate-90": isExpanded() }}
              size={16}
            />
          </button>
        )}
      </A>
      {hasChildren && (
        <div
          ref={setContentEl}
          class="flex flex-col pl-4 space-y-1 overflow-hidden transition-[max-height] duration-500 ease-in-out"
        >
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
      )}
    </div>
  );
}
