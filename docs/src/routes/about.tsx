import { Title } from "@solidjs/meta";

export default function About() {
  return (
    <main class="p-8 max-w-4xl mx-auto">
      <Title>About Tombi</Title>

      <h1 class="text-4xl font-bold mb-6">About Tombi</h1>

      <section class="mb-8">
        <h2 class="text-2xl font-semibold mb-4">Project Mission</h2>
        <p class="mb-4">
          Tombi is a language server developed to enhance the TOML file editing
          experience. It provides modern development environment features to
          make TOML file editing more efficient and comfortable.
        </p>
      </section>

      <section class="mb-8">
        <h2 class="text-2xl font-semibold mb-4">Technology Stack</h2>
        <ul class="list-disc pl-6 space-y-2">
          <li>Core Language Server: Rust</li>
          <li>VS Code Extension: TypeScript</li>
          <li>Schema Processing: JSONSchema Compatible</li>
        </ul>
      </section>

      <section class="mb-8">
        <h2 class="text-2xl font-semibold mb-4">Contributing</h2>
        <p class="mb-4">
          Tombi is developed as an open-source project. We welcome all forms of
          contributions, including bug reports, feature requests, and pull
          requests.
        </p>
        <div class="flex space-x-4">
          <a
            href="https://github.com/tombi-toml/tombi"
            class="text-blue-600 hover:underline"
            target="_blank"
            rel="noopener noreferrer"
          >
            GitHub Repository
          </a>
          <a
            href="https://github.com/tombi-toml/tombi/issues"
            class="text-blue-600 hover:underline"
            target="_blank"
            rel="noopener noreferrer"
          >
            Issue Tracker
          </a>
        </div>
      </section>

      <section>
        <h2 class="text-2xl font-semibold mb-4">License</h2>
        <p>
          Tombi is released under the MIT License. For more details, please
          check our
          <a
            href="https://github.com/tombi-toml/tombi/blob/main/LICENSE"
            class="text-blue-600 hover:underline"
            target="_blank"
            rel="noopener noreferrer"
          >
            license file
          </a>
          .
        </p>
      </section>
    </main>
  );
}
