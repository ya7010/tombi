import "prismjs";
import "prismjs/components/prism-toml";
import "prismjs/components/prism-bash";

import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { Suspense } from "solid-js";
import { MetaProvider } from "@solidjs/meta";
import "virtual:uno.css";
import "./app.css";
import Layout from "./routes/layout";

export default function App() {
  return (
    <Router
      base={import.meta.env.BASE_URL}
      root={(props) => (
        <MetaProvider>
          <Layout>
            <main class="flex-1 mt-20 pt-0">
              <div class="max-w-7xl mx-auto sm:px-6 lg:px-8">
                <Suspense fallback={<div class="text-center">Loading...</div>}>
                  {props.children}
                </Suspense>
              </div>
            </main>
          </Layout>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
