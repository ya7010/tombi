import { HeaderTab } from "./HeaderTab";

export function HeaderTabs() {
  return (
    <div class="hidden md:flex items-center px-8 space-x-8">
      <HeaderTab href="/documentation">Docs</HeaderTab>
      <HeaderTab href="/concepts">Concepts</HeaderTab>
    </div>
  );
}
