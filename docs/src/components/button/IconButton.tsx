import { ParentComponent } from "solid-js";


interface ButtonProps {
  onClick: () => void;
  alt: string;
  classes?: string;
}

export const IconButton: ParentComponent<ButtonProps> = (props) => {
  const baseClasses = "text-white hover:text-white/80 transition-colors bg-transparent border-0 btn-focus";

  const classes = `${baseClasses} ${props.classes || ""}`;

  return (
      <button
        onClick={props.onClick}
        class={classes}
        aria-label={props.alt}
      >
        {props.children}
      </button>
  );
};
