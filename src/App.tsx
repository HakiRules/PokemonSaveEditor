import { ChangeEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { Gen3Data } from "./types/Gen3";
import { Tab, TabGroup, TabList, TabPanel, TabPanels } from "@headlessui/react";
import { TeamView } from "./components/TeamView";

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
    <main className=" min-h-screen p-2 bg-linear-to-br from-blue-50 to-indigo-100 ">
      {gen3Data ?
        <div className="flex flex-col items-center gap-1">
          <div className="my-6">
            {gen3Data?.trainer?.nick && <label>{gen3Data?.trainer?.nick}</label>}
          </div>
          <TabGroup className="w-full max-w-225 " defaultIndex={0}>
            <TabList className="bg-muted rounded-md flex justify-between gap-1 px-1 py-1 ">
              <Tab className="w-full data-selected:bg-white text-black data-hover:bg-white/50 rounded-md">Team</Tab>
              <Tab className="w-full data-selected:bg-white text-black data-hover:bg-white/50 rounded-md">PC</Tab>
              <Tab className="w-full data-selected:bg-white text-black data-hover:bg-white/50 rounded-md">Bag</Tab>
              <Tab className="w-full data-selected:bg-white text-black data-hover:bg-white/50 rounded-md">Pokedex</Tab>
              <Tab className="w-full data-selected:bg-white text-black data-hover:bg-white/50 rounded-md">Trainer</Tab>
            </TabList>
            <TabPanels className="py-2">
              <TabPanel>
                <TeamView team={gen3Data.team} />
              </TabPanel>
              <TabPanel>
                Content2
              </TabPanel>
              <TabPanel>
                Content3
              </TabPanel>
              <TabPanel>
                Content4
              </TabPanel>
              <TabPanel>
                Content5
              </TabPanel>
            </TabPanels>
          </TabGroup>
        </div>
        :
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
      }
    </main >
  );
}

export default App;
