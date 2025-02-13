import { A } from "@solidjs/router";
import { createSignal, onMount, Show } from "solid-js";

export function Header() {
  const [isDark, setIsDark] = createSignal(false);
  const [isMenuOpen, setIsMenuOpen] = createSignal(false);

  onMount(() => {
    if (typeof window !== 'undefined') {
      const darkModePreference = window.matchMedia('(prefers-color-scheme: dark)').matches;
      const storedTheme = localStorage.getItem('theme');
      setIsDark(storedTheme === 'dark' || (!storedTheme && darkModePreference));
      document.documentElement.classList.toggle('dark', isDark());
    }
  });

  const toggleDarkMode = () => {
    const newDarkMode = !isDark();
    setIsDark(newDarkMode);
    localStorage.setItem('theme', newDarkMode ? 'dark' : 'light');
    document.documentElement.classList.toggle('dark', newDarkMode);
  };

  const toggleMenu = () => {
    setIsMenuOpen(!isMenuOpen());
  };

  return (
    <header class="fixed top-0 left-0 right-0 bg-[rgb(0,0,102)] shadow-lg z-50">
      <nav class="max-w-7xl mx-auto">
        <div class="flex justify-between h-20">
          <div class="flex">
            <div class="flex-shrink-0 flex items-center px-4">
              <A href="/" class="flex items-center no-underline">
                <img
                  src="/tombi.svg"
                  alt="Tombi Logo"
                  class="h-16 w-auto hidden md:block"
                />
                <img
                  src="/icon.svg"
                  alt="Tombi Logo"
                  class="h-16 w-16 md:hidden"
                />
              </A>
            </div>
            <div class="hidden md:flex items-center px-8 space-x-8">
              <A
                href="/documentation"
                class="text-[#FFFFFF] hover:text-[#FFFFFF]/80 text-lg font-medium no-underline"
              >
                Docs
              </A>
            </div>
          </div>
          <div class="flex items-center px-4 space-x-4">
            <button
              id="dark-mode-toggle"
              onClick={toggleDarkMode}
              class="text-[#FFFFFF] hover:text-[#FFFFFF]/80 p-2 transition-colors bg-transparent border-0 outline-none"
              aria-label="Toggle dark mode"
            >
              {isDark() ? (
                <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                  />
                </svg>
              ) : (
                <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
                  />
                </svg>
              )}
            </button>
            <a
              href="https://github.com/tombi-toml/tombi"
              target="_blank"
              rel="noopener noreferrer"
              class="text-[#FFFFFF] hover:text-[#FFFFFF]/80 transition-colors no-underline"
              aria-label="GitHub repository"
            >
              <svg
                viewBox="0 0 16 16"
                class="w-6 h-6"
                fill="currentColor"
                aria-hidden="true"
                stroke-width="0.5"
              >
                <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z" />
              </svg>
            </a>
            <button
              onClick={toggleMenu}
              class="md:hidden text-[#FFFFFF] hover:text-[#FFFFFF]/80 p-2 transition-colors bg-transparent border-0 outline-none"
              aria-label="Toggle menu"
            >
              <svg
                class="w-6 h-6"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
                stroke-width="2.5"
              >
                {isMenuOpen() ? (
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M6 18L18 6M6 6l12 12"
                  />
                ) : (
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M4 6h16M4 12h16M4 18h16"
                  />
                )}
              </svg>
            </button>
          </div>
        </div>

        {/* モバイルメニュー */}
        <Show when={isMenuOpen()}>
          <div class="md:hidden">
            <div class="px-2 pt-2 pb-3 space-y-1">
              <A
                href="/documentation"
                class="block px-3 py-2 rounded-md text-lg font-medium text-[#FFFFFF] hover:text-[#FFFFFF]/80 hover:bg-[rgba(255,255,255,0.1)] transition-colors no-underline"
              >
                Documentation
              </A>
            </div>
          </div>
        </Show>
      </nav>
    </header>
  );
}
