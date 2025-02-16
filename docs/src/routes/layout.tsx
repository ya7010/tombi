import { Header } from "~/components/header/index";

// biome-ignore lint/suspicious/noExplicitAny: <explanation>
export default function Layout(props: { children: any }) {
  return (
    <div class="min-h-screen bg-gray-50 dark:bg-gray-700 text-gray-900 dark:text-gray-100">
      <Header />
      {props.children}
    </div>
  );
}
