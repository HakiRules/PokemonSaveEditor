import { Pokemon } from "../types/Pokemon"

type PokemonButtonProps = {
  pokemon: Pokemon
}

export const PokemonButton = ({ pokemon }: PokemonButtonProps) => {

  const onButtonClick = (poke: string) => {
    console.log(poke)
  }

  return (
    <button
      className="font-semibold p-2 border border-gray-400 hover:border-gray-700 rounded shadow flex flex-col items-center"
      onClick={() => onButtonClick(pokemon.nick)}
      key={pokemon.nick}>
      <div className="w-16 h-16 flex justify-center align-middle">
        <img
          src={`https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/${pokemon.species}.png`}
        />
      </div>
      <p className="capitalize">
        {pokemon.species_name}
      </p>
      <p>{`"${pokemon.nick}"`}</p>
      <p>Lvl. {pokemon.experience}</p>
    </button>
  )
}