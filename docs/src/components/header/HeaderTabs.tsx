import { HeaderTab } from "./HeaderTab";

export function HeaderTabs() {
  return (
    <div class="hidden md:flex items-center space-x-8">
      <HeaderTab href="/documentation">Docs</HeaderTab>
      <HeaderTab href="/playground">Playground</HeaderTab>
    </div>
  );
}
