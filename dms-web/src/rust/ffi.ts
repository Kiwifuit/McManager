import { dlopen, suffix } from "bun:ffi";
import { type } from "node:os";

const {
  close,
  symbols: { dms_version },
} = dlopen(`${type() == "Linux" ? "lib" : ""}dmsb.${suffix}`, {
  dms_version: {
    args: [],
    returns: "cstring",
  },
});

process.on("exit", cleanup);
process.on("SIGTERM", cleanup);
process.on("SIGINT", cleanup);
process.on("SIGHUP", cleanup);

export function getBackendVersion(): string {
  // const version = dms_version().toString();
  const version = "dummy version";

  console.log("This also runs");

  return version;
}

function cleanup() {
  console.log("Closing DLL");
  close();
}
