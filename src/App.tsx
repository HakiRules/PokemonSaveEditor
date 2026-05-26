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
    <main className="p-2">
      {gen3Data ?
        <div className="flex flex-col items-center gap-1">
          <div>
            {gen3Data?.trainer?.nick && <label>{gen3Data?.trainer?.nick}</label>}
          </div>
          <TabGroup className="w-full max-w-225" defaultIndex={0}>
            <TabList className=" rounded-md flex justify-between gap-1 px-1 py-1 border border-primary-500">
              <Tab className="w-full data-selected:bg-primary-500 text-black data-hover:bg-primary-500/20 rounded-md">Team</Tab>
              <Tab className="w-full data-selected:bg-primary-500 text-black data-hover:bg-primary-500/20 rounded-md">PC</Tab>
              <Tab className="w-full data-selected:bg-primary-500 text-black data-hover:bg-primary-500/20 rounded-md">Bag</Tab>
              <Tab className="w-full data-selected:bg-primary-500 text-black data-hover:bg-primary-500/20 rounded-md">Pokedex</Tab>
              <Tab className="w-full data-selected:bg-primary-500 text-black data-hover:bg-primary-500/20 rounded-md">Trainer</Tab>
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
