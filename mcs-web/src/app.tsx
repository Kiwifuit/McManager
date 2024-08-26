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
              <li><a href="/" class="flex items-center">
                <BiSolidHomeAlt2 />
                Home
              </a></li>
              <li class="grow"><a href="/servers" class="flex items-center">
                <BiSolidServer />
                Servers
              </a></li>
              <li><a href="/about" class="flex items-center">
                <BiSolidInfoCircle />
                About
              </a></li>
              <li><a href="/options" class="flex items-center">
                <BiSolidCog />
                Options
              </a></li>
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
