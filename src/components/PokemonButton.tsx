import { Pokemon } from "../types/Pokemon"

type PokemonButtonProps = {
  pokemon: Pokemon
  onClick: () => void
}

const natures = [
  "Hardy",
  "Lonely",
  "Brave",
  "Adamant",
  "Naughty",
  "Bold",
  "Docile",
  "Relaxed",
  "Impish",
  "Lax",
  "Timid",
  "Hasty",
  "Serious",
  "Jolly",
  "Naive",
  "Modest",
  "Mild",
  "Quiet",
  "Bashful",
  "Rash",
  "Calm",
  "Gentle",
  "Sassy",
  "Careful",
  "Quirk",
]

export const PokemonButton = ({ pokemon, onClick }: PokemonButtonProps) => {

  return (
    <button
      className="cursor-pointer bg-white font-semibold p-2 border border-gray-300 hover:shadow-lg transition-shadow rounded-xl shadow flex flex-col items-center"
      onClick={onClick}
      key={pokemon.nick}>
      <div className="flex justify-start gap-4 w-full overflow-auto">
        <img
          alt={`${pokemon.nick}-${pokemon.species_name}`}
          src={`https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/${pokemon.species}.png`}
          className="w-24 h-24 pixelated"
        />
        <div className="">
          <h2 className="text-2xl">
            {pokemon.species_name}
          </h2>
          <h3 className="text-gray-500">
            &quot;{pokemon.nick}&quot;
          </h3>
          <p>{natures[pokemon.personality_id % 25]}</p>
          <p>Lv. {pokemon.experience}</p>
        </div>
      </div>
    </button>
  )
}