import { ChangeEvent } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {


  const handleFile = async (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (!file) return
    console.log(file)
    const buffer = await file.arrayBuffer()
    const bytes = new Uint8Array(buffer)

    console.log("File size:", bytes.length)
    console.log(bytes)
    const result = await invoke("open_file", { bytes })
    console.log("Result", result)
  }

  return (
    <main className="p-2">
      <label className="flex w-fit p-2 border border-blue-600 hover:cursor-pointer hover:bg-[#242424]">
        Browse file
        <input
          id="file-input"
          hidden
          onChange={handleFile}
          type="file"
          accept=".sav"
        />
      </label>
    </main>
  );
}

export default App;
