use serde::Serialize;

use crate::gen3constants::{
    CHARACTER_MAP, SAVE_B_OFFSET, SECTION_COUNT, SECTION_SIZE, TEAM_SECTION_ID, TRAINER_SECTION_ID,
};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Gen3Data {
    pub trainer_nick: String,
    pub team: Vec<Pokemon>,
}

#[derive(Serialize)]
pub struct Pokemon {
    nick: String,
    species: String,
}

pub fn parse_gen_3(bytes: Vec<u8>) -> Gen3Data {
    let save_b = &bytes[SAVE_B_OFFSET..SAVE_B_OFFSET * 2];
    let mut sections: Vec<&[u8]> = Vec::with_capacity(14);
    let mut sections_map: HashMap<u16, &[u8]> = HashMap::new();

    for index in 0..SECTION_COUNT {
        let start = index * SECTION_SIZE;
        sections.push(&save_b[start..start + SECTION_SIZE]);
    }
    //TODO:
    // - Add checksum

    for section in sections {
        let section_id = u16::from_le_bytes(section[0x0FF4..0x0FF6].try_into().unwrap());
        sections_map.insert(section_id, section);
    }

    let trainer_nick = parse_name(sections_map.get(&TRAINER_SECTION_ID).unwrap(), 0, 7);
    let team = parse_pokemon_team(sections_map.get(&TEAM_SECTION_ID).unwrap());
    Gen3Data { trainer_nick, team }
}

fn parse_name(section: &[u8], start: usize, size: usize) -> String {
    let mut name = String::new();
    for index in start..size {
        let byte = match section.get(index) {
            Some(&b) => b,
            None => 0,
        };
        if byte == 0xFF {
            break;
        }

        let character = CHARACTER_MAP[byte as usize];
        name.push(character);
    }
    name
}

fn parse_pokemon_team(team_section: &[u8]) -> Vec<Pokemon> {
    let mut pokemon_team: Vec<Pokemon> = Vec::new();
    let team_size = team_section[0x0234] as u8;
    println!("Team size: {}", team_size);

    let team_data = &team_section[0x0238..0x0238 + 600];

    for index in 0..team_size {
        let start = 0 + (100 * index as usize);
        let finish = 100 * (index as usize + 1);
        let pokemon_data = &team_data[start..finish];
        let nickname = parse_name(&pokemon_data, 8, 18);
        pokemon_team.push(Pokemon {
            nick: nickname,
            species: String::new(),
        })
    }
    pokemon_team
}
