// Copyright 2020 The RustABC Authors.
//
// Code is licensed under Apache License, Version 2.0.

use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::compare_functions::fibonaccis,
}
