import "prismjs";
import "prismjs/components/prism-toml";

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
      root={(props) => (
        <MetaProvider>
          <Layout>
            <Suspense fallback={<div class="text-center">Loading...</div>}>
              {props.children}
            </Suspense>
          </Layout>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
