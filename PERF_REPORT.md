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
| broomfilter-blocked | 0 | 245 | 0.002450 | 2048 bits |
| bloom | 0 | 1154 | 0.011540 | 2048 bits |

### Build

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 876.46 ns (871.71 ns, 881.59 ns) | 6.85 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/build_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 1.43 µs (1.43 µs, 1.43 µs) | 11.17 ns/op | 0.61x | `broomfilter-blocked [fp=0.002450, fn=0]` | [plot](target/criterion/build_compact-128/broomfilter-blocked%20%5Bfp%3D0.002450%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.92 µs (1.92 µs, 1.93 µs) | 15.03 ns/op | 0.46x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/build_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 2.85 µs (2.84 µs, 2.85 µs) | 22.23 ns/op | 0.31x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/build_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 4.46 µs (4.45 µs, 4.47 µs) | 34.85 ns/op | 0.20x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/build_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 22.88 µs (22.68 µs, 23.03 µs) | 5.59 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_member_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 47.54 µs (46.53 µs, 49.29 µs) | 11.61 ns/op | 0.48x | `broomfilter-blocked [fp=0.002450, fn=0]` | [plot](target/criterion/contains_member_compact-128/broomfilter-blocked%20%5Bfp%3D0.002450%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 59.85 µs (59.69 µs, 60.06 µs) | 14.61 ns/op | 0.38x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_member_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 81.32 µs (80.87 µs, 81.90 µs) | 19.85 ns/op | 0.28x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_member_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 142.47 µs (142.20 µs, 142.75 µs) | 34.78 ns/op | 0.16x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_member_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 8.72 µs (8.66 µs, 8.79 µs) | 2.13 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_absent_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 35.65 µs (35.40 µs, 35.87 µs) | 8.70 ns/op | 0.24x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_absent_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | broomfilter-blocked | 65.85 µs (65.06 µs, 67.18 µs) | 16.08 ns/op | 0.13x | `broomfilter-blocked [fp=0.002450, fn=0]` | [plot](target/criterion/contains_absent_compact-128/broomfilter-blocked%20%5Bfp%3D0.002450%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 70.22 µs (67.61 µs, 72.20 µs) | 17.14 ns/op | 0.12x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_absent_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 93.85 µs (93.41 µs, 94.35 µs) | 22.91 ns/op | 0.09x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_absent_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 17.81 µs (17.62 µs, 17.99 µs) | 4.35 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 63.06 µs (62.13 µs, 64.76 µs) | 15.40 ns/op | 0.28x | `broomfilter-blocked [fp=0.002450, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/broomfilter-blocked%20%5Bfp%3D0.002450%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 66.57 µs (65.94 µs, 67.12 µs) | 16.25 ns/op | 0.27x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 88.59 µs (88.11 µs, 89.05 µs) | 21.63 ns/op | 0.20x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 138.35 µs (138.16 µs, 138.57 µs) | 33.78 ns/op | 0.13x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

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
| broomfilter-blocked | 0 | 6195 | 0.061950 | 65536 bits |

### Build

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 25.70 µs (25.63 µs, 25.75 µs) | 6.27 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/build_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 48.77 µs (48.61 µs, 48.93 µs) | 11.91 ns/op | 0.53x | `broomfilter-blocked [fp=0.061950, fn=0]` | [plot](target/criterion/build_scale-4096/broomfilter-blocked%20%5Bfp%3D0.061950%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 65.03 µs (64.23 µs, 65.60 µs) | 15.88 ns/op | 0.40x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/build_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 91.28 µs (90.92 µs, 91.68 µs) | 22.28 ns/op | 0.28x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/build_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 141.60 µs (141.44 µs, 141.76 µs) | 34.57 ns/op | 0.18x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/build_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 93.42 µs (92.95 µs, 93.77 µs) | 5.70 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_member_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 195.20 µs (191.14 µs, 202.62 µs) | 11.91 ns/op | 0.48x | `broomfilter-blocked [fp=0.061950, fn=0]` | [plot](target/criterion/contains_member_scale-4096/broomfilter-blocked%20%5Bfp%3D0.061950%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 240.60 µs (239.95 µs, 241.20 µs) | 14.69 ns/op | 0.39x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_member_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 323.55 µs (322.51 µs, 324.59 µs) | 19.75 ns/op | 0.29x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_member_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 570.82 µs (569.68 µs, 572.11 µs) | 34.84 ns/op | 0.16x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_member_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 56.55 µs (51.31 µs, 60.85 µs) | 3.45 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 180.32 µs (175.16 µs, 185.73 µs) | 11.01 ns/op | 0.31x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | broomfilter-blocked | 293.47 µs (289.62 µs, 299.94 µs) | 17.91 ns/op | 0.19x | `broomfilter-blocked [fp=0.061950, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/broomfilter-blocked%20%5Bfp%3D0.061950%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 332.60 µs (328.07 µs, 337.34 µs) | 20.30 ns/op | 0.17x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 446.78 µs (445.55 µs, 447.99 µs) | 27.27 ns/op | 0.13x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 100.45 µs (98.93 µs, 102.18 µs) | 6.13 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 252.33 µs (248.70 µs, 258.89 µs) | 15.40 ns/op | 0.40x | `broomfilter-blocked [fp=0.061950, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/broomfilter-blocked%20%5Bfp%3D0.061950%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 283.82 µs (282.05 µs, 285.50 µs) | 17.32 ns/op | 0.35x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 359.36 µs (358.36 µs, 360.25 µs) | 21.93 ns/op | 0.28x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 551.95 µs (550.17 µs, 553.71 µs) | 33.69 ns/op | 0.18x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

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
| broomfilter-blocked | 0 | 21335 | 0.213350 | 1048576 bits |

### Build

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 432.35 µs (431.42 µs, 433.28 µs) | 6.60 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 788.76 µs (785.78 µs, 792.51 µs) | 12.04 ns/op | 0.55x | `broomfilter-blocked [fp=0.213350, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/broomfilter-blocked%20%5Bfp%3D0.213350%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.05 ms (1.03 ms, 1.07 ms) | 16.04 ns/op | 0.41x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.50 ms (1.50 ms, 1.50 ms) | 22.91 ns/op | 0.29x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 2.31 ms (2.30 ms, 2.31 ms) | 35.21 ns/op | 0.19x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 396.86 µs (395.95 µs, 397.75 µs) | 6.06 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 775.56 µs (759.69 µs, 805.42 µs) | 11.83 ns/op | 0.51x | `broomfilter-blocked [fp=0.213350, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/broomfilter-blocked%20%5Bfp%3D0.213350%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 967.90 µs (966.34 µs, 969.31 µs) | 14.77 ns/op | 0.41x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.29 ms (1.29 ms, 1.29 ms) | 19.69 ns/op | 0.31x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 2.27 ms (2.27 ms, 2.28 ms) | 34.68 ns/op | 0.17x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 642.98 µs (633.78 µs, 651.61 µs) | 9.81 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 1.22 ms (1.21 ms, 1.24 ms) | 18.66 ns/op | 0.53x | `broomfilter-blocked [fp=0.213350, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/broomfilter-blocked%20%5Bfp%3D0.213350%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.23 ms (1.22 ms, 1.24 ms) | 18.75 ns/op | 0.52x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.54 ms (1.54 ms, 1.54 ms) | 23.50 ns/op | 0.42x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 2.10 ms (2.09 ms, 2.12 ms) | 32.03 ns/op | 0.31x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 535.35 µs (532.58 µs, 538.05 µs) | 8.17 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 995.85 µs (980.95 µs, 1.02 ms) | 15.20 ns/op | 0.54x | `broomfilter-blocked [fp=0.213350, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/broomfilter-blocked%20%5Bfp%3D0.213350%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.18 ms (1.18 ms, 1.19 ms) | 18.06 ns/op | 0.45x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.44 ms (1.43 ms, 1.44 ms) | 21.96 ns/op | 0.37x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 2.25 ms (2.24 ms, 2.26 ms) | 34.26 ns/op | 0.24x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

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
| broomfilter-blocked | 0 | 32082 | 0.320820 | 524288 bits |

### Build

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 246.98 µs (246.54 µs, 247.35 µs) | 3.77 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 572.51 µs (562.92 µs, 583.33 µs) | 8.74 ns/op | 0.43x | `broomfilter-blocked [fp=0.320820, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/broomfilter-blocked%20%5Bfp%3D0.320820%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 737.63 µs (710.17 µs, 776.62 µs) | 11.26 ns/op | 0.33x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.21 ms (1.20 ms, 1.21 ms) | 18.40 ns/op | 0.20x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 1.76 ms (1.76 ms, 1.76 ms) | 26.87 ns/op | 0.14x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 232.31 µs (231.97 µs, 232.66 µs) | 3.54 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 573.14 µs (561.96 µs, 594.22 µs) | 8.75 ns/op | 0.41x | `broomfilter-blocked [fp=0.320820, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/broomfilter-blocked%20%5Bfp%3D0.320820%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 727.75 µs (726.36 µs, 728.85 µs) | 11.10 ns/op | 0.32x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 983.98 µs (980.71 µs, 987.48 µs) | 15.01 ns/op | 0.24x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 1.76 ms (1.75 ms, 1.77 ms) | 26.83 ns/op | 0.13x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 604.69 µs (594.54 µs, 615.62 µs) | 9.23 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 904.80 µs (892.39 µs, 928.13 µs) | 13.81 ns/op | 0.67x | `broomfilter-blocked [fp=0.320820, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/broomfilter-blocked%20%5Bfp%3D0.320820%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.20 ms (1.19 ms, 1.22 ms) | 18.36 ns/op | 0.50x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.57 ms (1.57 ms, 1.57 ms) | 23.96 ns/op | 0.39x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 2.12 ms (2.12 ms, 2.13 ms) | 32.40 ns/op | 0.28x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 401.53 µs (391.62 µs, 411.13 µs) | 6.13 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 770.78 µs (757.88 µs, 794.84 µs) | 11.76 ns/op | 0.52x | `broomfilter-blocked [fp=0.320820, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/broomfilter-blocked%20%5Bfp%3D0.320820%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.07 ms (1.05 ms, 1.08 ms) | 16.27 ns/op | 0.38x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.35 ms (1.34 ms, 1.36 ms) | 20.55 ns/op | 0.30x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 1.97 ms (1.96 ms, 1.98 ms) | 30.09 ns/op | 0.20x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

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
| broomfilter-blocked | 0 | 39531 | 0.395310 | 262144 bits |

### Build

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 157.90 µs (157.68 µs, 158.11 µs) | 2.41 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 450.81 µs (447.69 µs, 454.13 µs) | 6.88 ns/op | 0.35x | `broomfilter-blocked [fp=0.395310, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/broomfilter-blocked%20%5Bfp%3D0.395310%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 576.78 µs (575.38 µs, 578.52 µs) | 8.80 ns/op | 0.27x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 896.53 µs (895.83 µs, 897.19 µs) | 13.68 ns/op | 0.18x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 1.32 ms (1.31 ms, 1.32 ms) | 20.12 ns/op | 0.12x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 150.62 µs (150.41 µs, 150.79 µs) | 2.30 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 344.97 µs (327.17 µs, 379.38 µs) | 5.26 ns/op | 0.44x | `broomfilter-blocked [fp=0.395310, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/broomfilter-blocked%20%5Bfp%3D0.395310%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 581.08 µs (580.31 µs, 581.89 µs) | 8.87 ns/op | 0.26x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 788.70 µs (787.45 µs, 789.93 µs) | 12.03 ns/op | 0.19x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 1.30 ms (1.30 ms, 1.31 ms) | 19.90 ns/op | 0.12x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 526.28 µs (513.23 µs, 538.29 µs) | 8.03 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 627.46 µs (610.45 µs, 651.67 µs) | 9.57 ns/op | 0.84x | `broomfilter-blocked [fp=0.395310, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/broomfilter-blocked%20%5Bfp%3D0.395310%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.18 ms (1.17 ms, 1.20 ms) | 18.00 ns/op | 0.45x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.53 ms (1.53 ms, 1.54 ms) | 23.40 ns/op | 0.34x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 2.01 ms (2.00 ms, 2.01 ms) | 30.64 ns/op | 0.26x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 316.87 µs (303.13 µs, 331.24 µs) | 4.84 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 612.96 µs (601.66 µs, 634.54 µs) | 9.35 ns/op | 0.52x | `broomfilter-blocked [fp=0.395310, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/broomfilter-blocked%20%5Bfp%3D0.395310%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 975.47 µs (971.79 µs, 978.61 µs) | 14.88 ns/op | 0.32x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.21 ms (1.20 ms, 1.21 ms) | 18.42 ns/op | 0.26x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 1.67 ms (1.66 ms, 1.67 ms) | 25.44 ns/op | 0.19x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

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
| broomfilter-blocked | 0 | 63202 | 0.632020 | 131072 bits |

### Build

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 100.30 µs (98.64 µs, 102.54 µs) | 1.53 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 268.94 µs (268.02 µs, 270.34 µs) | 4.10 ns/op | 0.37x | `broomfilter-blocked [fp=0.632020, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/broomfilter-blocked%20%5Bfp%3D0.632020%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 495.55 µs (493.93 µs, 497.40 µs) | 7.56 ns/op | 0.20x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 603.14 µs (596.34 µs, 610.20 µs) | 9.20 ns/op | 0.17x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloom | 847.53 µs (839.70 µs, 856.65 µs) | 12.93 ns/op | 0.12x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 102.21 µs (101.92 µs, 102.50 µs) | 1.56 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 314.70 µs (309.07 µs, 325.42 µs) | 4.80 ns/op | 0.32x | `broomfilter-blocked [fp=0.632020, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/broomfilter-blocked%20%5Bfp%3D0.632020%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 510.38 µs (507.27 µs, 513.46 µs) | 7.79 ns/op | 0.20x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 590.35 µs (588.69 µs, 592.48 µs) | 9.01 ns/op | 0.17x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloom | 760.58 µs (752.58 µs, 770.85 µs) | 11.61 ns/op | 0.13x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 181.16 µs (151.23 µs, 214.95 µs) | 2.76 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 388.31 µs (381.20 µs, 400.22 µs) | 5.93 ns/op | 0.47x | `broomfilter-blocked [fp=0.632020, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/broomfilter-blocked%20%5Bfp%3D0.632020%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 950.43 µs (948.58 µs, 952.75 µs) | 14.50 ns/op | 0.19x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.12 ms (1.12 ms, 1.13 ms) | 17.14 ns/op | 0.16x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloom | 1.44 ms (1.43 ms, 1.44 ms) | 21.90 ns/op | 0.13x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 173.08 µs (157.38 µs, 189.02 µs) | 2.64 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 383.17 µs (375.79 µs, 396.94 µs) | 5.85 ns/op | 0.45x | `broomfilter-blocked [fp=0.632020, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/broomfilter-blocked%20%5Bfp%3D0.632020%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 727.12 µs (722.82 µs, 733.09 µs) | 11.09 ns/op | 0.24x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 851.46 µs (849.39 µs, 853.55 µs) | 12.99 ns/op | 0.20x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloom | 1.07 ms (1.07 ms, 1.08 ms) | 16.35 ns/op | 0.16x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

