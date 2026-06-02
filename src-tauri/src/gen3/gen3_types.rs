use serde::Serialize;

use crate::species_types::SpeciesInfo;

#[derive(Serialize)]
pub struct Gen3Data {
    pub trainer: Option<Player>,
    pub team: Vec<Pokemon>,
}

#[derive(Serialize)]
pub struct Player {
    pub nick: String,
    pub gender: bool,
    pub money: u32,
    pub trainer_id: u16,
    pub secret_id: u16,
    pub time_played: u32,
    pub badges: u32,
    pub security_key: u32,
}

#[derive(Serialize)]
pub struct Stats {
    pub hp: u8,
    pub attk: u8,
    pub s_attk: u8,
    pub def: u8,
    pub s_def: u8,
    pub speed: u8,
    pub ability: u8,
}

#[derive(Serialize)]
pub struct Move {
    pub move_id: u16,
    pub current_pp: u8,
    pub pp_bonus: u8,
}

#[derive(Serialize)]
pub struct ContestData {
    pub coolness: u8,
    pub beauty: u8,
    pub cuteness: u8,
    pub smartness: u8,
    pub toughness: u8,
    pub feel: u8,
}

#[derive(Serialize)]
pub struct Pokemon {
    pub nick: String,
    pub species: SpeciesInfo,
    pub item: u16,
    pub ev: Stats,
    pub iv: Stats,
    pub experience: u32,
    pub friendship: u8,
    pub moves: Vec<Move>,
    pub contest: ContestData,
    pub pokerus: u8,
    pub met_location: u8,
    pub personality_id: u32,
    pub is_egg: bool,
}
