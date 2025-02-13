import { A } from "@solidjs/router";

export function HeaderTabs() {
  return (
    <div class="hidden md:flex items-center px-8 space-x-8">
      <A
        href="/documentation"
        class="text-white hover:text-white/80 text-lg font-medium no-underline"
      >
        Docs
      </A>
    </div>
  );
}
