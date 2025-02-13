import { Title } from "@solidjs/meta";

export default function Home() {
  return (
    <div class="max-w-6xl mx-auto px-4 py-12">
      <Title>Tombi - Modern TOML Formatter</Title>

      <section class="text-center mb-16">
        <h1 class="text-5xl font-bold mb-4">Tombi</h1>
        <p class="text-xl text-gray-600 mb-12">Next Generation TOML Formatter</p>

        <div class="grid md:grid-cols-3 gap-8 mb-12">
          <div class="p-6 rounded-lg bg-gray-50">
            <h3 class="text-xl font-semibold mb-2">⚡️ Fast</h3>
            <p class="text-gray-600">High-performance formatter implemented in Rust</p>
          </div>
          <div class="p-6 rounded-lg bg-gray-50">
            <h3 class="text-xl font-semibold mb-2">🎯 Accurate</h3>
            <p class="text-gray-600">Full compliance with TOML specification</p>
          </div>
          <div class="p-6 rounded-lg bg-gray-50">
            <h3 class="text-xl font-semibold mb-2">🛠 Customizable</h3>
            <p class="text-gray-600">Flexible configuration for your project needs</p>
          </div>
        </div>

        <div class="flex gap-4 justify-center">
          <a
            href="/documentation/getting-started/installation"
            class="px-6 py-3 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            Get Started
          </a>
          <a
            href="/documentation"
            class="px-6 py-3 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
          >
            View Docs
          </a>
        </div>
      </section>

      <section class="max-w-3xl mx-auto">
        <h2 class="text-3xl font-bold text-center mb-8">Simple and Easy to Use</h2>
        <pre class="p-6 bg-gray-900 text-white rounded-lg overflow-x-auto">
          <code>{`# Before
title="TOML Example"
[package]
name="my-project"
version="0.1.0"
authors=["John Doe <john@example.com>",]

# After
title = "TOML Example"

[package]
name = "my-project"
version = "0.1.0"
authors = [
  "John Doe <john@example.com>",
]`}</code>
        </pre>
      </section>
    </div>
  );
}
