import { createSignal } from "solid-js";
import { TbCheck, TbCopy } from "solid-icons/tb";

interface CopyButtonProps {
  text: string;
}

export function CopyButton(props: CopyButtonProps) {
  const [copied, setCopied] = createSignal(false);

  const copyToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(props.text);
      setCopied(true);
      setTimeout(() => setCopied(false), 1000);
    } catch (err) {
      console.error("Failed to copy text: ", err);
    }
  };

  return (
    <button
      type="button"
      onClick={copyToClipboard}
      class="absolute top-0 right-0 p-3 bg-transparent border-0 text-gray-400 hover:text-gray-300 transition-colors"
      aria-label="Copy code to clipboard"
    >
      {copied() ? (
        <TbCheck class="w-4 h-4 text-green-400" />
      ) : (
        <TbCopy class="w-4 h-4" />
      )}
    </button>
  );
}
