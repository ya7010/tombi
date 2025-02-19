import { HeaderLogo } from "./HeaderLogo";
import { HeaderTabs } from "./HeaderTabs";
import { HeaderSearch } from "./HeaderSearch";
import { HeaderIcons } from "./HeaderIcons";

export function Header() {
  return (
    <header class="fixed top-0 left-0 right-0 bg-tombi-primary shadow-lg z-40">
      <nav class="max-w-7xl mx-auto">
        <div class="flex justify-between h-20">
          <HeaderLogo />
          <HeaderTabs />
          <HeaderSearch />
          <HeaderIcons />
        </div>
      </nav>
    </header>
  );
}
