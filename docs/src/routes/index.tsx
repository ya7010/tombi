import { Title } from "@solidjs/meta";
import { FeatureCard } from "~/components/FeatureCard";

const FEATURES = [
  {
    emoji: "⚡️",
    title: "Fast",
    description: "High-performance formatter implemented in Rust",
  },
  {
    emoji: "🎯",
    title: "Accurate",
    description: "Full compliance with TOML specification",
  },
  {
    emoji: "🛠",
    title: "Customizable",
    description: "Flexible configuration for your project needs",
  },
] as const;

export default function Home() {
  return (
    <div>
      <Title>Tombi - Modern TOML Formatter</Title>

      <section class="text-center mb-24">
          <h1 class="sr-only">Tombi</h1>
          <div class="relative mb-16 w-screen -mx-[calc((100vw-100%)/2)] overflow-hidden bg-gray-900">
              <img
                src="/tombi-transparent.svg"
                alt="Tombi Logo"
                class="w-auto max-h-80 mx-auto"
              />
          </div>

          <p class="text-xl text-tombi-800/80 dark:text-gray-300 mb-16 max-w-2xl mx-auto">
            Next Generation TOML Formatter - Bringing elegance and precision to your TOML configurations
          </p>

          <div class="grid md:grid-cols-3 gap-8 mb-16">
            {FEATURES.map((feature) => (
              <FeatureCard {...feature} />
            ))}
          </div>

          <div class="flex gap-4 justify-center">
            <a
              href="/documentation/getting-started/installation"
              class="px-8 py-4 bg-tombi-900 text-white rounded-xl hover:bg-tombi-800 transition-colors shadow-lg hover:shadow-xl no-underline"
            >
              Get Started
            </a>
            <a
              href="/documentation"
              class="px-8 py-4 bg-white dark:bg-tombi-900/30 border border-tombi-200 dark:border-tombi-700 rounded-xl hover:bg-tombi-50 dark:hover:bg-tombi-900/50 transition-colors text-tombi-900 dark:text-white shadow-lg hover:shadow-xl no-underline"
            >
              View Docs
            </a>
          </div>
      </section>

      <section class="max-w-3xl mx-auto px-4">
        <h2 class="text-3xl font-bold text-center mb-8 bg-gradient-to-r from-tombi-900 to-tombi-700 dark:from-white dark:to-tombi-200 bg-clip-text text-transparent">
          Simple and Easy to Use
        </h2>
        <pre class="p-8 bg-tombi-900 text-white rounded-xl overflow-x-auto shadow-lg text-left">
          <code class="text-left">{`# Before
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
