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

            fn parse_hands<I: Iterator<Item = String>>(mut lines: I) -> Vec<TileCounts> {
                const NUM_CASES: usize = 10_000;
                let mut hands = Vec::with_capacity(NUM_CASES);
                while let Some(line) = lines.next() {
                    let tokens: Vec<_> = line
                        .split_whitespace()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect();
                    if tokens.len() != 14 {
                        panic!(
                            "invalid input line: expected 14 tokens, got {}: '{}'",
                            tokens.len(),
                            line
                        );
                    }
                    let mut counts = [0u8; 34];
                    for &tid in &tokens {
                        counts[tid] += 1;
                    }
                    hands.push(counts);
                }

                if hands.len() != NUM_CASES {
                    panic!(
                        "invalid input file: expected {} lines, got {}",
                        NUM_CASES,
                        hands.len()
                    );
                }

                hands
            }

            fn load_hands(filename: &str) -> Vec<TileCounts> {
                let path = std::path::Path::new(filename);
                let file = File::open(&path).expect(&format!("hands file not found: {:?}", path));
                let lines = BufReader::new(file).lines().filter_map(Result::ok);
                parse_hands(lines)
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
