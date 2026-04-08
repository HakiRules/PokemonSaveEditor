import { Player } from "./Player"
import { Pokemon } from "./Pokemon"

export type Gen3Data = {
  trainer?: Player
  team: Pokemon[]
}
