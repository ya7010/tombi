import { createSignal, onMount } from "solid-js";

export function HeaderSearch() {
  const [isSearchOpen, setIsSearchOpen] = createSignal(false);
  let searchInputRef: HTMLInputElement | undefined;

  onMount(() => {
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
    <div class="flex-1 flex items-center justify-end md:justify-center mx-4 h-full">
      {/* モバイル用検索アイコン */}
      <button
        onClick={() => {
          setIsSearchOpen(!isSearchOpen());
          if (!isSearchOpen()) {
            setTimeout(() => searchInputRef?.focus(), 100);
          }
        }}
        class="md:hidden flex items-center justify-center p-2 text-white hover:text-white/80 transition-colors bg-transparent border-0 outline-none"
        aria-label={isSearchOpen() ? "Close search" : "Search"}
      >
        {isSearchOpen() ? (
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        ) : (
          <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        )}
      </button>
      {/* デスクトップ用検索バー */}
      <div class={`${
        isSearchOpen()
          ? 'absolute left-24 right-12 top-1/2 -translate-y-1/2 bg-tombi-900'
          : 'hidden'
        } md:static md:flex md:items-center md:w-full md:max-w-[320px] md:h-10 md:my-auto`}>
        <div class="relative w-full">
          <div class="absolute left-3 top-1/2 -translate-y-1/2 text-white/60">
            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
            </svg>
          </div>
          <input
            ref={searchInputRef}
            type="text"
            placeholder="Search"
            class="w-full h-10 bg-white/10 text-white placeholder-white/60 rounded-lg pl-10 pr-12 focus:outline-none focus:ring-2 focus:ring-white/30 transition-all"
          />
          <div class="absolute right-2 top-1/2 -translate-y-1/2 text-white/60 text-sm">
            ⌘K
          </div>
        </div>
      </div>
    </div>
  );
}
