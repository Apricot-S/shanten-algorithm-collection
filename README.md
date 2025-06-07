# shanten-algorithm-collection

Shanten Number Calculation Algorithm Collection

This project is a collection for implementing and comparing multiple algorithms to calculate the shanten number (the minimum number of tile replacements needed to reach a winning hand).  
It provides a common interface and test macros to make it easy to implement and verify each algorithm.

For details of each algorithm, please refer to the README of each subcrate.

## Module Overview

- [`common`](common)  
  A common library for implementing shanten number calculation algorithms.  
  Provides traits, types, and test macros required for algorithm implementation.

- [`dummy`](dummy)  
  An example implementation of a dummy shanten number calculation algorithm.  
  Useful as a reference for implementation and test macro usage.

- Other subcrates  
  Contains various implementations of shanten number calculation algorithms.  
  For details, see the README of each subcrate.

## Usage

1. To add a new algorithm, create a directory as a subcrate and implement the `common::ShantenCalculator` trait.
2. For testing, you can use the `common::shanten_tests!` macro to automatically generate standard test cases.

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
