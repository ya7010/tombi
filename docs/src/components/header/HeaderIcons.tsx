import { createSignal, onMount } from "solid-js";
import { TbBrandGithub, TbMoonFilled, TbSunFilled } from "solid-icons/tb";
import { LinkIconButton } from "../button/LinkIconButton";
import { IconButton } from "../button/IconButton";

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
    <div class="hidden md:flex items-center px-4 space-x-4">
      <IconButton
        id="dark-mode-toggle"
        onClick={toggleDarkMode}
        alt="Toggle dark mode"
        class={`flex items-center justify-center md:btn-focus transition-transform duration-300 ease-out forwards ${isDark() ? 'rotate-0' : '-rotate-90'}`}
      >
        {
          isDark()
            ? <TbMoonFilled size={28}/>
            : <TbSunFilled size={28}/>
        }
      </IconButton>
      <LinkIconButton
        href="https://github.com/tombi-toml/tombi"
        alt="GitHub"
        class="w-6 h-6"
      >
        <TbBrandGithub size={28} aria-label="GitHub"/>
      </LinkIconButton>
    </div>
  );
}
