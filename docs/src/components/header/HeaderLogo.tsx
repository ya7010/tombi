import { A } from "@solidjs/router";
import { createSignal } from "solid-js";
import { HeaderDropdown } from "./HeaderDropdown";

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
        <A href="/" class="flex items-center no-underline" onClick={(e) => e.preventDefault()}>
          <img
            src="/icon.svg"
            alt="Tombi Logo"
            class="h-16 w-16 md:hidden"
          />
          <img
            src="/tombi.svg"
            alt="Tombi Logo"
            class="hidden md:block h-16 w-auto"
          />
        </A>
      </div>

      <HeaderDropdown isOpen={isOpen} onSelect={handleSelect} />
    </div>
  );
}
