import { HeaderTab } from "./HeaderTab";

export function HeaderTabs() {
  return (
    <div class="hidden md:flex items-center space-x-4 px-4">
      <HeaderTab href="/docs">Docs</HeaderTab>
      <HeaderTab href="/playground">Playground</HeaderTab>
    </div>
  );
}
