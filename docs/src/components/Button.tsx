import { A } from "@solidjs/router";
import { ParentComponent } from "solid-js";

type ButtonVariant = "primary" | "secondary";

interface ButtonProps {
  href: string;
  variant?: ButtonVariant;
}

export const Button: ParentComponent<ButtonProps> = (props) => {
  const baseClasses = "px-8 py-4 rounded-xl transition-colors shadow-lg hover:shadow-xl no-underline btn-focus";

  const variantClasses = {
    primary: ["bg-tombi-primary text-white",
      "hover:bg-white hover:text-tombi-primary hover:border-tombi-primary hover:border-5"
    ].join(" "),
    secondary: "bg-white hover:bg-tombi-50 text-tombi-primary "
  };

  const classes = `${baseClasses} ${variantClasses[props.variant || "primary"]}`;

  return (
      <A href={props.href} class={classes}>
        {props.children}
      </A>
  );
};
