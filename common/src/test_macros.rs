/// Macro to generate shanten number calculation tests for a specified `ShantenCalculator` type.
///
/// This macro defines several typical test cases as unit tests.
/// Call this macro at the crate root or in your implementation file to automatically
/// generate tests for your `ShantenCalculator` implementation.
/// (Do not wrap it in your own `mod tests`.)
///
/// # Arguments
///
/// * `$calculator_type` - A type that implements the `ShantenCalculator` trait
///
/// # Example
///
/// ```
/// use common::shanten_tests;
/// use common::{ShantenCalculator, TileCounts};
///
/// struct Dummy {}
///
/// impl ShantenCalculator for Dummy {
///     fn new() -> Self {
///         Dummy {}
///     }
///     fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
///         0
///     }
/// }
///
/// shanten_tests!(Dummy);
/// ```
///
/// Historical implementations may explicitly declare the supported known-failure set. Exact
/// implementations must use the single-argument form so every case is required to pass.
#[macro_export]
macro_rules! shanten_tests {
    ($calculator_type:ty) => {
        $crate::shanten_tests!(@generate $calculator_type, [], []);
    };
    ($calculator_type:ty, known_failures = [
        test_shanten_incomplete_hand_4_melds_without_a_pair,
        test_shanten_waiting_for_the_5th_tile_1,
        test_shanten_waiting_for_the_5th_tile_2,
        test_shanten_waiting_for_the_5th_tile_3,
        test_shanten_2_isolated_4_tiles_3,
        test_shanten_2_isolated_4_tiles_5,
        test_shanten_4_honors_1,
        test_shanten_4_honors_2,
        test_shanten_4_honors_3,
        test_shanten_4_honors_4,
        test_shanten_lack_isolated_tile_13_4333,
        test_shanten_lack_isolated_tile_13_4432i,
        test_shanten_lack_isolated_tile_13_4432ii,
        test_shanten_lack_isolated_tile_13_4441,
        test_shanten_lack_isolated_tile_14_4433,
        test_shanten_lack_isolated_tile_14_4442i,
        test_shanten_lack_isolated_tile_14_4442ii $(,)?
    ], reason = $reason:literal) => {
        $crate::shanten_tests!(@generate $calculator_type, [#[ignore = $reason]], [#[ignore = $reason]]);
    };
    ($calculator_type:ty, known_failures = [
        test_shanten_waiting_for_the_5th_tile_1,
        test_shanten_waiting_for_the_5th_tile_2,
        test_shanten_waiting_for_the_5th_tile_3,
        test_shanten_2_isolated_4_tiles_3,
        test_shanten_2_isolated_4_tiles_5,
        test_shanten_4_honors_1,
        test_shanten_4_honors_2,
        test_shanten_4_honors_3,
        test_shanten_4_honors_4,
        test_shanten_lack_isolated_tile_13_4333,
        test_shanten_lack_isolated_tile_13_4432i,
        test_shanten_lack_isolated_tile_13_4432ii,
        test_shanten_lack_isolated_tile_13_4441,
        test_shanten_lack_isolated_tile_14_4433,
        test_shanten_lack_isolated_tile_14_4442i,
        test_shanten_lack_isolated_tile_14_4442ii $(,)?
    ], reason = $reason:literal) => {
        $crate::shanten_tests!(@generate $calculator_type, [#[ignore = $reason]], []);
    };
    (@generate $calculator_type:ty, [$($known_failure_attr:tt)*], [$($incomplete_hand_attr:tt)*]) => {
        #[cfg(test)]
        mod tests {
            use super::*;

            fn assert_shanten(hand: &str, expected: i8) {
                let calculator = <$calculator_type as $crate::ShantenCalculator>::new();
                let counts =
                    <$crate::TileCounts as $crate::TileCountsExt>::from_code(hand);
                let actual = $crate::ShantenCalculator::calculate_shanten(&calculator, &counts);
                assert_eq!(actual, expected, "hand: {hand}");
            }

            #[test]
            fn test_shanten_thirteen_orphans() {
                assert_shanten("11m19p19s1234567z", 7);
            }

            #[test]
            fn test_shanten_thirteen_wait_thirteen_orphans() {
                assert_shanten("19m19p19s1234567z", 8);
            }

            #[test]
            fn test_shanten_tenpai() {
                assert_shanten("123m456p789s1122z", 0);
            }

            #[test]
            fn test_shanten_win() {
                assert_shanten("123m456p789s11222z", -1);
            }

            #[test]
            fn test_shanten_with_meld() {
                assert_shanten("123m456p789s2z", 0);
            }

            #[test]
            fn test_shanten_without_pair() {
                // Source: https://blog.kobalab.net/entry/20151216/1450191666 雀頭がない場合
                assert_shanten("12389m456p12789s1z", 1);
            }

            #[test]
            fn test_shanten_too_many_meld_candidates() {
                // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子過多の場合
                assert_shanten("12389m456p1289s11z", 1);
            }

            #[test]
            fn test_shanten_not_enough_meld_candidates() {
                // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子不足の場合
                assert_shanten("133345568m23677z", 2);
            }

            #[test]
            $($incomplete_hand_attr)*
            fn test_shanten_incomplete_hand_4_melds_without_a_pair() {
                assert_shanten("234p567s", 1);
            }

            #[test]
            fn test_shanten_triplet_sequence() {
                assert_shanten("222345p1234567z", 4);
            }

            #[test]
            fn test_shanten_sequence_isolated_sequence() {
                assert_shanten("2344456p123456z", 4);
            }

            #[test]
            fn test_shanten_pair_triplet_sequence() {
                assert_shanten("11222345p12345z", 3);
            }

            #[test]
            fn test_shanten_pair_sequence_sequence_pair() {
                assert_shanten("2234556788p123z", 2);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_waiting_for_the_5th_tile_1() {
                // Source: https://blog.kobalab.net/entry/2022/04/17/174206 5枚目の牌を待つ形
                assert_shanten("1111m123p112233s", 1);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_waiting_for_the_5th_tile_2() {
                assert_shanten("1111234444m1111p", 1);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_waiting_for_the_5th_tile_3() {
                // Source: http://cmj3.web.fc2.com/#syanten
                assert_shanten("11112222333444z", 1);
            }

            #[test]
            fn test_shanten_2_isolated_4_tiles_1() {
                assert_shanten("1111247777m", 1);
            }

            #[test]
            fn test_shanten_2_isolated_4_tiles_2() {
                assert_shanten("1111247777m1112z", 1);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_2_isolated_4_tiles_3() {
                assert_shanten("11114444m", 1);
            }

            #[test]
            fn test_shanten_2_isolated_4_tiles_4() {
                assert_shanten("111124m1111z", 1);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_2_isolated_4_tiles_5() {
                assert_shanten("1111444478m", 2);
            }

            #[test]
            fn test_shanten_3_isolated_4_tiles() {
                assert_shanten("1111247777m1111z", 1);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_4_honors_1() {
                assert_shanten("1111z", 1);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_4_honors_2() {
                assert_shanten("123m1111z", 1);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_4_honors_3() {
                assert_shanten("11112222z", 1);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_4_honors_4() {
                assert_shanten("123m11p11112222z", 2);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_lack_isolated_tile_13_4333() {
                // Source: https://zenn.dev/tomohxx/articles/aecace4e3a3bc1
                assert_shanten("1111222333444z", 1);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_lack_isolated_tile_13_4432i() {
                // Source: https://zenn.dev/tomohxx/articles/aecace4e3a3bc1
                assert_shanten("11m11112222333z", 2);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_lack_isolated_tile_13_4432ii() {
                // Source: https://zenn.dev/tomohxx/articles/aecace4e3a3bc1
                assert_shanten("23m11112222333z", 2);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_lack_isolated_tile_13_4441() {
                // Source: https://zenn.dev/tomohxx/articles/aecace4e3a3bc1
                assert_shanten("1111222233334z", 3);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_lack_isolated_tile_14_4433() {
                // Source: https://zenn.dev/tomohxx/articles/aecace4e3a3bc1
                assert_shanten("11112222333444z", 1);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_lack_isolated_tile_14_4442i() {
                // Source: https://zenn.dev/tomohxx/articles/aecace4e3a3bc1
                assert_shanten("11m111122223333z", 2);
            }

            #[test]
            $($known_failure_attr)*
            fn test_shanten_lack_isolated_tile_14_4442ii() {
                // Source: https://zenn.dev/tomohxx/articles/aecace4e3a3bc1
                assert_shanten("23m111122223333z", 2);
            }
        }
    };
}
