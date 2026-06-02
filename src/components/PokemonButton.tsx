import { Pokemon } from "../types/Pokemon"

type PokemonButtonProps = {
  pokemon: Pokemon
  onClick: () => void
}

/* const natures = [
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
] */

export const PokemonButton = ({ pokemon, onClick }: PokemonButtonProps) => {

  return (
    <button
      className="cursor-pointer bg-white font-semibold p-6 border border-gray-300 hover:shadow-lg transition-shadow rounded-xl shadow flex flex-col items-center"
      onClick={onClick}
      key={pokemon.nick}>
      <div className="flex items-start justify-between gap-3 w-full overflow-auto">
        <div className="flex flex-col items-start">
          <h4 className="text-base capitalize">
            {pokemon.species.name}
          </h4>
          <h3 className="text-gray-500">
            &quot;{pokemon.nick}&quot;
          </h3>
          <p className="text-sm text-muted-foreground">Lv. {pokemon.experience}</p>
        </div>
        <img
          alt={`${pokemon.nick}-${pokemon.species.name}`}
          src={`https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/${pokemon.species.species_id}.png`}
          className="w-24 h-24 pixelated"
        />
      </div>
    </button>
  )
}