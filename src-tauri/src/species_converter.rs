const UNALIGNED_3_NATIONAL: u16 = 252;
const UNALIGNED_3_INTERNAL: u16 = 277;

pub fn get_national3(species: u16) -> u16 {
    if species < UNALIGNED_3_NATIONAL {
        return species;
    }
    let shift = species - UNALIGNED_3_INTERNAL;
    if shift >= (GEN_3_INTERNAL_TO_NATIONAL.len() as u16) {
        return 0;
    }

    let national_id = i32::from(species) + i32::from(GEN_3_INTERNAL_TO_NATIONAL[shift as usize]);
    return national_id as u16;
}

const GEN_3_INTERNAL_TO_NATIONAL: [i16; 135] = [
    -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
    -25, -25, -25, -25, -25, -11, -11, -11, -28, -28, -21, -21, 019, -31, -31, -28, -28, 007, 007,
    -15, -15, 035, 025, 025, -21, 003, -20, 016, 016, 045, 015, 015, 021, 021, -12, -12, -04, -04,
    -04, -39, -39, -28, -28, -17, -17, 022, 022, 022, -13, -13, 015, 015, -11, -11, -52, -26, -26,
    -42, -42, -52, -49, -49, -25, -25, 000, -06, -06, -48, -77, -77, -77, -51, -51, -12, -77, -77,
    -77, -07, -07, -07, -17, -24, -24, -43, -45, -12, -78, -78, -78, -34, -73, -73, -43, -43, -43,
    -43, -112, -112, -112, -24, -24, -24, -24, -24, -24, -24, -24, -24, -22, -22, -22, -27, -27,
    -24, -24, -53,
];
