import { A } from "@solidjs/router";

interface HeaderTabProps {
  href: string;
  children: string;
}

export function HeaderTab(props: HeaderTabProps) {
  return (
    <A
      href={props.href}
      class="text-white hover:text-white/80 text-lg font-medium no-underline"
    >
      {props.children}
    </A>
  );
}
