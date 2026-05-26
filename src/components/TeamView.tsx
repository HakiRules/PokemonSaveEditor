import { useState } from "react"
import { Pokemon } from "../types/Pokemon"
import { PokemonButton } from "./PokemonButton"
import { PokemonDetails } from "./PokemonDetails"

type TeamViewProps = {
  team: Pokemon[]
}

export const TeamView = ({ team }: TeamViewProps) => {
  const [selectedPoke, setSelectedPoke] = useState<Pokemon>()

  return (
    <div className="grid grid-cols-2 gap-2">
      {team.map(itm =>
        <PokemonButton
          key={itm.nick}
          pokemon={itm}
          onClick={() => setSelectedPoke(itm)}
        />
      )}
      {selectedPoke &&
        <PokemonDetails
          pokemon={selectedPoke}
          onClose={() => setSelectedPoke(undefined)}
        />
      }
    </div>
  )
}