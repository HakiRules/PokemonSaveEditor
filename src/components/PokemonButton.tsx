import { Pokemon } from "../types/Pokemon"

type PokemonButtonProps = {
  pokemon: Pokemon
  onClick: () => void
}

export const PokemonButton = ({ pokemon, onClick }: PokemonButtonProps) => {

  return (
    <button
      className="cursor-pointer bg-white font-semibold p-2 border border-gray-300 hover:shadow-lg transition-shadow rounded-xl shadow flex flex-col items-center"
      onClick={onClick}
      key={pokemon.nick}>
      <img
        alt={`${pokemon.nick}-${pokemon.species_name}`}
        className="w-16 h-16 flex justify-center align-middle bg-primary-500/20"
        src={`https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/${pokemon.species}.png`}
      />
      <p className="capitalize">
        {pokemon.species_name}
      </p>
      <p>{`"${pokemon.nick}"`}</p>
      <p>Lvl. {pokemon.experience}</p>
    </button>
  )
}