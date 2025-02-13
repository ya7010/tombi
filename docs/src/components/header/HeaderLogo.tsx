import { A } from "@solidjs/router";

export function HeaderLogo() {
  return (
    <div class="flex-shrink-0 flex items-center px-4">
      <A href="/" class="flex items-center no-underline">
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
  );
}
