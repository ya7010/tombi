import { ParentComponent } from "solid-js";

type ButtonVariant = "primary" | "secondary";

interface ButtonProps {
  href?: string;
  variant?: ButtonVariant;
}

export const Button: ParentComponent<ButtonProps> = (props) => {
  const baseClasses = "px-8 py-4 rounded-xl transition-colors shadow-lg hover:shadow-xl no-underline";

  const variantClasses = {
    primary: "bg-tombi-900 text-white hover:bg-tombi-800",
    secondary: "bg-white dark:bg-tombi-900/30 border border-tombi-200 dark:border-tombi-700 hover:bg-tombi-50 dark:hover:bg-tombi-900/50 text-tombi-900 dark:text-white"
  };

  const classes = `${baseClasses} ${variantClasses[props.variant || "primary"]}`;

  return (
    <a href={props.href} class={classes}>
      {props.children}
    </a>
  );
};
