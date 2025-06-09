/// Macro for a single shanten number calculation test case.
///
/// # Arguments
///
/// * `$calculator_type` - A type that implements the `ShantenCalculator` trait
/// * `$hand` - The hand to test (as a `TileCounts`)
/// * `$expected` - The expected shanten number
///
/// # Note
///
/// This macro is exported for use in other macros, but users should not call it directly.
/// Please use [`shanten_tests!`] to generate test cases.
#[macro_export]
macro_rules! shanten_test_case {
    ($calculator_type:ty, $hand:expr, $expected:expr) => {
        let calculator = <$calculator_type as $crate::ShantenCalculator>::new();
        let counts = TileCounts::from_code(&$hand);
        assert_eq!(
            $crate::ShantenCalculator::calculate_shanten(&calculator, &counts),
            $expected
        );
    };
}

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
#[macro_export]
macro_rules! shanten_tests {
    ($calculator_type:ty) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::TileCountsExt;
            use $crate::shanten_test_case;

            #[test]
            fn test_shanten_thirteen_orphans() {
                shanten_test_case!($calculator_type, "11m19p19s1234567z", 7);
            }

            #[test]
            fn test_shanten_thirteen_wait_thirteen_orphans() {
                shanten_test_case!($calculator_type, "19m19p19s1234567z", 8);
            }

            #[test]
            fn test_shanten_tenpai() {
                shanten_test_case!($calculator_type, "123m456p789s1122z", 0);
            }

            #[test]
            fn test_shanten_win() {
                shanten_test_case!($calculator_type, "123m456p789s11222z", -1);
            }

            #[test]
            fn test_shanten_with_meld() {
                shanten_test_case!($calculator_type, "123m456p789s2z", 0);
            }

            #[test]
            fn test_shanten_without_pair() {
                // Source: https://blog.kobalab.net/entry/20151216/1450191666 雀頭がない場合
                shanten_test_case!($calculator_type, "12389m456p12789s1z", 1);
            }

            #[test]
            fn test_shanten_too_many_meld_candidates() {
                // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子過多の場合
                shanten_test_case!($calculator_type, "12389m456p1289s11z", 1);
            }

            #[test]
            fn test_shanten_not_enough_meld_candidates() {
                // Source: https://blog.kobalab.net/entry/20151216/1450191666 搭子不足の場合
                shanten_test_case!($calculator_type, "133345568m23677z", 2);
            }

            #[test]
            fn test_shanten_incomplete_hand_4_melds_without_a_pair() {
                shanten_test_case!($calculator_type, "234p567s", 1);
            }

            #[test]
            fn test_shanten_triplet_sequence() {
                shanten_test_case!($calculator_type, "222345p1234567z", 4);
            }

            #[test]
            fn test_shanten_sequence_isolated_sequence() {
                shanten_test_case!($calculator_type, "2344456p123456z", 4);
            }

            #[test]
            fn test_shanten_pair_triplet_sequence() {
                shanten_test_case!($calculator_type, "11222345p12345z", 3);
            }

            #[test]
            fn test_shanten_pair_sequence_sequence_pair() {
                shanten_test_case!($calculator_type, "2234556788p123z", 2);
            }

            #[test]
            fn test_shanten_waiting_for_the_5th_tile_1() {
                // Source: https://blog.kobalab.net/entry/2022/04/17/174206 5枚目の牌を待つ形
                shanten_test_case!($calculator_type, "1111m123p112233s", 1);
            }

            #[test]
            fn test_shanten_waiting_for_the_5th_tile_2() {
                shanten_test_case!($calculator_type, "1111234444m1111p", 1);
            }

            #[test]
            fn test_shanten_waiting_for_the_5th_tile_3() {
                // Source: http://cmj3.web.fc2.com/#syanten
                shanten_test_case!($calculator_type, "11112222333444z", 1);
            }

            #[test]
            fn test_shanten_2_isolated_4_tiles_1() {
                shanten_test_case!($calculator_type, "1111247777m", 1);
            }

            #[test]
            fn test_shanten_2_isolated_4_tiles_2() {
                shanten_test_case!($calculator_type, "1111247777m1112z", 1);
            }

            #[test]
            fn test_shanten_2_isolated_4_tiles_3() {
                shanten_test_case!($calculator_type, "11114444m", 1);
            }

            #[test]
            fn test_shanten_2_isolated_4_tiles_4() {
                shanten_test_case!($calculator_type, "111124m1111z", 1);
            }

            #[test]
            fn test_shanten_2_isolated_4_tiles_5() {
                shanten_test_case!($calculator_type, "1111444478m", 2);
            }

            #[test]
            fn test_shanten_3_isolated_4_tiles() {
                shanten_test_case!($calculator_type, "1111247777m1111z", 1);
            }

            #[test]
            fn test_shanten_4_honors_1() {
                shanten_test_case!($calculator_type, "1111z", 1);
            }

            #[test]
            fn test_shanten_4_honors_2() {
                shanten_test_case!($calculator_type, "123m1111z", 1);
            }

            #[test]
            fn test_shanten_4_honors_3() {
                shanten_test_case!($calculator_type, "11112222z", 1);
            }

            #[test]
            fn test_shanten_4_honors_4() {
                shanten_test_case!($calculator_type, "123m11p11112222z", 2);
            }
        }
    };
}
