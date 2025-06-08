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
        }
    };
}
