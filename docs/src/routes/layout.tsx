import { Header } from "~/components/Header";

export default function Layout(props: { children: any }) {
  return (
    <div class="min-h-screen bg-gray-50">
      <Header />
      <main class="pt-20 py-10">
        <div class="max-w-7xl mx-auto sm:px-6 lg:px-8">
          {props.children}
        </div>
      </main>
    </div>
  );
}
