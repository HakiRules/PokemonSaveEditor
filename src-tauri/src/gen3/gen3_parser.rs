use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

use super::gen3_types::{ContestData, Gen3Data, Move, Player, Pokemon, Stats};
use super::gen3constants::{
    BLOCK_ORDER, CHARACTER_MAP, SAVE_B_OFFSET, SECTION_COUNT, SECTION_SIZE, TEAM_SECTION_ID,
    TRAINER_SECTION_ID,
};
use crate::species_converter::get_national3;
use crate::species_types::SpeciesInfo;

pub fn parse_gen_3(bytes: Vec<u8>) -> Gen3Data {
    let save_b = &bytes[SAVE_B_OFFSET..SAVE_B_OFFSET * 2];
    let mut sections: Vec<&[u8]> = Vec::with_capacity(14);
    let mut sections_map: HashMap<u8, &[u8]> = HashMap::new();

    for index in 0..SECTION_COUNT {
        let start = index * SECTION_SIZE;
        sections.push(&save_b[start..start + SECTION_SIZE]);
    }

    for section in sections {
        let section_id = section[0x0FF4];
        sections_map.insert(section_id, section);
    }

    let trainer_section = sections_map.get(&TRAINER_SECTION_ID).unwrap();
    let mut player = parse_trainer_info(trainer_section);

    let file = File::open(Path::new("./species.json")).expect("Error reading file");
    let species_data: Vec<SpeciesInfo> =
        serde_json::from_reader(file).expect("Error while parsing the file");

    let team: Vec<Pokemon> =
        parse_pokemon_team(sections_map.get(&TEAM_SECTION_ID).unwrap(), &species_data);
    let money = parse_money(
        sections_map.get(&TEAM_SECTION_ID).unwrap(),
        player.security_key,
    );
    player.money = money;

    let badges = trainer_section[21];
    player.badges = badges.count_ones();

    Gen3Data {
        trainer: Some(player),
        team,
    }
}

fn parse_money(section: &[u8], security_key: u32) -> u32 {
    let money_section = u32::from_le_bytes(section[1936..(1936 + 4)].try_into().unwrap());
    money_section ^ security_key
}

fn parse_trainer_info(section: &[u8]) -> Player {
    let trainer_nick = parse_name(section, 0, 7);
    let gender = *section.get(8).unwrap() == 0;
    let trainer_id = u16::from_le_bytes(section[10..12].try_into().unwrap());
    let secret_id = u16::from_le_bytes(section[12..14].try_into().unwrap());
    let security_key = u32::from_le_bytes(section[172..176].try_into().unwrap());

    let hours = u16::from_le_bytes(section[14..16].try_into().unwrap());
    let minutes = section[16];
    let seconds = section[17];

    Player {
        nick: trainer_nick,
        gender,
        money: 0,
        trainer_id,
        secret_id,
        time_played: (hours as u32 * 3600) + (minutes as u32 * 60) + seconds as u32,
        badges: 0,
        security_key,
    }
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

fn parse_pokemon_team(team_section: &[u8], species_data: &[SpeciesInfo]) -> Vec<Pokemon> {
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

        let mut moves: Vec<Move> = Vec::with_capacity(4);

        // || GROWTH DATA EXTRACT ||
        let growth_index = order
            .iter()
            .position(|&c| c == 'A')
            .expect("Pokemon data block not found");

        let growth_start = growth_index * 12;
        let growth_data = &decrypted_data[growth_start..(growth_start + 12)];

        let species_id =
            u16::from_le_bytes(growth_data[0..2].try_into().expect("growth data too short"));
        let item_id =
            u16::from_le_bytes(growth_data[2..4].try_into().expect("growth data too short"));
        let experience = u32::from_le_bytes(growth_data[4..8].try_into().unwrap());
        let pp_bonuses = growth_data[8];

        for i in (0..6).step_by(2) {
            moves.push(Move {
                move_id: 0,
                current_pp: 0,
                pp_bonus: get_bits_from_u8(pp_bonuses, i, 2),
            });
        }
        let friendship = growth_data[9];

        // || ATTACKS DATA EXTRACT ||
        let attacks_index = order
            .iter()
            .position(|&c| c == 'B')
            .expect("Pokemon data block not found");
        let attacks_start = attacks_index * 12;
        let attacks_data = &decrypted_data[attacks_start..(attacks_start + 12)];
        for i in 0..3 {
            let start = i * 2;
            let end = start + 2;
            let move_id = u16::from_le_bytes(
                attacks_data[start..end]
                    .try_into()
                    .expect("attack data too short"),
            );
            let move_pp = attacks_data[8 + i];
            let poke_move = moves.get_mut(i);
            match poke_move {
                Some(item) => {
                    item.move_id = move_id;
                    item.current_pp = move_pp;
                }
                None => println!("Error parsing move number {}", i),
            }
        }

        // || EVS & CONDITION DATA EXTRACT ||
        let evs_index = order
            .iter()
            .position(|&c| c == 'C')
            .expect("Pokemon data block not found");
        let evs_start = evs_index * 12;
        let evs_data = &decrypted_data[evs_start..(evs_start + 12)];

        // || MISCELLANOUS DATA EXTRACT ||
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

        let national_id = get_national3(species_id);

        let pokemon_specie = species_data[national_id as usize].clone();

        pokemon_team.push(Pokemon {
            nick: nickname,
            species: pokemon_specie,
            item: item_id,
            experience,
            friendship,
            moves,
            pokerus: misc_data[0],
            met_location: misc_data[1],
            personality_id: p_id,
            is_egg,
            ev: Stats {
                hp: evs_data[0],
                attk: evs_data[1],
                s_attk: evs_data[2],
                def: evs_data[3],
                s_def: evs_data[4],
                speed: evs_data[5],
                ability,
            },
            iv: Stats {
                hp: get_bits_from_u32(iv_value, 0, 5),
                attk: get_bits_from_u32(iv_value, 5, 5),
                s_attk: get_bits_from_u32(iv_value, 10, 5),
                def: get_bits_from_u32(iv_value, 15, 5),
                s_def: get_bits_from_u32(iv_value, 20, 5),
                speed: get_bits_from_u32(iv_value, 25, 5),
                ability,
            },
            contest: ContestData {
                coolness: evs_data[6],
                beauty: evs_data[7],
                cuteness: evs_data[8],
                smartness: evs_data[9],
                toughness: evs_data[10],
                feel: evs_data[11],
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

fn get_bits_from_u32(value: u32, offset: u32, width: u32) -> u8 {
    ((value >> offset) & ((1 << width) - 1)) as u8
}
fn get_bits_from_u8(value: u8, offset: u8, width: u8) -> u8 {
    ((value >> offset) & ((1 << width) - 1)) as u8
}
