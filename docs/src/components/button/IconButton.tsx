import { ParentComponent } from "solid-js";


interface IconButtonProps {
  id?: string;
  onClick: () => void;
  alt: string;
  class?: string;
}

export const IconButton: ParentComponent<IconButtonProps> = (props) => {
  const baseClasses = "text-white hover:text-white/80 transition-colors bg-transparent border-0 btn-focus";

  return (
      <button
        id={props.id}
        onClick={props.onClick}
        class={`${baseClasses} ${props.class || ""}`}
        aria-label={props.alt}
      >
        {props.children}
      </button>
  );
};
