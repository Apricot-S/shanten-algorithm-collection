#![feature(test)]

extern crate test;

use common::{NUM_TILE_TYPE, ShantenCalculator, TileCount, TileCounts};
use common::{shanten_benches, shanten_tests};

const MAX_SHANTEN: i8 = 8;
const NUM_MELD_TYPE: usize = NUM_TILE_TYPE + 7 * 3;

type Mentsu = [usize; 3];

fn add_mentsu(hora_form: &mut TileCounts, mentsu: &Mentsu) {
    mentsu.iter().for_each(|&t| hora_form[t] += 1);
}

fn remove_mentsu(hora_form: &mut TileCounts, mentsu: &Mentsu) {
    mentsu.iter().for_each(|&t| hora_form[t] -= 1);
}

fn is_valid_hora_form(hora_form: &TileCounts) -> bool {
    hora_form.iter().all(|&n| n <= 4)
}

fn calculate_shanten_number(tehai: &TileCounts, hora_form: &TileCounts) -> i8 {
    hora_form
        .iter()
        .zip(tehai)
        .map(|(&h, &t)| if (h - t) > 0 { h - t } else { 0 })
        .sum::<TileCount>() as i8
        - 1
}

struct MyAlgo {
    mentsu_set: [Mentsu; NUM_MELD_TYPE],
}

impl ShantenCalculator for MyAlgo {
    fn new() -> Self {
        let mut mentsu_set = [[0usize; 3]; NUM_MELD_TYPE];

        // kotsu
        for i in 0..NUM_TILE_TYPE {
            mentsu_set[i] = [i, i, i];
        }

        // shuntsu
        let min_shuntsu_id: Vec<usize> = (0..(9 * 3)).filter(|&n| n % 9 < 7).collect();
        for (i, &n) in min_shuntsu_id.iter().enumerate() {
            mentsu_set[NUM_TILE_TYPE + i] = [n, n + 1, n + 2];
        }

        MyAlgo { mentsu_set }
    }

    fn calculate_shanten(&self, tehai: &TileCounts) -> i8 {
        // Implementation of the Shanten calculation algorithm
        0
    }
}

shanten_tests!(MyAlgo);
shanten_benches!(MyAlgo);
