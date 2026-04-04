# Performance Report

Generated from the Criterion benchmark suite in `benches/comparison.rs`. This report compares `broomfilter` against other mutable Bloom-style filters under equal per-scenario memory budgets, deterministic setup, and identical key datasets.

- Overall report: [Criterion dashboard](target/criterion/report/index.html)
- Benchmarks covered: build, present lookups, absent lookups, and mixed lookups
- Comparison set: mutable Bloom-style filters that can be configured to the same exact bit budget through their public APIs
- Precision covered: false negatives on inserted keys and false positives over 100,000 deterministic absent-key probes per scenario

## Scenario Summary

| Scenario | Bits/item | Most precise | Fastest build | Fastest present lookup | Fastest absent lookup | Fastest mixed lookup |
| --- | ---: | --- | --- | --- | --- | --- |
| `compact-128` | 16.00 | fastbloom | broomfilter | broomfilter | broomfilter | broomfilter |
| `scale-4096` | 16.00 | bloomfilter | broomfilter | broomfilter | broomfilter | broomfilter |
| `large-65536-16bpi` | 16.00 | fastbloom | broomfilter | broomfilter | broomfilter | broomfilter |
| `large-65536-8bpi` | 8.00 | broomfilter | broomfilter | broomfilter | broomfilter | broomfilter |
| `large-65536-4bpi` | 4.00 | fastbloom | broomfilter | broomfilter | broomfilter | broomfilter |
| `stress-65536-2bpi` | 2.00 | broomfilter | broomfilter | broomfilter | broomfilter | broomfilter |

## Scenario `compact-128`

- Inserted items: 128
- Shared filter size: 2048 bits (256 bytes)
- Bits per inserted item: 16.00
- Query batch size: 4096
- Precision probes: 100000

### Precision

| Library | False negatives | False positives | FP rate | Config |
| --- | ---: | ---: | ---: | --- |
| **fastbloom** | 0 | 40 | 0.000400 | 2048 bits |
| bloomfilter | 0 | 75 | 0.000750 | 2048 bits |
| broomfilter | 0 | 82 | 0.000820 | 2048 bits |
| bloom | 0 | 1154 | 0.011540 | 2048 bits |

### Build

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 930.28 ns (926.84 ns, 933.76 ns) | 7.27 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/build_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.86 µs (1.85 µs, 1.88 µs) | 14.57 ns/op | 0.50x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/build_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 2.82 µs (2.81 µs, 2.83 µs) | 22.01 ns/op | 0.33x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/build_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 4.44 µs (4.36 µs, 4.55 µs) | 34.68 ns/op | 0.21x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/build_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 27.29 µs (26.84 µs, 27.80 µs) | 6.66 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_member_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 56.48 µs (56.26 µs, 56.72 µs) | 13.79 ns/op | 0.48x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_member_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 81.39 µs (79.96 µs, 82.75 µs) | 19.87 ns/op | 0.34x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_member_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 143.17 µs (141.51 µs, 144.84 µs) | 34.95 ns/op | 0.19x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_member_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 8.27 µs (8.14 µs, 8.40 µs) | 2.02 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_absent_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 39.55 µs (37.90 µs, 41.39 µs) | 9.66 ns/op | 0.21x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_absent_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 67.48 µs (66.58 µs, 68.51 µs) | 16.48 ns/op | 0.12x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_absent_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 96.46 µs (95.06 µs, 97.75 µs) | 23.55 ns/op | 0.09x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_absent_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 17.53 µs (17.36 µs, 17.73 µs) | 4.28 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 66.36 µs (64.79 µs, 68.17 µs) | 16.20 ns/op | 0.26x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 88.12 µs (87.29 µs, 89.15 µs) | 21.51 ns/op | 0.20x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 133.60 µs (132.74 µs, 134.27 µs) | 32.62 ns/op | 0.13x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

## Scenario `scale-4096`

- Inserted items: 4096
- Shared filter size: 65536 bits (8192 bytes)
- Bits per inserted item: 16.00
- Query batch size: 16384
- Precision probes: 100000

### Precision

| Library | False negatives | False positives | FP rate | Config |
| --- | ---: | ---: | ---: | --- |
| **bloomfilter** | 0 | 41 | 0.000410 | 65536 bits |
| broomfilter | 0 | 47 | 0.000470 | 65536 bits |
| fastbloom | 0 | 51 | 0.000510 | 65536 bits |
| bloom | 0 | 978 | 0.009780 | 65536 bits |

