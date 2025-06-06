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
