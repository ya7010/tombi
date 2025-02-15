import { TbSearch, TbX } from "solid-icons/tb";
import { createSignal, onMount } from "solid-js";
import { detectOperatingSystem } from "~/utils/platform";
import { IconButton } from "../button/IconButton";

export function HeaderSearch() {
  const [isSearchOpen, setIsSearchOpen] = createSignal(false);
  const [isMac, setIsMac] = createSignal(false);
  let searchInputRef: HTMLInputElement | undefined;

  onMount(() => {
    setIsMac(detectOperatingSystem() === 'mac');

    if (typeof window !== 'undefined') {
      document.addEventListener('keydown', (e) => {
        if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
          e.preventDefault();
          searchInputRef?.focus();
          setIsSearchOpen(true);
        }
      });
    }
  });

  return (
    <div class="flex justify-end w-full items-center">
      <div class={`${
        isSearchOpen()
          ? 'items-center'
          : 'hidden'
        } md:flex md:items-center w-full`}>
        <div class="relative w-full">
          <div class="absolute left-3 top-1/2 -translate-y-1/2 text-white/60">
            <TbSearch size={24}/>
          </div>
          <input
            ref={searchInputRef}
            type="text"
            placeholder="Search"
            class="w-full h-12 pl-12 bg-white/10 text-white placeholder-white/60 rounded-0 text-lg focus:bg-white/[0.15] border-white outline-none box-border rounded-2"
          />
          <div class="absolute right-4 top-1/2 -translate-y-1/2 text-white/60 text-lg">
            {isMac() ? '⌘K' : 'Ctrl+K'}
          </div>
        </div>
      </div>
      <IconButton
        onClick={() => {
          setIsSearchOpen(!isSearchOpen());
        }}
        classes="md:hidden px-6"
        alt={isSearchOpen() ? "Close Search" : "Search"}
      >
        {isSearchOpen() ? (
          <TbX size={24}/>
        ) : (
          <TbSearch size={24}/>
        )}
      </IconButton>
    </div>
  );
}
