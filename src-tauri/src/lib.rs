const SAVE_B_OFFSET: usize = 57344;
const SECTION_SIZE: usize = 4096;
const SECTION_COUNT: usize = 14;

#[tauri::command]
fn open_file(bytes: Vec<u8>) -> String {
    let save_b = &bytes[SAVE_B_OFFSET..SAVE_B_OFFSET * 2];
    let mut sections: Vec<&[u8]> = Vec::with_capacity(14);
    for index in 0..SECTION_COUNT {
        let start = index * SECTION_SIZE;
        sections.push(&save_b[start..start + SECTION_SIZE]);
    }
    //TODO:
    // - Add checksum
    // - Split data
    for section in sections {
        let section_id = u16::from_le_bytes(section[0x0FF4..0x0FF6].try_into().unwrap());
        println!("{}", section_id)
    }

    "Hello, {}! You've been greeted from Rust!".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![open_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
