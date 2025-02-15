import { A } from "@solidjs/router";
import { ParentComponent } from "solid-js";

interface ImageButtonProps {
  href: string;
  src: string;
  alt: string;
  classes?: string;
}

export const LinkImageButton: ParentComponent<ImageButtonProps> = (props) => {
  return (
    <A
      href={props.href}
      target="_blank"
      rel="noopener noreferrer"
      class="text-white hover:text-white/80 transition-colors no-underline btn-focus"
      aria-label={props.alt}
    >
      <img
        src={props.src}
        alt={props.alt}
        class={props.classes}
      />
    </A>
  );
};
