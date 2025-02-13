import { Title } from "@solidjs/meta";
import styles from "./index.module.css";

export default function Home() {
  return (
    <div class={styles.container}>
      <Title>Tombi - Modern TOML Formatter</Title>

      <section class={styles.hero}>
        <h1 class={styles.title}>Tombi</h1>
        <p class={styles.subtitle}>Next Generation TOML Formatter</p>

        <div class={styles.features}>
          <div class={styles.feature}>
            <h3>⚡️ Fast</h3>
            <p>High-performance formatter implemented in Rust</p>
          </div>
          <div class={styles.feature}>
            <h3>🎯 Accurate</h3>
            <p>Full compliance with TOML specification</p>
          </div>
          <div class={styles.feature}>
            <h3>🛠 Customizable</h3>
            <p>Flexible configuration for your project needs</p>
          </div>
        </div>

        <div class={styles.cta}>
          <a href="/documentation/getting-started/installation" class={styles.primaryButton}>
            Get Started
          </a>
          <a href="/documentation" class={styles.secondaryButton}>
            View Docs
          </a>
        </div>
      </section>

      <section class={styles.demo}>
        <h2>Simple and Easy to Use</h2>
        <pre class={styles.codeBlock}>
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
