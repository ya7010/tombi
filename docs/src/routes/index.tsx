import { Title } from "@solidjs/meta";

export default function Home() {
  return (
    <main class="p-8 max-w-4xl mx-auto">
      <Title>Tombi - TOML Language Server</Title>

      <h1 class="text-4xl font-bold mb-6">Tombi</h1>

      <p class="mb-4">
        Tombi is a high-performance language server for TOML files. It works
        with any editor that supports the Language Server Protocol (LSP),
        including VS Code and Neovim.
      </p>

      <h2 class="text-2xl font-semibold mt-8 mb-4">Key Features</h2>
      <ul class="list-disc pl-6 space-y-2">
        <li>TOML syntax checking and validation</li>
        <li>Schema-based completion</li>
        <li>Type information on hover</li>
        <li>Error diagnostics and quick fixes</li>
      </ul>

      <h2 class="text-2xl font-semibold mt-8 mb-4">Getting Started</h2>
      <p class="mb-4">
        For detailed installation and configuration instructions, please visit
        our{" "}
        <a href="/docs/getting-started" class="text-blue-600 hover:underline">
          Getting Started Guide
        </a>
        .
      </p>

      <div class="mt-8 p-4 bg-gray-100 rounded-lg">
        <h3 class="text-xl font-semibold mb-2">Quick Start</h3>
        <pre class="bg-gray-800 text-white p-4 rounded overflow-x-auto">
          <code>
            # Install VS Code Extension code --install-extension
            tombi.tombi-vscode # Or install Language Server directly cargo
            install tombi-lsp
          </code>
        </pre>
      </div>
    </main>
  );
}
