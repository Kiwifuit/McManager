import { MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { Suspense } from "solid-js";
import "./app.css";

export default function App() {
  return (
    <Router
      root={props => (
        <MetaProvider>
          <Title>MCS</Title>
          <nav>
            <ul class="flex gap-4 text-xl">
              <li><a href="/">Home</a></li>
              <li class="grow"><a href="/servers">Servers</a></li>
              <li><a href="/about">About</a></li>
              <li><a href="/options">MCS Options</a></li>
            </ul>
          </nav>
          <Suspense>{props.children}</Suspense>
        </MetaProvider>
      )}
    >
      <FileRoutes />
    </Router>
  );
}
