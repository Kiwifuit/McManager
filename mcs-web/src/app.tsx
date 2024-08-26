import { MetaProvider, Title } from "@solidjs/meta";
import { Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { BiSolidCog, BiSolidHomeAlt2, BiSolidInfoCircle } from 'solid-icons/bi';
import { FiMoon } from 'solid-icons/fi';
import { Suspense } from "solid-js";
import "./app.css";

export default function App() {
  return (
    <Router
      root={props => (
        <MetaProvider>
          <Title>MCS</Title>
          <nav>
            <ul class="flex gap-4 text-xl px-5 py-3 dark:bg-dark-accent bg-light-accent">
              <li class="grow"><a href="/" class="flex items-center w-fit">
                <BiSolidHomeAlt2 />
                Home
              </a></li>
              <li><a href="/about" class="flex items-center">
                <BiSolidInfoCircle />
                About
              </a></li>
              <li><a href="/options" class="flex items-center">
                <BiSolidCog />
                Options
              </a></li>
              <li><button>
                <FiMoon />
                {/* <FiSun /> */}
              </button></li>
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
