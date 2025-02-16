import type { ParentComponent } from "solid-js";
import { CopyButton } from "./CopyButton";

interface CodeBlockProps {
  code: string;
  language?: string;
}

export const CodeBlock: ParentComponent<CodeBlockProps> = (props) => {
  return (
    <div class="relative">
      <pre class={`language-${props.language || "text"}`}>
        <code class={`language-${props.language || "text"}`}>{props.code}</code>
      </pre>
      <CopyButton text={props.code} />
    </div>
  );
};
