import { A } from "@solidjs/router";
import { ParentComponent } from "solid-js";

type ButtonVariant = "primary" | "secondary";

interface ButtonProps {
  href: string;
  variant?: ButtonVariant;
}

export const Button: ParentComponent<ButtonProps> = (props) => {
  const baseClasses = "px-8 py-4 rounded-xl transition-colors shadow-lg hover:shadow-xl no-underline";

  const variantClasses = {
    primary: ["bg-tombi-900 text-white border-transparent",
      "hover:bg-white hover:text-tombi-900 hover:border-tombi-900 hover:border-5"
    ].join(" "),
    secondary: "bg-white border border-tombi-200 hover:bg-tombi-50 text-tombi-900 "
  };

  const classes = `${baseClasses} ${variantClasses[props.variant || "primary"]}`;

  return (
    <A href={props.href} class={classes}>
      {props.children}
    </A>
  );
};
