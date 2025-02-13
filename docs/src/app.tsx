import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { Suspense } from "solid-js";
import { MetaProvider } from "@solidjs/meta";
import "virtual:uno.css";
import "./app.css";

export default function App() {
  return (
    <Router
      root={(props) => (
        <MetaProvider>
          <main class="min-h-screen p-4">
            <nav class="flex gap-4 mb-8">
              <a href="/" class="nav-link">Index</a>
              <a href="/about" class="nav-link">About</a>
              <a href="/documentation" class="nav-link">Documentation</a>
            </nav>
            <div class="container mx-auto">
              <Suspense fallback={<div class="text-center">Loading...</div>}>
                {props.children}
              </Suspense>
            </div>
          </main>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
