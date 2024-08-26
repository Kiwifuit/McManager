import { Title } from "@solidjs/meta";
import { HttpStatusCode } from "@solidjs/start";

export default function NotFound() {
  return (
    <main class="grid place-content-center">
      <Title>Not Found</Title>
      <HttpStatusCode code={404} />
      <h1 class="text-3xl font-bold">Page Not Found</h1>
      <p class="ml-3 max-w-xl">
        This could mean that the URL you typed was wrong. If this shouldn't
        be the case, contact the developers of the website immediately
      </p>
    </main>
  );
}
