import { HeaderTab } from "./HeaderTab";

export function HeaderTabs() {
  return (
    <div class="hidden md:flex items-center px-8 space-x-8">
      <HeaderTab href="/concepts">Concepts</HeaderTab>
      <HeaderTab href="/documentation">Docs</HeaderTab>
      <HeaderTab href="/playground">Playground</HeaderTab>
    </div>
  );
}
