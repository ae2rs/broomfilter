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
| `large-65536-4bpi` | 4.00 | fastbloom | broomfilter | broomfilter | broomfilter-blocked | broomfilter |
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
| 1 | **broomfilter** | 972.73 ns (929.17 ns, 1.03 µs) | 7.60 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/build_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 1.28 µs (1.25 µs, 1.32 µs) | 10.00 ns/op | 0.76x | `broomfilter-blocked [fp=0.002450, fn=0]` | [plot](target/criterion/build_compact-128/broomfilter-blocked%20%5Bfp%3D0.002450%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.93 µs (1.92 µs, 1.94 µs) | 15.05 ns/op | 0.50x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/build_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 2.91 µs (2.90 µs, 2.92 µs) | 22.74 ns/op | 0.33x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/build_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 4.51 µs (4.49 µs, 4.53 µs) | 35.24 ns/op | 0.22x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/build_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 26.36 µs (25.07 µs, 28.02 µs) | 6.43 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_member_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 43.17 µs (42.23 µs, 43.71 µs) | 10.54 ns/op | 0.61x | `broomfilter-blocked [fp=0.002450, fn=0]` | [plot](target/criterion/contains_member_compact-128/broomfilter-blocked%20%5Bfp%3D0.002450%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 60.10 µs (59.80 µs, 60.40 µs) | 14.67 ns/op | 0.44x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_member_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 82.59 µs (82.18 µs, 83.05 µs) | 20.16 ns/op | 0.32x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_member_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 144.65 µs (144.24 µs, 145.08 µs) | 35.31 ns/op | 0.18x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_member_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 10.33 µs (9.58 µs, 11.11 µs) | 2.52 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_absent_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 36.58 µs (35.82 µs, 37.34 µs) | 8.93 ns/op | 0.28x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_absent_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | broomfilter-blocked | 45.68 µs (44.20 µs, 46.65 µs) | 11.15 ns/op | 0.23x | `broomfilter-blocked [fp=0.002450, fn=0]` | [plot](target/criterion/contains_absent_compact-128/broomfilter-blocked%20%5Bfp%3D0.002450%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 64.81 µs (61.42 µs, 68.75 µs) | 15.82 ns/op | 0.16x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_absent_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 96.00 µs (93.84 µs, 98.28 µs) | 23.44 ns/op | 0.11x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_absent_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 22.89 µs (21.19 µs, 24.85 µs) | 5.59 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 45.60 µs (44.07 µs, 46.71 µs) | 11.13 ns/op | 0.50x | `broomfilter-blocked [fp=0.002450, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/broomfilter-blocked%20%5Bfp%3D0.002450%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 65.24 µs (65.08 µs, 65.40 µs) | 15.93 ns/op | 0.35x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 89.53 µs (89.02 µs, 90.07 µs) | 21.86 ns/op | 0.26x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 139.39 µs (138.82 µs, 139.95 µs) | 34.03 ns/op | 0.16x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 25.92 µs (25.87 µs, 25.98 µs) | 6.33 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/build_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 42.41 µs (41.27 µs, 44.27 µs) | 10.35 ns/op | 0.61x | `broomfilter-blocked [fp=0.061950, fn=0]` | [plot](target/criterion/build_scale-4096/broomfilter-blocked%20%5Bfp%3D0.061950%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 61.35 µs (61.10 µs, 61.59 µs) | 14.98 ns/op | 0.42x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/build_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 92.23 µs (92.08 µs, 92.42 µs) | 22.52 ns/op | 0.28x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/build_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 143.88 µs (143.53 µs, 144.16 µs) | 35.13 ns/op | 0.18x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/build_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 95.02 µs (94.41 µs, 95.63 µs) | 5.80 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_member_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 184.85 µs (180.09 µs, 187.41 µs) | 11.28 ns/op | 0.51x | `broomfilter-blocked [fp=0.061950, fn=0]` | [plot](target/criterion/contains_member_scale-4096/broomfilter-blocked%20%5Bfp%3D0.061950%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 245.26 µs (244.61 µs, 245.96 µs) | 14.97 ns/op | 0.39x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_member_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 340.18 µs (337.80 µs, 342.16 µs) | 20.76 ns/op | 0.28x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_member_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 580.29 µs (579.27 µs, 581.43 µs) | 35.42 ns/op | 0.16x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_member_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 46.02 µs (41.92 µs, 50.29 µs) | 2.81 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 186.14 µs (180.85 µs, 189.17 µs) | 11.36 ns/op | 0.25x | `broomfilter-blocked [fp=0.061950, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/broomfilter-blocked%20%5Bfp%3D0.061950%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 190.45 µs (183.43 µs, 198.60 µs) | 11.62 ns/op | 0.24x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 333.34 µs (330.44 µs, 336.59 µs) | 20.35 ns/op | 0.14x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 454.26 µs (451.88 µs, 456.72 µs) | 27.73 ns/op | 0.10x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 102.55 µs (100.38 µs, 104.76 µs) | 6.26 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 186.21 µs (181.63 µs, 189.00 µs) | 11.37 ns/op | 0.55x | `broomfilter-blocked [fp=0.061950, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/broomfilter-blocked%20%5Bfp%3D0.061950%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 286.72 µs (286.03 µs, 287.26 µs) | 17.50 ns/op | 0.36x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 361.39 µs (360.69 µs, 362.00 µs) | 22.06 ns/op | 0.28x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 557.77 µs (556.41 µs, 558.97 µs) | 34.04 ns/op | 0.18x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 438.93 µs (438.07 µs, 439.88 µs) | 6.70 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 676.14 µs (662.40 µs, 699.32 µs) | 10.32 ns/op | 0.65x | `broomfilter-blocked [fp=0.213350, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/broomfilter-blocked%20%5Bfp%3D0.213350%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 982.56 µs (978.43 µs, 986.53 µs) | 14.99 ns/op | 0.45x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.50 ms (1.50 ms, 1.50 ms) | 22.87 ns/op | 0.29x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 2.32 ms (2.31 ms, 2.33 ms) | 35.38 ns/op | 0.19x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 403.56 µs (402.37 µs, 404.78 µs) | 6.16 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 738.63 µs (717.47 µs, 749.77 µs) | 11.27 ns/op | 0.55x | `broomfilter-blocked [fp=0.213350, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/broomfilter-blocked%20%5Bfp%3D0.213350%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 975.20 µs (973.34 µs, 977.55 µs) | 14.88 ns/op | 0.41x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.30 ms (1.29 ms, 1.32 ms) | 19.87 ns/op | 0.31x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 2.29 ms (2.28 ms, 2.30 ms) | 34.90 ns/op | 0.18x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 645.27 µs (636.01 µs, 654.72 µs) | 9.85 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 742.65 µs (723.50 µs, 753.22 µs) | 11.33 ns/op | 0.87x | `broomfilter-blocked [fp=0.213350, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/broomfilter-blocked%20%5Bfp%3D0.213350%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.23 ms (1.21 ms, 1.24 ms) | 18.71 ns/op | 0.53x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.54 ms (1.54 ms, 1.55 ms) | 23.56 ns/op | 0.42x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 2.10 ms (2.10 ms, 2.11 ms) | 32.10 ns/op | 0.31x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 551.76 µs (549.11 µs, 554.90 µs) | 8.42 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 740.29 µs (717.98 µs, 752.74 µs) | 11.30 ns/op | 0.75x | `broomfilter-blocked [fp=0.213350, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/broomfilter-blocked%20%5Bfp%3D0.213350%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.18 ms (1.18 ms, 1.19 ms) | 18.07 ns/op | 0.47x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.44 ms (1.44 ms, 1.45 ms) | 22.04 ns/op | 0.38x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 2.25 ms (2.24 ms, 2.26 ms) | 34.30 ns/op | 0.25x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 248.61 µs (247.80 µs, 249.51 µs) | 3.79 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 442.07 µs (430.47 µs, 464.58 µs) | 6.75 ns/op | 0.56x | `broomfilter-blocked [fp=0.320820, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/broomfilter-blocked%20%5Bfp%3D0.320820%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 708.68 µs (707.58 µs, 710.22 µs) | 10.81 ns/op | 0.35x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.21 ms (1.21 ms, 1.22 ms) | 18.52 ns/op | 0.20x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 1.78 ms (1.78 ms, 1.79 ms) | 27.16 ns/op | 0.14x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 233.52 µs (232.93 µs, 234.12 µs) | 3.56 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 624.45 µs (598.72 µs, 637.93 µs) | 9.53 ns/op | 0.37x | `broomfilter-blocked [fp=0.320820, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/broomfilter-blocked%20%5Bfp%3D0.320820%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 735.59 µs (731.71 µs, 738.43 µs) | 11.22 ns/op | 0.32x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 996.65 µs (992.74 µs, 1.00 ms) | 15.21 ns/op | 0.23x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 1.77 ms (1.76 ms, 1.78 ms) | 26.98 ns/op | 0.13x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 603.14 µs (588.45 µs, 617.75 µs) | 9.20 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 628.32 µs (597.66 µs, 649.75 µs) | 9.59 ns/op | 0.96x | `broomfilter-blocked [fp=0.320820, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/broomfilter-blocked%20%5Bfp%3D0.320820%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.26 ms (1.24 ms, 1.28 ms) | 19.20 ns/op | 0.48x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.60 ms (1.59 ms, 1.60 ms) | 24.36 ns/op | 0.38x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 2.15 ms (2.14 ms, 2.16 ms) | 32.78 ns/op | 0.28x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 396.86 µs (390.65 µs, 403.07 µs) | 6.06 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 620.35 µs (592.62 µs, 636.81 µs) | 9.47 ns/op | 0.64x | `broomfilter-blocked [fp=0.320820, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/broomfilter-blocked%20%5Bfp%3D0.320820%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.08 ms (1.07 ms, 1.09 ms) | 16.46 ns/op | 0.37x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.35 ms (1.34 ms, 1.36 ms) | 20.56 ns/op | 0.29x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 1.99 ms (1.98 ms, 2.00 ms) | 30.39 ns/op | 0.20x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 159.25 µs (158.89 µs, 159.53 µs) | 2.43 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 365.88 µs (356.56 µs, 383.95 µs) | 5.58 ns/op | 0.44x | `broomfilter-blocked [fp=0.395310, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/broomfilter-blocked%20%5Bfp%3D0.395310%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 585.42 µs (584.12 µs, 586.64 µs) | 8.93 ns/op | 0.27x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 899.98 µs (898.14 µs, 901.97 µs) | 13.73 ns/op | 0.18x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 1.34 ms (1.34 ms, 1.35 ms) | 20.50 ns/op | 0.12x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 151.84 µs (151.27 µs, 152.66 µs) | 2.32 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 450.43 µs (437.90 µs, 457.13 µs) | 6.87 ns/op | 0.34x | `broomfilter-blocked [fp=0.395310, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/broomfilter-blocked%20%5Bfp%3D0.395310%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 586.47 µs (584.80 µs, 588.54 µs) | 8.95 ns/op | 0.26x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 797.82 µs (795.66 µs, 799.90 µs) | 12.17 ns/op | 0.19x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 1.31 ms (1.31 ms, 1.32 ms) | 20.06 ns/op | 0.12x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | broomfilter-blocked | 449.57 µs (437.45 µs, 456.37 µs) | 6.86 ns/op | 1.22x | `broomfilter-blocked [fp=0.395310, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/broomfilter-blocked%20%5Bfp%3D0.395310%2C%20fn%3D0%5D/report/index.html) |
| 2 | **broomfilter** | 549.80 µs (526.88 µs, 570.56 µs) | 8.39 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 1.23 ms (1.22 ms, 1.25 ms) | 18.79 ns/op | 0.45x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.57 ms (1.56 ms, 1.58 ms) | 23.94 ns/op | 0.35x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 2.03 ms (2.02 ms, 2.04 ms) | 30.99 ns/op | 0.27x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 320.11 µs (314.11 µs, 326.08 µs) | 4.88 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 452.03 µs (439.37 µs, 458.85 µs) | 6.90 ns/op | 0.71x | `broomfilter-blocked [fp=0.395310, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/broomfilter-blocked%20%5Bfp%3D0.395310%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 980.15 µs (975.27 µs, 984.99 µs) | 14.96 ns/op | 0.33x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.21 ms (1.20 ms, 1.22 ms) | 18.47 ns/op | 0.26x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 1.68 ms (1.67 ms, 1.69 ms) | 25.63 ns/op | 0.19x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 99.07 µs (98.90 µs, 99.25 µs) | 1.51 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 244.24 µs (240.72 µs, 249.83 µs) | 3.73 ns/op | 0.41x | `broomfilter-blocked [fp=0.632020, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/broomfilter-blocked%20%5Bfp%3D0.632020%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 498.11 µs (496.82 µs, 499.40 µs) | 7.60 ns/op | 0.20x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 592.08 µs (589.76 µs, 594.49 µs) | 9.03 ns/op | 0.17x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloom | 834.81 µs (832.50 µs, 837.33 µs) | 12.74 ns/op | 0.12x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 103.12 µs (102.84 µs, 103.47 µs) | 1.57 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 336.10 µs (326.60 µs, 342.26 µs) | 5.13 ns/op | 0.31x | `broomfilter-blocked [fp=0.632020, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/broomfilter-blocked%20%5Bfp%3D0.632020%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 513.08 µs (510.48 µs, 515.79 µs) | 7.83 ns/op | 0.20x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 591.28 µs (589.67 µs, 593.09 µs) | 9.02 ns/op | 0.17x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloom | 756.80 µs (754.48 µs, 759.01 µs) | 11.55 ns/op | 0.14x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 220.56 µs (190.39 µs, 250.47 µs) | 3.37 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 334.68 µs (324.29 µs, 340.42 µs) | 5.11 ns/op | 0.66x | `broomfilter-blocked [fp=0.632020, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/broomfilter-blocked%20%5Bfp%3D0.632020%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 962.46 µs (960.03 µs, 965.19 µs) | 14.69 ns/op | 0.23x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.12 ms (1.12 ms, 1.12 ms) | 17.08 ns/op | 0.20x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloom | 1.42 ms (1.41 ms, 1.43 ms) | 21.68 ns/op | 0.16x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 171.60 µs (159.67 µs, 184.31 µs) | 2.62 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | broomfilter-blocked | 335.53 µs (324.62 µs, 341.55 µs) | 5.12 ns/op | 0.51x | `broomfilter-blocked [fp=0.632020, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/broomfilter-blocked%20%5Bfp%3D0.632020%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 747.13 µs (740.69 µs, 753.13 µs) | 11.40 ns/op | 0.23x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 859.03 µs (855.05 µs, 863.24 µs) | 13.11 ns/op | 0.20x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloom | 1.09 ms (1.08 ms, 1.09 ms) | 16.57 ns/op | 0.16x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

