import { MetaProvider, Title } from "@solidjs/meta";
import { A, Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { BiSolidCog, BiSolidHomeAlt2, BiSolidInfoCircle } from 'solid-icons/bi';
import { FiMoon, FiSun } from 'solid-icons/fi';
import { createSignal, Suspense } from "solid-js";
import "./app.css";

export default function App() {
  let [lightMode, setLightMode] = createSignal(true)

  const toggleDarkMode = () => {
    setLightMode(!lightMode())
  }

  return (
    <Router
      root={props => (
        <MetaProvider>
          <Title>MCS</Title>
          <nav>
            <ul class="flex gap-4 text-xl px-5 py-3 dark:bg-dark-accent bg-light-accent">
              <li class="grow"><A href="/" class="flex items-center w-fit">
                <BiSolidHomeAlt2 />
                Home
              </A></li>
              <li><A href="/about" class="flex items-center">
                <BiSolidInfoCircle />
                About
              </A></li>
              <li><A href="/options" class="flex items-center">
                <BiSolidCog />
                Options
              </A></li>
              <li>
                <button onclick={toggleDarkMode}>
                  {
                    lightMode() ?
                      (<FiMoon />)
                      : (<FiSun />)
                  }
                </button>
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
