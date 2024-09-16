import { Title } from "@solidjs/meta";
import { useParams } from "@solidjs/router";
import { createSignal, For, onCleanup, onMount } from "solid-js";
import DashboardNavBar from "~/components/ServerDashboard";
import "./server.css";

export default function Dashboard() {
  const params = useParams();
  const displayName = params.id.replaceAll("-", " ").toUpperCase();
  // Initial console logs
  // should be empty during 1.0
  const [logs, setLogs] = createSignal<string[]>([]);

  // Console content autoscroll
  let consoleContent: HTMLDivElement | undefined;

  const autoScroll = () => {
    if (consoleContent) {
      consoleContent.scrollTop = consoleContent.scrollHeight;
    }
  };

  const onNewLog = (newLog: string) => {
    setLogs([...logs(), newLog]);
    autoScroll();
  };

  let dev_intervalid: NodeJS.Timeout;

  onMount(() => {
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
        clearInterval(dev_intervalid);
      });
    }

    autoScroll();
  });

  // pre-1.0 only
  let counter = 0;
  dev_intervalid = setInterval(() => {
    onNewLog(`new log line ${counter}`);
    counter++;
  }, 1000);

  return (
    <main id="dashboard-ui">
      <Title>{displayName} dashboard</Title>
      <DashboardNavBar displayName={displayName} id={params.id} />
      <div class="col-start-2 mx-4 mt-3 overflow-x-auto">
        <div
          id="log-title"
          class="flex h-9 content-center p-5 dark:bg-dark-dashboard-title"
        >
          <h1 class="grow self-center text-lg font-bold">Console</h1>
        </div>
        <div
          id="console-content"
          ref={consoleContent}
          class="h-[500px] overflow-y-auto p-5 font-mono dark:bg-dark-dashboard-body"
        >
          <For each={logs()} fallback={<p>Fetching logs...</p>}>
            {(log) => <p class="font-mono">{log}</p>}
          </For>
        </div>
      </div>
    </main>
  );
}
