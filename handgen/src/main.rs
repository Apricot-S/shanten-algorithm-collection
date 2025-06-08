use common::{MAX_HAND_SIZE, MAX_NUM_TILE, NUM_TILE_TYPE};
use rand::rngs::StdRng;
use rand::seq::{IndexedRandom, SliceRandom};
use rand::{Rng, SeedableRng};
use std::fs::File;
use std::io::{BufWriter, Write};

const HAND_SIZE: usize = MAX_HAND_SIZE as usize;
const NUM_WALL: usize = (NUM_TILE_TYPE as usize) * (MAX_NUM_TILE as usize);
const NUM_CASES: usize = 10_000;

fn draw_tiles(wall: &[u8]) -> [u8; HAND_SIZE] {
    let mut hand = [0u8; HAND_SIZE];
    hand.copy_from_slice(&wall[..HAND_SIZE]);
    hand
}

fn generate_normal_hand(rng: &mut impl Rng) -> [u8; HAND_SIZE] {
    let mut wall: [u8; NUM_WALL] = std::array::from_fn(|i| (i / 4) as u8);
    wall.shuffle(rng);
    draw_tiles(wall.as_slice())
}

fn generate_half_flush_hand(rng: &mut impl Rng) -> [u8; HAND_SIZE] {
    let color_start = [0, 9, 18].choose(rng).unwrap();
    let suits: [u8; 9 * 4] = std::array::from_fn(|i| (i / 4 + color_start) as u8);
    let honors: [u8; 7 * 4] = std::array::from_fn(|i| (i / 4 + 27) as u8);
    let mut combined = suits.into_iter().chain(honors);
    let mut wall: [u8; 36 + 28] = std::array::from_fn(|_| combined.next().unwrap());
    wall.shuffle(rng);
    draw_tiles(wall.as_slice())
}

fn generate_full_flush_hand(rng: &mut impl Rng) -> [u8; HAND_SIZE] {
    let color_start = [0, 9, 18].choose(rng).unwrap();
    let mut wall: [u8; 9 * 4] = std::array::from_fn(|i| (i / 4 + color_start) as u8);
    wall.shuffle(rng);
    draw_tiles(wall.as_slice())
}

fn generate_non_simple_hand(rng: &mut impl Rng) -> [u8; HAND_SIZE] {
    const NON_SIMPLES: [u8; 13] = [0, 8, 9, 17, 18, 26, 27, 28, 29, 30, 31, 32, 33];
    let mut wall: [u8; 13 * 4] = std::array::from_fn(|i| NON_SIMPLES[i % 13]);
    wall.shuffle(rng);
    draw_tiles(wall.as_slice())
}

fn write_cases(filename: &str, cases: &Vec<[u8; HAND_SIZE]>) -> std::io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    for hand in cases {
        for (i, &tile) in hand.iter().enumerate() {
            if i > 0 {
                write!(writer, " ")?;
            }
            write!(writer, "{}", tile)?;
        }
        writeln!(writer)?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut rng = StdRng::seed_from_u64(42);

    let mut normal_cases = Vec::with_capacity(NUM_CASES);
    let mut honitsu_cases = Vec::with_capacity(NUM_CASES);
    let mut chinitsu_cases = Vec::with_capacity(NUM_CASES);
    let mut non_simple_cases = Vec::with_capacity(NUM_CASES);

    for _ in 0..NUM_CASES {
        normal_cases.push(generate_normal_hand(&mut rng));
        honitsu_cases.push(generate_half_flush_hand(&mut rng));
        chinitsu_cases.push(generate_full_flush_hand(&mut rng));
        non_simple_cases.push(generate_non_simple_hand(&mut rng));
    }

    write_cases("resources/hands_normal_10000.txt", &normal_cases)?;
    write_cases("resources/hands_half_flush_10000.txt", &honitsu_cases)?;
    write_cases("resources/hands_full_flush_10000.txt", &chinitsu_cases)?;
    write_cases(
        "resources/hands_thirteen_orphans_10000.txt",
        &non_simple_cases,
    )?;

    Ok(())
}
