// @refresh reload
import { mount, StartClient } from "@solidjs/start/client";

console.log("Hello")
mount(() => <StartClient />, document.getElementById("app")!);