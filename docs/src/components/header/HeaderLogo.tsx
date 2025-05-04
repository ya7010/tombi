import { A } from "@solidjs/router";
import { createSignal, For } from "solid-js";
import { HeaderDropdown } from "./HeaderDropdown";

type LogoProps = {
  id: string;
  src: string;
  class: string;
  linkClass: string;
  preventDefault: boolean;
};

const logoProps: LogoProps[] = [
  {
    id: "mobile-logo",
    src: `${import.meta.env.BASE_URL}/icon.svg`,
    class: "mx-4 h-16 w-16",
    linkClass: "md:hidden flex",
    preventDefault: true,
  },
  {
    id: "desktop-logo",
    src: `${import.meta.env.BASE_URL}/tombi-transparent.svg`,
    class: "ml-4 mr-0 h-16 w-auto",
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
    <div class="flex-shrink-0 flex items-center relative">
      <div
        onClick={toggleMenu}
        onKeyUp={toggleMenu}
        class="cursor-pointer md:cursor-default"
      >
        <For each={logoProps}>
          {(props) => (
            <A
              id={props.id}
              href="/"
              class={`${props.linkClass} outline-none items-center no-underline transition-all duration-300 ease-in-out focus-visible:ring-2 focus-visible:ring-tombi-focus focus:rounded-lg relative`}
              onClick={(e) => props.preventDefault && e.preventDefault()}
            >
              <img
                src={props.src}
                alt="Tombi Logo"
                class={`${props.class} rounded-lg`}
              />
            </A>
          )}
        </For>
      </div>

      <HeaderDropdown isOpen={isOpen} onSelect={handleSelect} />
    </div>
  );
}
