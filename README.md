# concaveman-cpp benchmark in Rust

## Introduction

Benchmark test for [concaveman-cpp](https://github.com/sadaszewski/concaveman-cpp) wrapped in RUST, using this repository <https://github.com/TehGoat/rs_concaveman>

## Requirements

- Rust

## Execute

Skip this step if you have installed Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then execute the benchmark

```bash
cargo bench
```

```text
Running benches/concave_benchmark.rs (target/release/deps/concave_benchmark-ca9021b328b2da03)
Benchmarking create concave hull from 10^6 random points: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 487.7s, or reduce sample count to 10.
create concave hull from 10^6 random points             time:   [4.2612 s 4.3341 s 4.4155 s]
                                                        change: [+273284% +279935% +285742%] (p = 0.00 < 0.05)
                                                        Performance has regressed.
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) high mild
  3 (3.00%) high severe
```
