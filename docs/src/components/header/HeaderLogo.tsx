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
    src: "/icon.svg",
    class: "h-16 w-16",
    linkClass: "md:hidden flex",
    preventDefault: true,
  },
  {
    id: "desktop-logo",
    src: "/tombi-transparent.svg",
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
    <div class="flex-shrink-0 flex items-center relative ">
      <div onClick={toggleMenu} class="cursor-pointer md:cursor-default">
        <For each={logoProps}>
          {(props) => (
            <A
              id={props.id}
              href="/"
              class={`ml-4 ${props.linkClass} outline-none items-center no-underline transition-colors focus-visible:ring-2 focus-visible:ring-tombi-focus focus:rounded-lg relative`}
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
