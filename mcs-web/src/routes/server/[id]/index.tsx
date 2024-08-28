import { Title } from "@solidjs/meta";
import { useParams } from "@solidjs/router";

export default function Dashboard() {
  const params = useParams();

  return (
    <main>
      <Title>{params.id} dashboard</Title>
      <h1 class="text-4xl font-bold">Dashboard</h1>
    </main>
  );
}
