import { Title } from "@solidjs/meta";
import { useParams } from "@solidjs/router";
import DashboardNavBar from "~/components/ServerDashboard";
import "./server.css";

export default function Dashboard() {
  const params = useParams();
  const displayName = params.id.replaceAll("-", " ").toUpperCase();

  return (
    <main id="dashboard-ui">
      <Title>{displayName} dashboard</Title>
      <DashboardNavBar name={displayName} />
      <div class="col-start-2 ml-5 mt-3">
        <h1 class="text-4xl font-bold">Dashboard</h1>
      </div>
    </main>
  );
}
