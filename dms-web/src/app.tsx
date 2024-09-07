import { MetaProvider, Title } from "@solidjs/meta";
import { A, Router } from "@solidjs/router";
import { FileRoutes } from "@solidjs/start/router";
import { BiSolidCog, BiSolidHomeAlt2, BiSolidInfoCircle } from "solid-icons/bi";
import { FiMoon, FiSun } from "solid-icons/fi";
import { createEffect, Suspense } from "solid-js";
import { createStore } from "solid-js/store";
import Cookies from "universal-cookie";
import "./app.css";

const APP_STATE_NAME = "AppState";
const cookieJar = new Cookies();

export default function App() {
  const cookies = cookieJar.get(APP_STATE_NAME);
  const [appState, setAppState] = createStore<{ isDarkMode: boolean }>(
    cookies || { isDarkMode: true },
  );

  createEffect(() => {
    if (appState.isDarkMode) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }

    const expiry = new Date();
    expiry.setFullYear(expiry.getFullYear() + 20);

    cookieJar.set(APP_STATE_NAME, appState, {
      sameSite: "strict",
      expires: expiry,
    });
  });

  const toggleDarkMode = () => {
    const new_state = {
      isDarkMode: !appState.isDarkMode,
    };

    setAppState(new_state);
  };

  return (
    <Router
      root={(props) => (
        <MetaProvider>
          <Title>MCS</Title>
          <nav>
            <ul class="flex gap-4 bg-light-accent px-5 py-3 text-xl dark:bg-dark-accent">
              <li class="grow">
                <A href="/" class="flex w-fit items-center">
                  <BiSolidHomeAlt2 />
                  Home
                </A>
              </li>
              <li>
                <A href="/about" class="flex items-center">
                  <BiSolidInfoCircle />
                  About
                </A>
              </li>
              <li>
                <A href="/options" class="flex items-center">
                  <BiSolidCog />
                  Options
                </A>
              </li>
              <li>
                <button onclick={toggleDarkMode}>
                  {appState.isDarkMode ? <FiSun /> : <FiMoon />}
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
