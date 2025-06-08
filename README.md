# shanten-algorithm-collection

Shanten Number Calculation Algorithm Collection

This project is a collection for implementing and comparing multiple algorithms to calculate the shanten number (the minimum number of tile replacements needed to reach a winning hand).  
It provides a common interface and test macros to make it easy to implement and verify each algorithm.

## Components

- [common](common)  
  A common library for implementing shanten number calculation algorithms.  
  Provides traits, types, and test macros required for algorithm implementation.

- [dummy](dummy)  
  An example implementation of a dummy shanten number calculation algorithm.  
  Useful as a reference for implementation and test macro usage.

- Other subcrates  
  Contains various implementations of shanten number calculation algorithms.

## Usage

1. To add a new algorithm, first create a directory as a subcrate using the following command:

    ```sh
    cargo new --lib your_algorithm
    ```

   Replace `your_algorithm` with the desired subcrate name.

2. Next, implement the `common::ShantenCalculator` trait in your subcrate.
3. For testing, you can use the `common::shanten_tests!` macro to automatically generate standard test cases.
4. To run the tests for your algorithm, execute the following command:

    ```sh
    cargo test --package your_algorithm
    ```

   Replace `your_algorithm` with the name of your subcrate.

### Example Implementation

```rust
use common::shanten_tests;
use common::{ShantenCalculator, TileCounts};

struct MyAlgorithm {}

impl ShantenCalculator for MyAlgorithm {
    fn new() -> Self { MyAlgorithm {} }
    fn calculate_shanten(&self, hand: &TileCounts) -> i8 {
        // Algorithm implementation
        0
    }
}

// Automatically generate test cases
shanten_tests!(MyAlgorithm);
```

## License

Copyright (c) Apricot S. All rights reserved.

Licensed under the [MIT license](LICENSE).
