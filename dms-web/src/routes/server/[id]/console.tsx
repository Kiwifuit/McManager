import { Title } from "@solidjs/meta";
import { useParams } from "@solidjs/router";
import { BiRegularNoSignal } from "solid-icons/bi";
import { BsArrowReturnLeft } from "solid-icons/bs";
import { createSignal, For, Match, onCleanup, onMount, Switch } from "solid-js";
import DashboardNavBar from "~/components/ServerDashboard";
import "./server.css";

export default function Dashboard() {
  const params = useParams();
  const displayName = params.id.replaceAll("-", " ").toUpperCase();
  const offline = false;

  const [logs, setLogs] = createSignal<string[]>([]);
  const [currentCommand, setCommand] = createSignal<string>("");

  // Console Content Updater
  const onNewLog = (newLog: string) => {
    setLogs([...logs(), newLog]);
    autoScroll();
  };

  // Command Handler + Input Field
  let consoleCommandInput: HTMLInputElement | undefined;
  const handleCommand = () => {
    console.log(`Command sent: ${currentCommand()}`);
    onNewLog(`> ${currentCommand()}`);
    onNewLog(`Command Response`);
    setCommand("");

    if (consoleCommandInput) {
      consoleCommandInput.value = "";
    }
  };

  // Console content autoscroll
  let consoleContent: HTMLDivElement | undefined;

  const autoScroll = () => {
    if (consoleContent) {
      consoleContent.scrollTop = consoleContent.scrollHeight;
    }
  };

  onMount(() => {
    // Console log updater
    if (typeof MutationObserver !== undefined) {
      const newContentObserver = new MutationObserver(autoScroll);

      if (consoleContent) {
        newContentObserver.observe(consoleContent, {
          childList: true,
        });
      } else {
        console.error("expected consoleContent to be available on this time!");
      }

      onCleanup(() => {
        newContentObserver.disconnect();
      });
    }

    autoScroll();
  });

  return (
    <main id="dashboard-ui">
      <Title>{displayName} dashboard</Title>
      <DashboardNavBar displayName={displayName} id={params.id} />
      <div class="col-start-2 mx-4 mt-3 overflow-x-auto">
        <div
          id="log-title"
          class="flex h-9 content-center bg-light-dashboard-title p-5 dark:bg-dark-dashboard-title"
        >
          <h1 class="grow self-center text-lg font-bold">Console</h1>
        </div>
        <Switch>
          <Match when={!offline}>
            <div
              id="console-content"
              ref={consoleContent}
              class="h-[500px] overflow-y-auto bg-light-dashboard-body p-5 font-mono dark:bg-dark-dashboard-body"
            >
              <For each={logs()} fallback={<p>Fetching logs...</p>}>
                {(log) => <p class="font-mono">{log}</p>}
              </For>
            </div>
            <div id="console-input" class="flex">
              <input
                class="grow bg-light-dashboard-text px-2 py-1 font-mono text-light-fg outline-none placeholder:italic placeholder:text-light-placeholder-text dark:bg-dark-dashboard-text dark:text-dark-fg dark:placeholder:text-dark-placeholder-text"
                type="text"
                placeholder="> say hello world!"
                ref={consoleCommandInput}
                onInput={(e) => {
                  e.preventDefault();

                  setCommand(e.target.value);
                }}
                onKeyPress={(e) => {
                  if (e.key === "Enter") {
                    handleCommand();
                  }
                }}
              />
              <button
                class="bg-light-dashboard-button px-2 py-1 dark:bg-dark-dashboard-button"
                onclick={handleCommand}
                disabled={currentCommand().trim() === ""}
              >
                <BsArrowReturnLeft />
              </button>
            </div>
          </Match>
          <Match when={offline}>
            <div
              id="console-offline-content"
              class="grid h-[500px] overflow-y-auto bg-light-dashboard-body p-5 dark:bg-dark-dashboard-body"
            >
              <p class="m-auto flex flex-row justify-center gap-3 text-6xl">
                <BiRegularNoSignal />
                Offline
              </p>
            </div>
          </Match>
        </Switch>
      </div>
    </main>
  );
}
