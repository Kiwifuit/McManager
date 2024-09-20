import { Title } from "@solidjs/meta";
import { useParams } from "@solidjs/router";
import { AiOutlineEnter } from "solid-icons/ai";
import { createSignal, For, onCleanup, onMount } from "solid-js";
import DashboardNavBar from "~/components/ServerDashboard";
import "./server.css";

export default function Dashboard() {
  const params = useParams();
  const displayName = params.id.replaceAll("-", " ").toUpperCase();
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
    onNewLog(`New command: ${currentCommand()}`);
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
          class="bg-light-dashboard-title flex h-9 content-center p-5 dark:bg-dark-dashboard-title"
        >
          <h1 class="grow self-center text-lg font-bold">Console</h1>
        </div>
        <div
          id="console-content"
          ref={consoleContent}
          class="bg-light-dashboard-body h-[500px] overflow-y-auto p-5 font-mono dark:bg-dark-dashboard-body"
        >
          <For each={logs()} fallback={<p>Fetching logs...</p>}>
            {(log) => <p class="font-mono">{log}</p>}
          </For>
        </div>
        <div id="console-input" class="flex">
          <input
            class="bg-light-dashboard-text placeholder:text-light-placeholder-text dark:placeholder:text-dark-placeholder-text grow px-2 py-1 text-light-fg outline-none placeholder:italic dark:bg-dark-dashboard-text dark:text-dark-fg"
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
            <AiOutlineEnter />
          </button>
        </div>
      </div>
    </main>
  );
}
