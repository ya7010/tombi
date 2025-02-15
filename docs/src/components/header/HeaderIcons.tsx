import { createSignal, onMount } from "solid-js";
import { TbMoonFilled, TbSunFilled } from "solid-icons/tb";
import { LinkImageButton } from "../common/LinkImageButton";

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
    <div class="hidden md:flex items-center px-4 space-x-4 flex-shrink-0 input-focus">
      <button
        id="dark-mode-toggle"
        onClick={toggleDarkMode}
        class="text-white hover:text-white/80 transition-colors bg-transparent border-0 btn-focus"
        aria-label="Toggle dark mode"
      >
        {isDark() ? <TbMoonFilled size={28}/> : <TbSunFilled size={28}/>}
      </button>
      <LinkImageButton
        href="https://github.com/tombi-toml/tombi"
        src="/github-mark.svg"
        alt="GitHub"
        classes="w-6 h-6"
      />
    </div>
  );
}
