import { Title } from "@solidjs/meta";
import { useParams } from "@solidjs/router";
import { BiSolidCopyAlt } from "solid-icons/bi";
import { FaSolidUpload } from "solid-icons/fa";
import { Accessor, createSignal, For, onCleanup, onMount } from "solid-js";
import DashboardNavBar from "~/components/ServerDashboard";
import "./server.css";

type McLogsSucess = {
  success: true;
  id: string;
  url: string;
  raw: string;
};

type McLogsError = {
  success: false;
  error: string;
};

export default function Dashboard() {
  const params = useParams();
  const displayName = params.id.replaceAll("-", " ").toUpperCase();
  const [logs, setLogs] = createSignal<string[]>([]);

  // Console Content Updater
  const onNewLog = (newLog: string) => {
    setLogs([...logs(), newLog]);
    autoScroll();
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

      // Dev stuff
      setTimeout(() => {
        for (let i = 0; i <= 30; i++) {
          onNewLog(`dev log: ${i}`);
        }
      }, 500);
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
          <h1 class="grow self-center text-lg font-bold">Logs</h1>
          <div class="flex gap-3 self-center">
            <button class="rounded-md bg-light-dashboard-button p-2 dark:bg-dark-dashboard-button">
              <BiSolidCopyAlt />
            </button>
            <button
              class="rounded-md bg-light-dashboard-button p-2 dark:bg-dark-dashboard-button"
              onClick={() => uploadLogs(logs)}
            >
              <FaSolidUpload />
            </button>
          </div>
        </div>
        <div
          id="log-content"
          ref={consoleContent}
          class="max-h-[500px] overflow-auto text-nowrap bg-light-dashboard-body p-5 font-mono dark:bg-dark-dashboard-body"
        >
          <table class="table-auto">
            <For each={logs()} fallback={<p>loading...</p>}>
              {(log, index) => <LogRow lineno={index()} log_line={log} />}
            </For>
          </table>
        </div>
      </div>
    </main>
  );
}

function LogRow(props: { log_line: string; lineno: number }) {
  return (
    <tr class="whitespace-pre">
      <td class="w-12 select-none pr-4 text-right font-mono">
        {props.lineno + 1}
      </td>
      <td class="font-mono">{props.log_line}</td>
    </tr>
  );
}

async function uploadLogs(logs: Accessor<string[]>) {
  let log_content = logs().join("\n");
  let log_line_count = logs().length;

  let response = await fetch("https://api.mclo.gs/1/log", {
    method: "POST",
    headers: {
      "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
    },
    body: new URLSearchParams({
      content: log_content,
    }),
  });

  // This should only happen if the server's wifi is cut off
  if (!response.body) {
    alert("Failed to upload logs");
    return;
  }

  // let response_body = new TextDecoder("utf-8").decode();
  let uploaded_log: McLogsSucess | McLogsError = JSON.parse(
    await response.text(),
  );

  if (uploaded_log.success) {
    let mclogs_url = uploaded_log.url;

    alert(`[OK] Log file: ${mclogs_url}`);
  } else {
    alert(`Error while uploading logs: ${uploaded_log.error}`);
  }
}
