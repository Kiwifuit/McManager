import { Title } from "@solidjs/meta";
import { useParams } from "@solidjs/router";
import { BiSolidCopyAlt } from "solid-icons/bi";
import { FaSolidUpload } from "solid-icons/fa";
import { For } from "solid-js";
import DashboardNavBar from "~/components/ServerDashboard";
import "./server.css";

export default function Dashboard() {
  const params = useParams();
  const displayName = params.id.replaceAll("-", " ").toUpperCase();
  const logs = Array(100).fill(
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque aliquet euismod maximus. In scelerisque lacus at arcu tempus, quis tempor velit condimentum. Vivamus eu dui lorem. Quisque tellus ex, aliquam id laoreet quis, rhoncus fermentum massa. Nam tortor ex, dapibus eget tincidunt eu, suscipit a mi. Vivamus auctor quis nulla a mollis. Integer sollicitudin massa et imperdiet dictum. Vivamus scelerisque, tellus eget pellentesque sollicitudin, velit purus interdum mi, a mattis mi ex ut ex. Nulla diam mi, scelerisque ut eleifend quis, laoreet vel velit.",
  );

  return (
    <main id="dashboard-ui">
      <Title>{displayName} dashboard</Title>
      <DashboardNavBar displayName={displayName} id={params.id} />
      <div class="col-start-2 mx-4 mt-3 overflow-x-auto">
        <div
          id="log-title"
          class="dark:bg-dark-dashboard-title flex h-9 content-center p-5"
        >
          <h1 class="grow self-center text-lg font-bold">Logs</h1>
          <div class="flex gap-3 self-center">
            <button class="dark:bg-dark-dashboard-button rounded-md p-2">
              <BiSolidCopyAlt />
            </button>
            <button class="dark:bg-dark-dashboard-button rounded-md p-2">
              <FaSolidUpload />
            </button>
          </div>
        </div>
        <div
          id="log-content"
          class="dark:bg-dark-dashboard-body max-h-[500px] overflow-auto text-nowrap p-5 font-mono"
        >
          <For each={logs} fallback={<p>loading...</p>}>
            {(log) => <p class="font-mono">{log}</p>}
          </For>
        </div>
      </div>
    </main>
  );
}
