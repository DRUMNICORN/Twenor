// With the Tauri API npm package:
import { invoke } from "@tauri-apps/api/tauri";
// With the Tauri global script:
const invoke = window.__TAURI__.invoke;

document.addEventListener("DOMContentLoaded", () => {
  console.log("Splashscreen closed");
  // This will wait for the window to load, but you could
  // run this function on whatever trigger you want
  invoke("close_splashscreen");
});
