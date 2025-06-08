/// ベンチマーク用マクロ
///
/// # Example
///
/// ```
/// #![feature(test)]
///
/// extern crate test;
///
/// use common::shanten_benches;
///
/// shanten_benches!(MyAlgorithm);
/// ```
#[macro_export]
macro_rules! shanten_benches {
    ($calculator_type:ty) => {
        #[cfg(test)]
        mod benches {
            use super::*;
            use std::fs::File;
            use std::io::{BufRead, BufReader};
            use test::Bencher;
            use $crate::{ShantenCalculator, TileCounts};

            fn read_hand_indices<I: Iterator<Item = String>>(mut lines: I) -> Vec<TileCounts> {
                let mut hands = Vec::new();
                while let Some(line) = lines.next() {
                    let tokens: Vec<_> = line
                        .split_whitespace()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect();
                    if tokens.len() < 14 {
                        continue;
                    }
                    let mut counts = [0u8; 34];
                    for &tid in &tokens[0..14] {
                        counts[tid] += 1;
                    }
                    hands.push(counts);
                }
                hands
            }

            fn load_hands(filename: &str) -> Vec<TileCounts> {
                let path = std::path::Path::new(filename);
                let file = File::open(&path).expect(&format!("hands file not found: {:?}", path));
                let lines = BufReader::new(file).lines().filter_map(Result::ok);
                read_hand_indices(lines)
            }

            #[bench]
            fn bench_shanten_normal_10000(b: &mut Bencher) {
                let calculator = <$calculator_type as $crate::ShantenCalculator>::new();
                let hands = load_hands("../../resources/hands_normal_10000.txt");
                b.iter(|| {
                    for hand in &hands {
                        _ = $crate::ShantenCalculator::calculate_shanten(&calculator, hand);
                    }
                });
            }

            #[bench]
            fn bench_shanten_half_flush_10000(b: &mut Bencher) {
                let calculator = <$calculator_type as $crate::ShantenCalculator>::new();
                let hands = load_hands("../../resources/hands_half_flush_10000.txt");
                b.iter(|| {
                    for hand in &hands {
                        _ = $crate::ShantenCalculator::calculate_shanten(&calculator, hand);
                    }
                });
            }

            #[bench]
            fn bench_shanten_full_flush_10000(b: &mut Bencher) {
                let calculator = <$calculator_type as $crate::ShantenCalculator>::new();
                let hands = load_hands("../../resources/hands_full_flush_10000.txt");
                b.iter(|| {
                    for hand in &hands {
                        _ = $crate::ShantenCalculator::calculate_shanten(&calculator, hand);
                    }
                });
            }

            #[bench]
            fn bench_shanten_thirteen_orphans_10000(b: &mut Bencher) {
                let calculator = <$calculator_type as $crate::ShantenCalculator>::new();
                let hands = load_hands("../../resources/hands_thirteen_orphans_10000.txt");
                b.iter(|| {
                    for hand in &hands {
                        _ = $crate::ShantenCalculator::calculate_shanten(&calculator, hand);
                    }
                });
            }
        }
    };
}
