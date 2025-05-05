import type { ParentComponent } from "solid-js";

interface IconButtonProps {
  id?: string;
  onClick: () => void;
  alt: string;
  class?: string;
  style?: string;
}

export const IconButton: ParentComponent<IconButtonProps> = (props) => {
  const baseClasses =
    "text-white hover:text-white/80 transition-colors bg-transparent border-0 p-2 btn-focus";

  return (
    <button
      type="button"
      id={props.id}
      onClick={props.onClick}
      class={`${baseClasses} ${props.class || ""}`}
      aria-label={props.alt}
      style={props.style}
    >
      {props.children}
    </button>
  );
};
