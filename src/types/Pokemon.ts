export type Pokemon = {
  nick: string
  species: Species
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

export type Species = {
  species_id: number
  name: string
  hp: number
  attack: number
  defense: number
  speed: number
  sp_attack: number
  sp_defense: number
  type1: number
  type2: number
  catch_rate: number
  base_exp: number
  ev_hp: number
  ev_attack: number
  ev_defense: number
  ev_speed: number
  ev_sp_attack: number
  ev_sp_defense: number
  gender_ratio: number
  egg_cycles: number
  friendship: number
  growth_rate: number
  egg_group1: number
  egg_group2: number
  ability1: number
  ability2: number
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
