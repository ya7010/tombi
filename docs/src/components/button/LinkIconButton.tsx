import { A } from "@solidjs/router";
import { ParentComponent } from "solid-js";

interface LinkIconButtonProps {
  id?: string;
  href: string;
  class?: string;
  alt: string;
}

export const LinkIconButton: ParentComponent<LinkIconButtonProps> = (props) => {
  const baseClasses = "text-white hover:text-white/80 bg-transparent border-0 btn-focus";

  return (
    <A
      id={props.id}
      href={props.href}
      target="_blank"
      rel="noopener noreferrer"
      class={`${baseClasses} ${props.class}`}
      aria-label={props.alt}
    >
      {props.children}
    </A>
  );
};
