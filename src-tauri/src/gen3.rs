use serde::Serialize;

use crate::gen3constants::{
    BLOCK_ORDER, CHARACTER_MAP, SAVE_B_OFFSET, SECTION_COUNT, SECTION_SIZE, TEAM_SECTION_ID,
    TRAINER_SECTION_ID,
};
use std::collections::HashMap;

#[derive(Serialize)]
pub struct Gen3Data {
    pub trainer_nick: String,
    pub team: Vec<Pokemon>,
}

#[derive(Serialize)]
pub struct Stats {
    hp: u8,
    attk: u8,
    s_attk: u8,
    def: u8,
    s_def: u8,
    speed: u8,
    is_egg: bool,
    ability: u8,
}

#[derive(Serialize)]
pub struct Pokemon {
    nick: String,
    species: u16,
    item: u16,
    ev: Stats,
    iv: Stats,
}

pub fn parse_gen_3(bytes: Vec<u8>) -> Gen3Data {
    let save_b = &bytes[SAVE_B_OFFSET..SAVE_B_OFFSET * 2];
    let mut sections: Vec<&[u8]> = Vec::with_capacity(14);
    let mut sections_map: HashMap<u8, &[u8]> = HashMap::new();

    for index in 0..SECTION_COUNT {
        let start = index * SECTION_SIZE;
        sections.push(&save_b[start..start + SECTION_SIZE]);
    }
    //TODO:
    // - Add checksum
    // - Get pokemon species from decrypted data
    for section in sections {
        let section_id = section[0x0FF4];
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

    let team_data = &team_section[0x0238..0x0238 + 600];

    for index in 0..team_size {
        let start = 0 + (100 * index as usize);
        let finish = 100 * (index as usize + 1);
        let pokemon_data = &team_data[start..finish];
        let nickname = parse_name(&pokemon_data, 8, 18);

        //Pokemon data starts at 32 and its 48 length
        let advanced_data = &pokemon_data[32..(32 + 48)];
        let p_id = u32::from_le_bytes(pokemon_data[0..4].try_into().unwrap());
        let ot_id = u32::from_le_bytes(pokemon_data[4..8].try_into().unwrap());

        //Data is encrypted, and the encryption key is an XOR of personality ID and original trainer ID
        let encryption_key = &ot_id ^ &p_id;
        let decrypted_data = decrypt_pokemon_data(advanced_data, encryption_key);

        //Data is stored in 4 chunks of 12, the order depends on the personality ID.
        //Doing the modulo 24 of the personality value we get the order
        let order = BLOCK_ORDER
            .get((p_id % 24) as usize)
            .expect("Error retrieving BlOCK_ORDER");
        let growth_index = order
            .iter()
            .position(|&c| c == 'A')
            .expect("Pokemon data block not found");

        let growth_start = growth_index * 12;
        let growth_data = &decrypted_data[growth_start..(growth_start + 12)];

        let species_id =
            u16::from_le_bytes(growth_data[0..2].try_into().expect("growthData too short"));
        let item_id =
            u16::from_le_bytes(growth_data[2..4].try_into().expect("growthData too short"));

        let evs_index = order
            .iter()
            .position(|&c| c == 'C')
            .expect("Pokemon data block not found");
        let evs_start = evs_index * 12;
        let evs_data = &decrypted_data[evs_start..(evs_start + 12)];

        let misc_index = order
            .iter()
            .position(|&c| c == 'D')
            .expect("Pokemon data block not found");
        let misc_start = misc_index * 12;
        let misc_data = &decrypted_data[misc_start..(misc_start + 12)];

        let iv_data = &misc_data[4..8];
        let iv_value = u32::from_le_bytes(iv_data.try_into().unwrap());

        //bit 30 is 0 or 1 if the pokemon is an egg and bit 31 indicates the ability of the pokemon
        let is_egg = ((iv_value >> 30) & 1) != 0;
        let ability = ((iv_value >> 31) & 1) as u8;

        pokemon_team.push(Pokemon {
            nick: nickname,
            species: species_id,
            item: item_id,
            ev: Stats {
                hp: evs_data[0],
                attk: evs_data[1],
                s_attk: evs_data[2],
                def: evs_data[3],
                s_def: evs_data[4],
                speed: evs_data[5],
                is_egg,
                ability,
            },
            iv: Stats {
                hp: get_bits(iv_value, 0),
                attk: get_bits(iv_value, 5),
                s_attk: get_bits(iv_value, 10),
                def: get_bits(iv_value, 15),
                s_def: get_bits(iv_value, 20),
                speed: get_bits(iv_value, 25),
                is_egg,
                ability,
            },
        })
    }
    pokemon_team
}

//Pokemon data is 48 length
fn decrypt_pokemon_data(data: &[u8], key: u32) -> [u8; 48] {
    let mut decrypted = [0u8; 48];

    for i in (0..48).step_by(4) {
        let value = u32::from_le_bytes(data[i..(i + 4)].try_into().unwrap()) ^ key;
        decrypted[i..i + 4].copy_from_slice(&value.to_le_bytes());
    }
    decrypted
}

fn get_bits(value: u32, offset: u32) -> u8 {
    ((value >> offset) & ((1 << 5) - 1)) as u8
}
