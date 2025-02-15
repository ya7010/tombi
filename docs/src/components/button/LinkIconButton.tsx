import { ParentComponent } from "solid-js";

interface LinkIconButtonProps {
  id?: string;
  onClick: () => void;
  class?: string;
  alt: string;
}

export const LinkIconButton: ParentComponent<LinkIconButtonProps> = (props) => {
  const baseClasses = "text-white hover:text-white/80 bg-transparent border-0 btn-focus";

  return (
    <button
        id={props.id}
        onClick={props.onClick}
        class={`${baseClasses} ${props.class}`}
        aria-label={props.alt}
      >
        {props.children}
      </button>
  );
};
