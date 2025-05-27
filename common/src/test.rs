//! This module provides macros for testing shanten calculations.

#[macro_export]
macro_rules! shanten_test_case {
    ($calculator_type:ty, $hand:expr, $expected:expr) => {
        let calculator = <$calculator_type>::new();
        assert_eq!(calculator.calculate_shanten(&$hand), $expected);
    };
}

#[macro_export]
macro_rules! shanten_tests {
    ($calculator_type:ty) => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use $crate::shanten_test_case;

            #[test]
            fn test_shanten_thirteen_orphans() {
                shanten_test_case!(
                    $calculator_type,
                    [
                        2, 0, 0, 0, 0, 0, 0, 0, 1, // m
                        1, 0, 0, 0, 0, 0, 0, 0, 1, // p
                        1, 0, 0, 0, 0, 0, 0, 0, 1, // s
                        1, 1, 1, 1, 1, 1, 1, // z
                    ],
                    7
                );
            }

            #[test]
            fn test_shanten_thirteen_wait_thirteen_orphans() {
                shanten_test_case!(
                    $calculator_type,
                    [
                        1, 0, 0, 0, 0, 0, 0, 0, 1, // m
                        1, 0, 0, 0, 0, 0, 0, 0, 1, // p
                        1, 0, 0, 0, 0, 0, 0, 0, 1, // s
                        1, 1, 1, 1, 1, 1, 1, // z
                    ],
                    8
                );
            }

            #[test]
            fn test_shanten_tenpai() {
                shanten_test_case!(
                    $calculator_type,
                    [
                        1, 1, 1, 0, 0, 0, 0, 0, 0, // m
                        0, 0, 0, 1, 1, 1, 0, 0, 0, // p
                        0, 0, 0, 0, 0, 0, 1, 1, 1, // s
                        2, 2, 0, 0, 0, 0, 0, // z
                    ],
                    0
                );
            }

            #[test]
            fn test_shanten_tenpai_win() {
                shanten_test_case!(
                    $calculator_type,
                    [
                        1, 1, 1, 0, 0, 0, 0, 0, 0, // m
                        0, 0, 0, 1, 1, 1, 0, 0, 0, // p
                        0, 0, 0, 0, 0, 0, 1, 1, 1, // s
                        2, 3, 0, 0, 0, 0, 0, // z
                    ],
                    -1
                );
            }
        }
    };
}
