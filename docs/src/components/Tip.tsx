import type { ParentComponent } from "solid-js";

export const Tip: ParentComponent = (props) => {
  return (
    <div class="my-4 p-4 bg-blue-50 border-l-4 border-blue-500 dark:bg-blue-950 dark:border-blue-400">
      <div class="flex items-center gap-2 font-medium text-blue-800 dark:text-blue-200 mb-2">
        <span>💡</span>
        <span>Tip</span>
      </div>
      <div class="text-gray-700 dark:text-gray-300">{props.children}</div>
    </div>
  );
};
