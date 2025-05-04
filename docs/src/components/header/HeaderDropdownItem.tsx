import { A } from "@solidjs/router";
import type { ParentProps } from "solid-js";
import { IoChevronDown } from "solid-icons/io";
import { createSignal, For, onMount, createEffect } from "solid-js";
import docIndex from "../../../doc-index.json";
import type { DicIndex } from "~/utils/doc-index";

interface HeaderDropdownItemProps {
  href: string;
  hasBorder?: boolean;
  onSelect: () => void;
  childrenItems?: DicIndex[];
  level?: number;
  onHeightChange?: (height: number) => void;
  maxHeight?: number;
}

const findItemByPath = (path: string): DicIndex | undefined => {
  const findInItems = (items: DicIndex[]): DicIndex | undefined => {
    for (const item of items) {
      if (item.path === path) return item;
      if (item.children) {
        const found = findInItems(item.children);
        if (found) return found;
      }
    }
    return undefined;
  };
  return findInItems(docIndex);
};

export function HeaderDropdownItem(
  props: ParentProps<HeaderDropdownItemProps>,
) {
  const [isExpanded, setIsExpanded] = createSignal(false);
  const [contentHeight, setContentHeight] = createSignal(0);
  const item = findItemByPath(props.href);
  const hasChildren = props.childrenItems && props.childrenItems.length > 0;
  const level = props.level ?? 0;
  let contentRef: HTMLDivElement | undefined;

  const calculateHeight = () => {
    if (!contentRef) return;
    const height = contentRef.scrollHeight;
    setContentHeight(height);
    props.onHeightChange?.(height);
  };

  createEffect(() => {
    if (isExpanded()) {
      calculateHeight();
    }
  });

  const handleParentClick = (e: MouseEvent) => {
    // If dropdown button is clicked, do not navigate
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
    <div>
      <A
        href={props.href}
        class={`flex items-center p-x4 p-y2 m-2 h-8 text-white ${level === 0 ? "text-lg" : "text-base"} font-medium no-underline
                hover:bg-white/5 active:bg-white/10 transition-colors btn-focus
                ${props.hasBorder ? "border-b border-white/10" : ""}`}
        onClick={handleParentClick}
      >
        <span class="flex-grow" style={{ "padding-left": `${level * 24}px` }}>
          {props.children}
        </span>
        {hasChildren && (
          <button
            type="button"
            onClick={toggleExpanded}
            class="w-5 h-5 flex items-center justify-center font-bold text-white/60 hover:text-white focus:outline-none bg-transparent border-none transition-colors duration-200"
            aria-expanded={isExpanded()}
            aria-label={`${isExpanded() ? "Collapse" : "Expand"} ${props.children} section`}
            tabIndex={0}
          >
            <div
              class="transform transition-transform duration-300 ease-in-out"
              classList={{ "rotate-180": isExpanded() }}
            >
              <IoChevronDown size={16} />
            </div>
          </button>
        )}
      </A>
      {hasChildren && (
        <div
          ref={contentRef}
          class="overflow-hidden transition-all duration-300 ease-in-out"
          style={{
            "max-height": isExpanded()
              ? props.maxHeight && contentHeight() > props.maxHeight
                ? `${props.maxHeight}px`
                : `${contentHeight()}px`
              : "0px",
            opacity: isExpanded() ? "1" : "0",
            transform: `translateY(${isExpanded() ? "0" : "-10px"})`,
            overflow:
              isExpanded() &&
              props.maxHeight &&
              contentHeight() > props.maxHeight
                ? "auto"
                : "hidden",
          }}
        >
          <div>
            <For each={props.childrenItems}>
              {(child) => (
                <HeaderDropdownItem
                  href={child.path}
                  hasBorder={false}
                  onSelect={props.onSelect}
                  childrenItems={child.children}
                  level={level + 1}
                  maxHeight={props.maxHeight}
                  onHeightChange={(height) => {
                    calculateHeight();
                  }}
                >
                  {child.title}
                </HeaderDropdownItem>
              )}
            </For>
          </div>
        </div>
      )}
    </div>
  );
}