### Build

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 28.73 µs (28.59 µs, 28.89 µs) | 7.02 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/build_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 58.68 µs (58.53 µs, 58.82 µs) | 14.33 ns/op | 0.49x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/build_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 91.14 µs (90.69 µs, 91.64 µs) | 22.25 ns/op | 0.32x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/build_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 143.22 µs (142.78 µs, 143.83 µs) | 34.96 ns/op | 0.20x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/build_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 103.81 µs (103.42 µs, 104.21 µs) | 6.34 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_member_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 233.14 µs (232.03 µs, 234.15 µs) | 14.23 ns/op | 0.45x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_member_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 337.49 µs (336.34 µs, 338.67 µs) | 20.60 ns/op | 0.31x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_member_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 573.54 µs (570.88 µs, 576.16 µs) | 35.01 ns/op | 0.18x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_member_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 44.90 µs (38.85 µs, 52.13 µs) | 2.74 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 219.61 µs (209.48 µs, 228.94 µs) | 13.40 ns/op | 0.20x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 335.65 µs (332.87 µs, 338.58 µs) | 20.49 ns/op | 0.13x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 461.42 µs (455.75 µs, 466.49 µs) | 28.16 ns/op | 0.10x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 94.13 µs (92.22 µs, 96.41 µs) | 5.74 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 280.91 µs (279.46 µs, 282.84 µs) | 17.15 ns/op | 0.34x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 364.99 µs (363.95 µs, 365.98 µs) | 22.28 ns/op | 0.26x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 542.09 µs (539.91 µs, 544.43 µs) | 33.09 ns/op | 0.17x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

## Scenario `large-65536-16bpi`

- Inserted items: 65536
- Shared filter size: 1048576 bits (131072 bytes)
- Bits per inserted item: 16.00
- Query batch size: 65536
- Precision probes: 100000

### Precision

| Library | False negatives | False positives | FP rate | Config |
| --- | ---: | ---: | ---: | --- |
| **fastbloom** | 0 | 50 | 0.000500 | 1048576 bits |
| bloomfilter | 0 | 55 | 0.000550 | 1048576 bits |
| broomfilter | 0 | 56 | 0.000560 | 1048576 bits |
| bloom | 0 | 1047 | 0.010470 | 1048576 bits |

### Build

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 495.61 µs (493.62 µs, 497.09 µs) | 7.56 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 954.62 µs (948.67 µs, 959.73 µs) | 14.57 ns/op | 0.52x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.50 ms (1.50 ms, 1.51 ms) | 22.96 ns/op | 0.33x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.31 ms (2.30 ms, 2.32 ms) | 35.22 ns/op | 0.21x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 433.88 µs (430.70 µs, 437.46 µs) | 6.62 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 961.45 µs (957.14 µs, 965.56 µs) | 14.67 ns/op | 0.45x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.34 ms (1.32 ms, 1.35 ms) | 20.39 ns/op | 0.32x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.28 ms (2.27 ms, 2.29 ms) | 34.80 ns/op | 0.19x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 613.55 µs (598.04 µs, 629.95 µs) | 9.36 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.20 ms (1.19 ms, 1.21 ms) | 18.35 ns/op | 0.51x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.58 ms (1.57 ms, 1.58 ms) | 24.07 ns/op | 0.39x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.07 ms (2.06 ms, 2.07 ms) | 31.55 ns/op | 0.30x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 560.73 µs (556.82 µs, 565.37 µs) | 8.56 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.17 ms (1.16 ms, 1.17 ms) | 17.83 ns/op | 0.48x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.45 ms (1.44 ms, 1.46 ms) | 22.16 ns/op | 0.39x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.23 ms (2.22 ms, 2.23 ms) | 34.01 ns/op | 0.25x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

## Scenario `large-65536-8bpi`

- Inserted items: 65536
- Shared filter size: 524288 bits (65536 bytes)
- Bits per inserted item: 8.00
- Query batch size: 65536
- Precision probes: 100000

### Precision

| Library | False negatives | False positives | FP rate | Config |
| --- | ---: | ---: | ---: | --- |
| **broomfilter** | 0 | 2109 | 0.021090 | 524288 bits |
| bloomfilter | 0 | 2152 | 0.021520 | 524288 bits |
| fastbloom | 0 | 2218 | 0.022180 | 524288 bits |
| bloom | 0 | 4515 | 0.045150 | 524288 bits |

### Build

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 292.48 µs (291.85 µs, 293.14 µs) | 4.46 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 710.59 µs (707.90 µs, 713.62 µs) | 10.84 ns/op | 0.41x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.19 ms (1.19 ms, 1.20 ms) | 18.16 ns/op | 0.25x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.77 ms (1.76 ms, 1.78 ms) | 27.01 ns/op | 0.17x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 250.61 µs (249.54 µs, 251.80 µs) | 3.82 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 729.64 µs (725.34 µs, 733.57 µs) | 11.13 ns/op | 0.34x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 978.95 µs (974.15 µs, 984.07 µs) | 14.94 ns/op | 0.26x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.74 ms (1.74 ms, 1.75 ms) | 26.59 ns/op | 0.14x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 595.84 µs (586.79 µs, 607.10 µs) | 9.09 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.23 ms (1.22 ms, 1.24 ms) | 18.76 ns/op | 0.48x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.58 ms (1.57 ms, 1.59 ms) | 24.11 ns/op | 0.38x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.11 ms (2.09 ms, 2.12 ms) | 32.14 ns/op | 0.28x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 393.46 µs (386.60 µs, 400.31 µs) | 6.00 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.06 ms (1.05 ms, 1.07 ms) | 16.19 ns/op | 0.37x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.34 ms (1.34 ms, 1.34 ms) | 20.45 ns/op | 0.29x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.94 ms (1.93 ms, 1.94 ms) | 29.57 ns/op | 0.20x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

