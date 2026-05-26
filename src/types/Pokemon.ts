export type Pokemon = {
  nick: string
  species: number
  species_name: string
  item: number
  ev: Stats
  iv: Stats
  experience: number
  friendship: number
  moves: Array<Move>
  contest: ContestData
  pokerus: number
  met_location: number
  personality_id: number
}

export type Stats = {
  hp: number
  attk: number
  s_attk: number
  def: number
  s_def: number
  speed: number
  is_egg: boolean
  ability: number
}

export type Move = {
  move_id: number
  current_pp: number
  pp_bonus: number
}

export type ContestData = {
  coolness: number
  beauty: number
  cuteness: number
  smartness: number
  toughness: number
  feel: number
}
