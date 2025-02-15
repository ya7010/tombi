import { ParentComponent } from "solid-js";

interface LinkIconButtonProps {
  id?: string;
  onClick: () => void;
  classes?: string;
  alt: string;
}

export const LinkIconButton: ParentComponent<LinkIconButtonProps> = (props) => {
  return (
    <button
        id={props.id}
        onClick={props.onClick}
        class="text-white hover:text-white/80 transition-colors bg-transparent border-0 btn-focus"
        aria-label={props.alt}
      >
        {props.children}
      </button>
  );
};
