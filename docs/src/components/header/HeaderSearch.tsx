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
    <div class="flex flex-auto items-center justify-end md:justify-center mx-4 h-full">
      <div class={`${
        isSearchOpen()
          ? 'absolute left-32 right-12 top-1/2 -translate-y-1/2 bg-tombi-primary'
          : 'hidden'
        } md:static md:flex md:items-center md:w-auto md:max-w-[320px] md:h-10 md:my-auto`}>
        <div class="relative w-full">
          <div class="absolute left-3 top-1/2 -translate-y-1/2 text-white/60">
            <TbSearch size={28}/>
          </div>
          <input
            ref={searchInputRef}
            type="text"
            placeholder="Search"
            class="w-full h-10 bg-white/10 text-white placeholder-white/60 rounded-lg pl-10 pr-12 focus:bg-white/[0.15]"
          />
          <div class="absolute right-2 top-1/2 -translate-y-1/2 text-white/60 text-sm">
            {isMac() ? '⌘K' : 'Ctrl+K'}
          </div>
        </div>
      </div>
      <IconButton
        onClick={() => {
          setIsSearchOpen(!isSearchOpen());
          if (!isSearchOpen()) {
            setTimeout(() => searchInputRef?.focus(), 100);
          }
        }}
        classes="md:hidden flex items-center justify-center"
        alt={isSearchOpen() ? "Close Search" : "Search"}
      >
        {isSearchOpen() ? (
          <TbX size={28}/>
        ) : (
          <TbSearch size={28}/>
        )}
      </IconButton>
    </div>
  );
}
