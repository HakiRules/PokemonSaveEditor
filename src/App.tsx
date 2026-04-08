import { ChangeEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { Gen3Data } from "./types/Gen3";
import { PokemonButton } from "./components/PokemonButton";

function App() {

  const [gen3Data, setGen3Data] = useState<Gen3Data>()

  const handleFile = async (event: ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0]
    if (!file) return

    const buffer = await file.arrayBuffer()
    const bytes = new Uint8Array(buffer)

    const result = await invoke<Gen3Data>("open_file", { bytes })
    console.log(result)
    if (result) setGen3Data(result)
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
      <div>
        <div className="flex justify-center">
          {gen3Data?.trainer?.nick && <label>{gen3Data?.trainer?.nick}</label>}
        </div>
        <div className="flex gap-1 justify-center">
          {gen3Data?.team?.map(itm =>
            <PokemonButton pokemon={itm} key={itm.nick} />
          )}
        </div>
      </div>
    </main>
  );
}

export default App;
