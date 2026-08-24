/// Macro to generate benchmark functions for a specified `ShantenCalculator` type.
///
/// This macro reads benchmark data files containing 14-tile hands (e.g., `hands_normal_10000.txt`)
/// and generates benchmark functions that measure the performance of the `calculate_shanten` method
/// for each hand.
///
/// # Arguments
///
/// * `$calculator_type` - A type that implements the `ShantenCalculator` trait
///
/// # Generated Benchmark Functions
///
/// - `bench_shanten_normal_10000`: 10,000 normal hands
/// - `bench_shanten_half_flush_10000`: 10,000 half flush hands
/// - `bench_shanten_full_flush_10000`: 10,000 full flush hands
/// - `bench_shanten_thirteen_orphans_10000`: 10,000 thirteen orphans hands
///
/// Each function loads the corresponding resource file from the `resources/` directory and
/// benchmarks the `calculate_shanten` method of `$calculator_type` for all hands in the file.
///
/// # Example
///
/// ```
/// #![feature(test)]
/// extern crate test;
/// use common::shanten_benches;
///
/// shanten_benches!(YourAlgorithm);
/// ```
///
/// # Notes
///
/// - Running the benchmarks requires nightly Rust and the `test` crate.
/// - Each line in the input file must consist of 14 tile indices (0–33).
///   The macro will panic if any line is malformed or if the number of lines is incorrect.
#[macro_export]
macro_rules! shanten_benches {
    ($calculator_type:ty) => {
        #[cfg(test)]
        mod benches {
            use std::fs::File;
            use std::io::{BufRead, BufReader};

            use test::Bencher;
            use $crate::{ShantenCalculator, TileCounts};

            use super::*;

            fn parse_hands<I: IntoIterator<Item = String>>(lines: I) -> Vec<TileCounts> {
                const NUM_CASES: usize = 10_000;
                let mut hands = Vec::with_capacity(NUM_CASES);
                for line in lines {
                    let tokens: Vec<_> = line
                        .split_whitespace()
                        .filter_map(|s| s.parse::<usize>().ok())
                        .collect();
                    assert_eq!(
                        tokens.len(),
                        14,
                        "invalid input line: expected 14 tokens, got {}: '{line}'",
                        tokens.len()
                    );
                    let mut counts = [0u8; 34];
                    for &tid in &tokens {
                        counts[tid] += 1;
                    }
                    hands.push(counts);
                }

                assert_eq!(
                    hands.len(),
                    NUM_CASES,
                    "invalid input file: expected {NUM_CASES} lines, got {}",
                    hands.len()
                );

                hands
            }

            fn load_hands(filename: &str) -> Vec<TileCounts> {
                let path = std::path::Path::new(filename);
                let file = File::open(path)
                    .unwrap_or_else(|error| panic!("hands file not found: {path:?}: {error}"));
                let lines = BufReader::new(file).lines().map_while(Result::ok);
                parse_hands(lines)
            }

            fn bench_hands(b: &mut Bencher, filename: &str) {
                let calculator = <$calculator_type as $crate::ShantenCalculator>::new();
                let hands = load_hands(filename);
                b.iter(|| {
                    for hand in &hands {
                        std::hint::black_box($crate::ShantenCalculator::calculate_shanten(
                            &calculator,
                            hand,
                        ));
                    }
                });
            }

            #[bench]
            fn bench_shanten_normal_10000(b: &mut Bencher) {
                bench_hands(b, "../../resources/hands_normal_10000.txt");
            }

            #[bench]
            fn bench_shanten_half_flush_10000(b: &mut Bencher) {
                bench_hands(b, "../../resources/hands_half_flush_10000.txt");
            }

            #[bench]
            fn bench_shanten_full_flush_10000(b: &mut Bencher) {
                bench_hands(b, "../../resources/hands_full_flush_10000.txt");
            }

            #[bench]
            fn bench_shanten_thirteen_orphans_10000(b: &mut Bencher) {
                bench_hands(b, "../../resources/hands_thirteen_orphans_10000.txt");
            }
        }
    };
}
