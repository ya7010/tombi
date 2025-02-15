import { A } from "@solidjs/router";
import { ParentComponent } from "solid-js";

type ButtonVariant = "primary" | "secondary";

interface ButtonProps {
  href: string;
  variant?: ButtonVariant;
  class?: string;
}

export const LinkButton: ParentComponent<ButtonProps> = (props) => {
  const baseClasses =
    "px-8 py-4 rounded-xl transition-colors shadow-lg hover:shadow-xl no-underline btn-focus";

  const variantClasses = {
    primary: [
      "bg-tombi-primary text-white ring-transparent ring-3 ring-inset",
      "hover:bg-tombi-50 hover:text-tombi-primary hover:ring-tombi-600",
      "focus:bg-tombi-50 focus:text-tombi-primary focus:ring-tombi-600",
    ].join(" "),
    secondary: [
      "bg-white text-tombi-primary ring-transparent ring-3 ring-inset",
      "hover:bg-tombi-50 hover:ring-tombi-600",
      "focus:bg-tombi-50 focus:ring-tombi-600",
    ].join(" "),
  };

  const classes = `${baseClasses} ${variantClasses[props.variant || "primary"]} ${props.class || ""}`;

  return (
    <A href={props.href} class={classes}>
      {props.children}
    </A>
  );
};
