import { createSignal, onMount } from "solid-js";

export function HeaderIcons() {
  const [isDark, setIsDark] = createSignal(false);

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

  return (
    <div class="hidden md:flex items-center px-4 space-x-4 flex-shrink-0">
      <button
        id="dark-mode-toggle"
        onClick={toggleDarkMode}
        class="text-white hover:text-white/80 transition-colors bg-transparent border-0 btn-focus"
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
        class="text-white hover:text-white/80 transition-colors no-underline btn-focus"
        aria-label="GitHub repository"
      >
        <img
          src="/github-mark-white.svg"
          alt="GitHub"
          class="w-6 h-6"
        />
      </a>
    </div>
  );
}