## Scenario `large-65536-4bpi`

- Inserted items: 65536
- Shared filter size: 262144 bits (32768 bytes)
- Bits per inserted item: 4.00
- Query batch size: 65536
- Precision probes: 100000

### Precision

| Library | False negatives | False positives | FP rate | Config |
| --- | ---: | ---: | ---: | --- |
| **fastbloom** | 0 | 14509 | 0.145090 | 262144 bits |
| broomfilter | 0 | 14548 | 0.145480 | 262144 bits |
| bloomfilter | 0 | 14589 | 0.145890 | 262144 bits |
| bloom | 0 | 15880 | 0.158800 | 262144 bits |

### Build

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 204.92 µs (204.47 µs, 205.59 µs) | 3.13 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 647.97 µs (645.75 µs, 649.97 µs) | 9.89 ns/op | 0.32x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 944.96 µs (931.09 µs, 958.50 µs) | 14.42 ns/op | 0.22x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.32 ms (1.31 ms, 1.33 ms) | 20.15 ns/op | 0.16x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 161.44 µs (160.99 µs, 161.89 µs) | 2.46 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 590.56 µs (588.17 µs, 592.71 µs) | 9.01 ns/op | 0.27x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 836.85 µs (818.43 µs, 864.57 µs) | 12.77 ns/op | 0.19x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.30 ms (1.30 ms, 1.31 ms) | 19.89 ns/op | 0.12x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 533.40 µs (519.01 µs, 546.95 µs) | 8.14 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.24 ms (1.23 ms, 1.26 ms) | 18.97 ns/op | 0.43x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.60 ms (1.59 ms, 1.62 ms) | 24.48 ns/op | 0.33x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.97 ms (1.96 ms, 1.98 ms) | 30.08 ns/op | 0.27x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 260.12 µs (253.30 µs, 268.28 µs) | 3.97 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 961.96 µs (958.32 µs, 965.41 µs) | 14.68 ns/op | 0.27x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.25 ms (1.23 ms, 1.28 ms) | 19.13 ns/op | 0.21x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.65 ms (1.64 ms, 1.66 ms) | 25.19 ns/op | 0.16x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

## Scenario `stress-65536-2bpi`

- Inserted items: 65536
- Shared filter size: 131072 bits (16384 bytes)
- Bits per inserted item: 2.00
- Query batch size: 65536
- Precision probes: 100000

### Precision

| Library | False negatives | False positives | FP rate | Config |
| --- | ---: | ---: | ---: | --- |
| **broomfilter** | 0 | 39316 | 0.393160 | 131072 bits |
| fastbloom | 0 | 39395 | 0.393950 | 131072 bits |
| bloomfilter | 0 | 39482 | 0.394820 | 131072 bits |
| bloom | 0 | 40007 | 0.400070 | 131072 bits |

### Build

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 184.21 µs (159.44 µs, 219.68 µs) | 2.81 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | bloomfilter | 606.08 µs (604.21 µs, 607.90 µs) | 9.25 ns/op | 0.30x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 652.68 µs (649.06 µs, 655.89 µs) | 9.96 ns/op | 0.28x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 833.85 µs (830.05 µs, 838.37 µs) | 12.72 ns/op | 0.22x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 105.78 µs (104.96 µs, 106.55 µs) | 1.61 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 516.14 µs (513.84 µs, 518.23 µs) | 7.88 ns/op | 0.20x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloomfilter | 600.18 µs (599.00 µs, 601.76 µs) | 9.16 ns/op | 0.18x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 755.66 µs (751.97 µs, 759.65 µs) | 11.53 ns/op | 0.14x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 255.57 µs (241.53 µs, 268.89 µs) | 3.90 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 975.85 µs (970.97 µs, 981.00 µs) | 14.89 ns/op | 0.26x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloomfilter | 1.13 ms (1.13 ms, 1.14 ms) | 17.27 ns/op | 0.23x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.41 ms (1.40 ms, 1.41 ms) | 21.47 ns/op | 0.18x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 121.81 µs (111.05 µs, 134.68 µs) | 1.86 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 742.61 µs (739.91 µs, 745.27 µs) | 11.33 ns/op | 0.16x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloomfilter | 869.86 µs (864.67 µs, 876.28 µs) | 13.27 ns/op | 0.14x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.07 ms (1.07 ms, 1.07 ms) | 16.32 ns/op | 0.11x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

