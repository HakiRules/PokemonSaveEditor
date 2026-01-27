pub const SAVE_B_OFFSET: usize = 57344;
pub const SECTION_SIZE: usize = 4096;
pub const SECTION_COUNT: usize = 14;
pub const TRAINER_SECTION_ID: u8 = 0;
pub const TEAM_SECTION_ID: u8 = 1;

pub const CHARACTER_MAP: [char; 256] = {
    let mut map = ['?'; 256];
    map[0x7F] = ' ';
    map[0xBB] = 'A';
    map[0xBC] = 'B';
    map[0xBD] = 'C';
    map[0xBE] = 'D';
    map[0xBF] = 'E';
    map[0xC0] = 'F';
    map[0xC1] = 'G';
    map[0xC2] = 'H';
    map[0xC3] = 'I';
    map[0xC4] = 'J';
    map[0xC5] = 'K';
    map[0xC6] = 'L';
    map[0xC7] = 'M';
    map[0xC8] = 'N';
    map[0xC9] = 'O';
    map[0xCA] = 'P';
    map[0xCB] = 'Q';
    map[0xCC] = 'R';
    map[0xCD] = 'S';
    map[0xCE] = 'T';
    map[0xCF] = 'U';
    map[0xD0] = 'V';
    map[0xD1] = 'W';
    map[0xD2] = 'X';
    map[0xD3] = 'Y';
    map[0xD4] = 'Z';
    map[0xD5] = 'a';
    map[0xD6] = 'b';
    map[0xD7] = 'c';
    map[0xD8] = 'd';
    map[0xD9] = 'e';
    map[0xDA] = 'f';
    map[0xDB] = 'g';
    map[0xDC] = 'h';
    map[0xDD] = 'i';
    map[0xDE] = 'j';
    map[0xDF] = 'k';
    map[0xE0] = 'l';
    map[0xE1] = 'm';
    map[0xE2] = 'n';
    map[0xE3] = 'o';
    map[0xE4] = 'p';
    map[0xE5] = 'q';
    map[0xE6] = 'r';
    map[0xE7] = 's';
    map[0xE8] = 't';
    map[0xE9] = 'u';
    map[0xEA] = 'v';
    map[0xEB] = 'w';
    map[0xEC] = 'x';
    map[0xED] = 'y';
    map[0xEE] = 'z';
    map
};

pub const BLOCK_ORDER: [[char; 4]; 24] = [
    ['A', 'B', 'C', 'D'],
    ['A', 'B', 'D', 'C'],
    ['A', 'C', 'B', 'D'],
    ['A', 'C', 'D', 'B'],
    ['A', 'D', 'B', 'C'],
    ['A', 'D', 'C', 'B'],
    ['B', 'A', 'C', 'D'],
    ['B', 'A', 'D', 'C'],
    ['B', 'C', 'A', 'D'],
    ['B', 'C', 'D', 'A'],
    ['B', 'D', 'A', 'C'],
    ['B', 'D', 'C', 'A'],
    ['C', 'A', 'B', 'D'],
    ['C', 'A', 'D', 'B'],
    ['C', 'B', 'A', 'D'],
    ['C', 'B', 'D', 'A'],
    ['C', 'D', 'A', 'B'],
    ['C', 'D', 'B', 'A'],
    ['D', 'A', 'B', 'C'],
    ['D', 'A', 'C', 'B'],
    ['D', 'B', 'A', 'C'],
    ['D', 'B', 'C', 'A'],
    ['D', 'C', 'A', 'B'],
    ['D', 'C', 'B', 'A'],
];
