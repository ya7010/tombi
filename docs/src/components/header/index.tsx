import { HeaderLogo } from "./HeaderLogo";
import { HeaderTabs } from "./HeaderTabs";
import { HeaderSearch } from "./HeaderSearch";
import { HeaderIcons } from "./HeaderIcons";

export function Header() {
  return (
    <header class="fixed top-0 left-0 right-0 bg-tombi-primary shadow-lg z-50">
      <nav class="max-w-7xl mx-auto">
        <div class="flex justify-between h-20">
          <div class="flex items-center">
            <HeaderLogo />
            <HeaderTabs />
          </div>
          <HeaderSearch />
          <HeaderIcons />
        </div>
      </nav>
    </header>
  );
}
