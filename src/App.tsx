import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { open } from "@tauri-apps/plugin-dialog";
import { ask } from "@tauri-apps/plugin-dialog";

// Open a dialog

// Prints file path and name to the console
function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [file, setFile] = useState(""); // [path, name, size, type, lastModified
  const [name, setName] = useState("");

  async function greet() {
    // const answer = await ask("This action cannot be reverted. Are you sure?", {
    //   title: "Tauri",
    //   kind: "warning",
    //   // type: "warning",
    // });
    // console.log(answer);

    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  async function pick() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // await invoke("pick");
    // setGreetMsg(await invoke("greet", { name }));
    // setFile(await invoke("pick"));
    invoke("pick");
  }

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        {/* <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a> */}
        {/* <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a> */}
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <button
        onClick={() => {
          console.log("clicked");
          pick();
        }}
      >
        Pick a file
      </button>

      {/* <p>Click on the Tauri, Vite, and React logos to learn more.</p> */}

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          // pick();
          greet();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button type="submit">Greet</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
