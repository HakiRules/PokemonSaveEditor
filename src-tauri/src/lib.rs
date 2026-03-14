mod gen3;
mod gen3constants;
mod species_converter;
mod species_name;

use crate::gen3::Gen3Data;

const GEN_4_SIZE: usize = 524410;
const GEN_3_SIZE: usize = 131088;

#[tauri::command]
fn open_file(bytes: Vec<u8>) -> Gen3Data {
    let save_length = bytes.len();
    if save_length == GEN_3_SIZE || save_length < GEN_4_SIZE {
        gen3::parse_gen_3(bytes)
    } else {
        Gen3Data {
            team: Vec::new(),
            trainer_nick: String::new(),
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
