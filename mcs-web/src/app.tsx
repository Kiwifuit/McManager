import { MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { BiSolidCog, BiSolidHomeAlt2, BiSolidInfoCircle, BiSolidServer } from 'solid-icons/bi';
import { Suspense } from "solid-js";
import "./app.css";

export default function App() {
  return (
    <Router
      root={props => (
        <MetaProvider>
          <Title>MCS</Title>
          <nav class="mx-3 my-1">
            <ul class="flex gap-4 text-xl">
              <li class="flex items-center">
                <BiSolidHomeAlt2 />
                <a href="/">Home</a>
              </li>
              <li class="grow flex items-center">
                <BiSolidServer />
                <a href="/servers">Servers</a>
              </li>
              <li class="flex items-center">
                <BiSolidInfoCircle />
                <a href="/about">About</a>
              </li>
              <li class="flex items-center">
                <BiSolidCog />
                <a href="/options">MCS Options</a>
              </li>
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
