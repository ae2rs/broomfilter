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
| 1 | **broomfilter** | 734.26 ns (730.57 ns, 737.47 ns) | 5.74 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/build_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.81 µs (1.80 µs, 1.82 µs) | 14.14 ns/op | 0.41x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/build_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 2.84 µs (2.82 µs, 2.86 µs) | 22.17 ns/op | 0.26x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/build_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 4.38 µs (4.37 µs, 4.40 µs) | 34.25 ns/op | 0.17x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/build_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 24.93 µs (24.79 µs, 25.08 µs) | 6.09 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_member_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 56.92 µs (56.64 µs, 57.17 µs) | 13.90 ns/op | 0.44x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_member_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 79.02 µs (78.46 µs, 79.59 µs) | 19.29 ns/op | 0.32x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_member_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 139.67 µs (139.06 µs, 140.23 µs) | 34.10 ns/op | 0.18x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_member_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 8.34 µs (8.19 µs, 8.49 µs) | 2.04 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_absent_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 33.78 µs (33.51 µs, 34.04 µs) | 8.25 ns/op | 0.25x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_absent_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 61.99 µs (59.68 µs, 64.52 µs) | 15.14 ns/op | 0.13x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_absent_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 91.09 µs (89.32 µs, 92.96 µs) | 22.24 ns/op | 0.09x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_absent_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 17.72 µs (17.57 µs, 17.89 µs) | 4.33 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 62.65 µs (62.11 µs, 63.27 µs) | 15.30 ns/op | 0.28x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 86.71 µs (86.24 µs, 87.21 µs) | 21.17 ns/op | 0.20x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 134.67 µs (133.91 µs, 135.48 µs) | 32.88 ns/op | 0.13x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 22.21 µs (22.07 µs, 22.38 µs) | 5.42 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/build_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 57.70 µs (57.49 µs, 57.97 µs) | 14.09 ns/op | 0.38x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/build_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 91.26 µs (90.92 µs, 91.66 µs) | 22.28 ns/op | 0.24x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/build_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 141.10 µs (140.36 µs, 141.85 µs) | 34.45 ns/op | 0.16x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/build_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 103.31 µs (103.01 µs, 103.65 µs) | 6.31 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_member_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 233.83 µs (232.61 µs, 235.11 µs) | 14.27 ns/op | 0.44x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_member_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 325.62 µs (323.64 µs, 327.74 µs) | 19.87 ns/op | 0.32x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_member_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 586.39 µs (569.10 µs, 609.60 µs) | 35.79 ns/op | 0.18x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_member_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 55.61 µs (51.41 µs, 59.24 µs) | 3.39 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 191.06 µs (186.04 µs, 198.64 µs) | 11.66 ns/op | 0.29x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 331.25 µs (327.99 µs, 334.25 µs) | 20.22 ns/op | 0.17x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 439.38 µs (433.40 µs, 446.57 µs) | 26.82 ns/op | 0.13x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 101.92 µs (99.55 µs, 104.53 µs) | 6.22 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 269.63 µs (268.65 µs, 270.63 µs) | 16.46 ns/op | 0.38x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 354.16 µs (352.38 µs, 355.89 µs) | 21.62 ns/op | 0.29x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 545.13 µs (543.70 µs, 546.65 µs) | 33.27 ns/op | 0.19x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 398.43 µs (397.26 µs, 399.65 µs) | 6.08 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 947.03 µs (942.90 µs, 951.28 µs) | 14.45 ns/op | 0.42x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.50 ms (1.49 ms, 1.50 ms) | 22.87 ns/op | 0.27x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.31 ms (2.30 ms, 2.33 ms) | 35.27 ns/op | 0.17x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 430.69 µs (428.48 µs, 432.98 µs) | 6.57 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 954.62 µs (946.12 µs, 966.98 µs) | 14.57 ns/op | 0.45x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.29 ms (1.29 ms, 1.30 ms) | 19.75 ns/op | 0.33x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.27 ms (2.26 ms, 2.28 ms) | 34.61 ns/op | 0.19x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 618.94 µs (612.02 µs, 625.88 µs) | 9.44 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.17 ms (1.14 ms, 1.19 ms) | 17.81 ns/op | 0.53x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.55 ms (1.54 ms, 1.57 ms) | 23.71 ns/op | 0.40x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.06 ms (2.04 ms, 2.08 ms) | 31.46 ns/op | 0.30x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 567.66 µs (560.71 µs, 574.91 µs) | 8.66 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.15 ms (1.14 ms, 1.15 ms) | 17.53 ns/op | 0.49x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.44 ms (1.43 ms, 1.46 ms) | 22.02 ns/op | 0.39x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.22 ms (2.21 ms, 2.22 ms) | 33.86 ns/op | 0.26x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 228.51 µs (227.88 µs, 229.33 µs) | 3.49 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 701.37 µs (699.87 µs, 702.71 µs) | 10.70 ns/op | 0.33x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.21 ms (1.20 ms, 1.21 ms) | 18.40 ns/op | 0.19x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.75 ms (1.75 ms, 1.76 ms) | 26.76 ns/op | 0.13x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 254.57 µs (251.10 µs, 258.25 µs) | 3.88 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 725.15 µs (722.71 µs, 727.20 µs) | 11.06 ns/op | 0.35x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 981.70 µs (977.92 µs, 985.28 µs) | 14.98 ns/op | 0.26x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.74 ms (1.73 ms, 1.74 ms) | 26.52 ns/op | 0.15x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 602.51 µs (586.62 µs, 619.41 µs) | 9.19 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.20 ms (1.17 ms, 1.23 ms) | 18.32 ns/op | 0.50x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.61 ms (1.60 ms, 1.61 ms) | 24.51 ns/op | 0.38x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.11 ms (2.10 ms, 2.12 ms) | 32.14 ns/op | 0.29x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 398.97 µs (391.84 µs, 405.86 µs) | 6.09 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.04 ms (1.03 ms, 1.04 ms) | 15.81 ns/op | 0.39x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.36 ms (1.35 ms, 1.37 ms) | 20.73 ns/op | 0.29x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.95 ms (1.94 ms, 1.96 ms) | 29.76 ns/op | 0.20x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 147.87 µs (147.47 µs, 148.27 µs) | 2.26 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 580.61 µs (577.08 µs, 584.01 µs) | 8.86 ns/op | 0.25x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 897.14 µs (894.22 µs, 901.01 µs) | 13.69 ns/op | 0.16x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.32 ms (1.31 ms, 1.32 ms) | 20.12 ns/op | 0.11x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 162.23 µs (161.24 µs, 163.22 µs) | 2.48 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 586.35 µs (583.02 µs, 589.92 µs) | 8.95 ns/op | 0.28x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 792.26 µs (789.75 µs, 795.51 µs) | 12.09 ns/op | 0.20x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.31 ms (1.30 ms, 1.31 ms) | 19.92 ns/op | 0.12x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 553.68 µs (545.04 µs, 560.60 µs) | 8.45 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.25 ms (1.23 ms, 1.26 ms) | 19.00 ns/op | 0.44x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.55 ms (1.54 ms, 1.56 ms) | 23.66 ns/op | 0.36x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.00 ms (1.99 ms, 2.01 ms) | 30.49 ns/op | 0.28x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 289.66 µs (279.50 µs, 300.64 µs) | 4.42 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 964.85 µs (962.42 µs, 967.38 µs) | 14.72 ns/op | 0.30x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.20 ms (1.20 ms, 1.21 ms) | 18.36 ns/op | 0.24x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.67 ms (1.66 ms, 1.67 ms) | 25.42 ns/op | 0.17x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 94.66 µs (94.43 µs, 94.91 µs) | 1.44 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 496.45 µs (494.55 µs, 498.29 µs) | 7.58 ns/op | 0.19x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloomfilter | 593.50 µs (590.78 µs, 596.12 µs) | 9.06 ns/op | 0.16x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 839.41 µs (837.46 µs, 842.01 µs) | 12.81 ns/op | 0.11x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 103.72 µs (103.47 µs, 104.01 µs) | 1.58 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 511.53 µs (508.23 µs, 515.29 µs) | 7.81 ns/op | 0.20x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloomfilter | 591.18 µs (589.19 µs, 593.23 µs) | 9.02 ns/op | 0.18x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 748.96 µs (745.59 µs, 752.87 µs) | 11.43 ns/op | 0.14x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 284.43 µs (259.48 µs, 306.33 µs) | 4.34 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 941.40 µs (936.30 µs, 947.57 µs) | 14.36 ns/op | 0.30x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloomfilter | 1.10 ms (1.10 ms, 1.11 ms) | 16.83 ns/op | 0.26x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.42 ms (1.41 ms, 1.43 ms) | 21.70 ns/op | 0.20x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 105.36 µs (103.04 µs, 108.36 µs) | 1.61 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 725.86 µs (722.90 µs, 729.05 µs) | 11.08 ns/op | 0.15x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloomfilter | 859.69 µs (851.78 µs, 868.71 µs) | 13.12 ns/op | 0.12x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.09 ms (1.08 ms, 1.09 ms) | 16.56 ns/op | 0.10x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

