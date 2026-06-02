import { Button, Dialog, DialogBackdrop, DialogPanel, DialogTitle } from "@headlessui/react"
import { Pokemon } from "../types/Pokemon"
import CloseIcon from "../assets/close_icon.svg"

type PokemonDetailsProps = {
  pokemon: Pokemon
  onClose: () => void
}
export const PokemonDetails = ({ pokemon, onClose }: PokemonDetailsProps) => {

  return (
    <Dialog
      open
      onClose={onClose}
      className="relative z-50"
    >
      <DialogBackdrop className="fixed inset-0 bg-black/30" />
      <div className="fixed inset-0 flex w-screen items-center justify-center p-4 ">
        <DialogPanel className="w-full max-w-3xl max-h-[90vh] overflow-y-auto p-6 rounded-xl border border-gray-300 bg-white">
          <DialogTitle className="flex items-start gap-4 flex-1">
            <img
              alt={`${pokemon.nick}-${pokemon.species.name}`}
              src={`https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/${pokemon.species}.png`}
              className="w-24 h-24 pixelated"
            />
            <div className="flex-1 items-start">
              <h4 className="text-2xl">
                {pokemon.species.name}
              </h4>
              <h5 className="text-gray-500">
                &quot;{pokemon.nick}&quot;
              </h5>
            </div>
            <Button
              className="hover:bg-[#e9ebef] hover:text-[#e9ebef]-foreground dark:hover:bg-[#e9ebef]/50 size-9 rounded-full flex items-center justify-center"
              onClick={onClose}>
              <img
                className="w-4 h-4"
                src={CloseIcon}
              />
            </Button>
          </DialogTitle>
        </DialogPanel>
      </div>
    </Dialog>
  )
}