import { A } from "@solidjs/router";
import { createSignal, For } from "solid-js";
import { HeaderDropdown } from "./HeaderDropdown";

type LogoMode = {
  id: string;
  src: string;
  class: string;
  linkClass: string;
  preventDefault: boolean;
};

const logoModes: LogoMode[] = [
  {
    id: "mobile-logo",
    src: "/icon.svg",
    class: "h-16 w-16",
    linkClass: "md:hidden flex",
    preventDefault: true,
  },
  {
    id: "desktop-logo",
    src: "/tombi.svg",
    class: "h-16 w-auto",
    linkClass: "hidden md:flex",
    preventDefault: false,
  },
];

export function HeaderLogo() {
  const [isOpen, setIsOpen] = createSignal(false);

  const toggleMenu = () => {
    setIsOpen(!isOpen());
  };

  const handleSelect = () => {
    setIsOpen(false);
  };

  return (
    <div class="flex-shrink-0 flex items-center px-4 relative">
      <div onClick={toggleMenu} class="cursor-pointer md:cursor-default">
        <For each={logoModes}>
          {(config) => (
            <A
              id={config.id}
              href="/"
              class={`${config.linkClass} items-center no-underline`}
              onClick={(e) => config.preventDefault && e.preventDefault()}
            >
              <img
                src={config.src}
                alt="Tombi Logo"
                class={config.class}
              />
            </A>
          )}
        </For>
      </div>

      <HeaderDropdown isOpen={isOpen} onSelect={handleSelect} />
    </div>
  );
}
