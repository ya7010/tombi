import { A } from "@solidjs/router";

export function Header() {
  return (
    <header class="fixed top-0 left-0 right-0 bg-[rgb(0,0,102)] shadow-lg z-50">
      <nav class="max-w-7xl mx-auto">
        <div class="flex h-20">
          <div class="flex-shrink-0 flex items-center px-4">
            <A href="/" class="flex items-center">
              <img src="/icon.svg" alt="Tombi Logo" class="h-16 w-16" />
            </A>
          </div>
          <div class="flex items-center px-8 space-x-8">
            <A
              href="/documentation"
              class="text-gray-300 hover:text-white text-base font-medium"
            >
              Documentation
            </A>
          </div>
        </div>
      </nav>
    </header>
  );
}
