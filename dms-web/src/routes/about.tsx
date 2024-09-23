import { Title } from "@solidjs/meta";
import { getBackendVersion } from "~/rust/ffi";

export default function Test() {
  return (
    <main>
      <Title>This should be visible if the component is rendered</Title>
      <p>Test {getVersionDummy()}</p>
    </main>
  );
}

function getVersionDummy(): string {
  console.log("this runs");
  return getBackendVersion();
}
