import { A } from "@solidjs/router";
import type { ParentComponent } from "solid-js";

interface ImageButtonProps {
  id?: string;
  href: string;
  src: string;
  alt: string;
  class?: string;
}

export const LinkImageButton: ParentComponent<ImageButtonProps> = (props) => {
  const baseClasses =
    "text-white hover:text-white/80 transition-colors no-underline btn-focus";

  return (
    <A
      id={props.id}
      href={props.href}
      target="_blank"
      rel="noopener noreferrer"
      class={`${baseClasses} ${props.class}`}
      aria-label={props.alt}
    >
      <img src={props.src} alt={props.alt} class={props.class} />
    </A>
  );
};
