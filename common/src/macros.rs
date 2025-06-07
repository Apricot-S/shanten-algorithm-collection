/// Macro for a single shanten number calculation test case.
///
/// # Arguments
///
/// * `$calculator_type` - The type implementing `ShantenCalculator`
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
        let calculator = <$calculator_type>::new();
        assert_eq!(calculator.calculate_shanten(&$hand), $expected);
    };
}

/// Macro to generate a set of shanten number calculation tests for a calculator type.
///
/// This macro defines several typical test cases as unit tests.
/// Call this macro at the crate root or in your implementation file to automatically
/// generate tests for your `ShantenCalculator` implementation.
/// (Do not wrap it in your own `mod tests`.)
///
/// # Arguments
///
/// * `$calculator_type` - The type implementing `ShantenCalculator`
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
                shanten_test_case!(
                    $calculator_type,
                    TileCounts::from_code("11m19p19s1234567z"),
                    7
                );
            }

            #[test]
            fn test_shanten_thirteen_wait_thirteen_orphans() {
                shanten_test_case!(
                    $calculator_type,
                    TileCounts::from_code("19m19p19s1234567z"),
                    8
                );
            }

            #[test]
            fn test_shanten_tenpai() {
                shanten_test_case!(
                    $calculator_type,
                    TileCounts::from_code("123m456p789s1122z"),
                    0
                );
            }

            #[test]
            fn test_shanten_tenpai_win() {
                shanten_test_case!(
                    $calculator_type,
                    TileCounts::from_code("123m456p789s11222z"),
                    -1
                );
            }
        }
    };
}
