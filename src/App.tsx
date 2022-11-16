import { createSignal } from "solid-js";
import logo from "./assets/logo.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

listen("auth_received", (data) => {
  console.log("Auth received: ", data);
});

function App() {
  return (
    <div class="container">
      <h1>Welcome to Tauri!</h1>

      <div class="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://solidjs.com" target="_blank">
          <img src={logo} class="logo solid" alt="Solid logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and Solid logos to learn more.</p>

      <div class="row">
        <div>
          <button
            type="button"
            onClick={() =>
              invoke("open_in_browser", {
                url: "https://spacedrive.com",
              })
            }
          >
            Open page in browser
          </button>
          <button type="button" onClick={() => invoke("start_server")}>
            Start localhost server
          </button>
        </div>
      </div>
    </div>
  );
}

export default App;
