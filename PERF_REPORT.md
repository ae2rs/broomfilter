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
| 1 | **broomfilter** | 734.89 ns (729.68 ns, 740.97 ns) | 5.74 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/build_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.81 µs (1.80 µs, 1.82 µs) | 14.12 ns/op | 0.41x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/build_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 2.86 µs (2.84 µs, 2.87 µs) | 22.31 ns/op | 0.26x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/build_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 4.39 µs (4.37 µs, 4.42 µs) | 34.31 ns/op | 0.17x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/build_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 24.93 µs (24.79 µs, 25.06 µs) | 6.09 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_member_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 56.75 µs (56.37 µs, 57.18 µs) | 13.85 ns/op | 0.44x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_member_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 81.61 µs (80.77 µs, 82.50 µs) | 19.92 ns/op | 0.31x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_member_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 140.13 µs (139.28 µs, 141.08 µs) | 34.21 ns/op | 0.18x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_member_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 8.42 µs (8.37 µs, 8.48 µs) | 2.06 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_absent_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 34.38 µs (34.11 µs, 34.66 µs) | 8.39 ns/op | 0.25x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_absent_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 68.50 µs (66.70 µs, 70.64 µs) | 16.72 ns/op | 0.12x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_absent_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 93.49 µs (92.45 µs, 94.66 µs) | 22.83 ns/op | 0.09x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_absent_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 2048-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 17.62 µs (17.47 µs, 17.77 µs) | 4.30 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 61.75 µs (61.34 µs, 62.24 µs) | 15.08 ns/op | 0.29x | `fastbloom [fp=0.000400, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/fastbloom%20%5Bfp%3D0.000400%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 87.91 µs (87.13 µs, 88.51 µs) | 21.46 ns/op | 0.20x | `bloom [fp=0.011540, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/bloom%20%5Bfp%3D0.011540%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 135.04 µs (133.93 µs, 136.74 µs) | 32.97 ns/op | 0.13x | `bloomfilter [fp=0.000750, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/bloomfilter%20%5Bfp%3D0.000750%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 22.35 µs (22.23 µs, 22.49 µs) | 5.46 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/build_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 58.32 µs (57.92 µs, 58.62 µs) | 14.24 ns/op | 0.38x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/build_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 92.00 µs (91.44 µs, 92.67 µs) | 22.46 ns/op | 0.24x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/build_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 141.74 µs (141.32 µs, 142.14 µs) | 34.60 ns/op | 0.16x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/build_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 104.13 µs (103.55 µs, 104.79 µs) | 6.36 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_member_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 235.73 µs (235.00 µs, 236.48 µs) | 14.39 ns/op | 0.44x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_member_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 335.50 µs (333.63 µs, 337.28 µs) | 20.48 ns/op | 0.31x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_member_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 573.18 µs (570.13 µs, 576.14 µs) | 34.98 ns/op | 0.18x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_member_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 50.40 µs (45.66 µs, 55.01 µs) | 3.08 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 165.59 µs (157.74 µs, 175.62 µs) | 10.11 ns/op | 0.30x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 333.14 µs (329.91 µs, 336.59 µs) | 20.33 ns/op | 0.15x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 457.19 µs (455.00 µs, 459.39 µs) | 27.90 ns/op | 0.11x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 65536-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 101.47 µs (96.76 µs, 106.31 µs) | 6.19 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 275.96 µs (273.87 µs, 277.96 µs) | 16.84 ns/op | 0.37x | `fastbloom [fp=0.000510, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/fastbloom%20%5Bfp%3D0.000510%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 362.04 µs (359.98 µs, 363.89 µs) | 22.10 ns/op | 0.28x | `bloom [fp=0.009780, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/bloom%20%5Bfp%3D0.009780%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 543.13 µs (541.16 µs, 545.16 µs) | 33.15 ns/op | 0.19x | `bloomfilter [fp=0.000410, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/bloomfilter%20%5Bfp%3D0.000410%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 401.27 µs (399.74 µs, 402.91 µs) | 6.12 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 979.88 µs (975.21 µs, 984.66 µs) | 14.95 ns/op | 0.41x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.50 ms (1.50 ms, 1.51 ms) | 22.96 ns/op | 0.27x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.30 ms (2.30 ms, 2.31 ms) | 35.13 ns/op | 0.17x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/build_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 427.72 µs (424.72 µs, 430.03 µs) | 6.53 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 968.37 µs (962.84 µs, 976.15 µs) | 14.78 ns/op | 0.44x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.33 ms (1.32 ms, 1.35 ms) | 20.36 ns/op | 0.32x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.29 ms (2.29 ms, 2.31 ms) | 35.02 ns/op | 0.19x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_member_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 618.02 µs (608.97 µs, 627.43 µs) | 9.43 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.23 ms (1.20 ms, 1.26 ms) | 18.73 ns/op | 0.50x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.58 ms (1.58 ms, 1.59 ms) | 24.17 ns/op | 0.39x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.07 ms (2.06 ms, 2.07 ms) | 31.53 ns/op | 0.30x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_absent_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 1048576-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 566.33 µs (560.68 µs, 573.00 µs) | 8.64 ns/op | 1.00x | `broomfilter [fp=0.000560, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/broomfilter%20%5Bfp%3D0.000560%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.16 ms (1.15 ms, 1.16 ms) | 17.63 ns/op | 0.49x | `fastbloom [fp=0.000500, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/fastbloom%20%5Bfp%3D0.000500%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.46 ms (1.45 ms, 1.46 ms) | 22.20 ns/op | 0.39x | `bloom [fp=0.010470, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/bloom%20%5Bfp%3D0.010470%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.27 ms (2.25 ms, 2.30 ms) | 34.66 ns/op | 0.25x | `bloomfilter [fp=0.000550, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-16bpi/bloomfilter%20%5Bfp%3D0.000550%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 230.50 µs (229.67 µs, 231.45 µs) | 3.52 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 703.82 µs (701.06 µs, 707.01 µs) | 10.74 ns/op | 0.33x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.20 ms (1.19 ms, 1.21 ms) | 18.34 ns/op | 0.19x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.76 ms (1.76 ms, 1.77 ms) | 26.90 ns/op | 0.13x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/build_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 252.96 µs (251.57 µs, 254.29 µs) | 3.86 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 722.11 µs (719.52 µs, 725.00 µs) | 11.02 ns/op | 0.35x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 980.28 µs (976.46 µs, 985.82 µs) | 14.96 ns/op | 0.26x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.75 ms (1.75 ms, 1.76 ms) | 26.72 ns/op | 0.14x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_member_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 606.31 µs (592.51 µs, 620.21 µs) | 9.25 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.24 ms (1.22 ms, 1.26 ms) | 18.95 ns/op | 0.49x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.62 ms (1.61 ms, 1.63 ms) | 24.70 ns/op | 0.37x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 2.11 ms (2.11 ms, 2.12 ms) | 32.27 ns/op | 0.29x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_absent_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 524288-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 403.98 µs (396.65 µs, 411.25 µs) | 6.16 ns/op | 1.00x | `broomfilter [fp=0.021090, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/broomfilter%20%5Bfp%3D0.021090%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.05 ms (1.05 ms, 1.06 ms) | 16.04 ns/op | 0.38x | `fastbloom [fp=0.022180, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/fastbloom%20%5Bfp%3D0.022180%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.33 ms (1.33 ms, 1.34 ms) | 20.34 ns/op | 0.30x | `bloom [fp=0.045150, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/bloom%20%5Bfp%3D0.045150%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.95 ms (1.94 ms, 1.95 ms) | 29.70 ns/op | 0.21x | `bloomfilter [fp=0.021520, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-8bpi/bloomfilter%20%5Bfp%3D0.021520%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 147.59 µs (147.23 µs, 148.12 µs) | 2.25 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 576.71 µs (574.93 µs, 578.71 µs) | 8.80 ns/op | 0.26x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 912.18 µs (906.20 µs, 918.26 µs) | 13.92 ns/op | 0.16x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.32 ms (1.31 ms, 1.33 ms) | 20.12 ns/op | 0.11x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/build_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 161.11 µs (160.25 µs, 162.18 µs) | 2.46 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 580.67 µs (578.65 µs, 583.40 µs) | 8.86 ns/op | 0.28x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 794.66 µs (793.25 µs, 796.13 µs) | 12.13 ns/op | 0.20x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.30 ms (1.30 ms, 1.31 ms) | 19.89 ns/op | 0.12x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_member_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 523.03 µs (511.95 µs, 535.55 µs) | 7.98 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 1.22 ms (1.19 ms, 1.25 ms) | 18.60 ns/op | 0.43x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.58 ms (1.58 ms, 1.59 ms) | 24.14 ns/op | 0.33x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.99 ms (1.98 ms, 2.00 ms) | 30.42 ns/op | 0.26x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_absent_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 262144-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 253.39 µs (252.03 µs, 255.11 µs) | 3.87 ns/op | 1.00x | `broomfilter [fp=0.145480, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/broomfilter%20%5Bfp%3D0.145480%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 965.87 µs (961.85 µs, 970.52 µs) | 14.74 ns/op | 0.26x | `fastbloom [fp=0.145090, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/fastbloom%20%5Bfp%3D0.145090%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloom | 1.19 ms (1.19 ms, 1.20 ms) | 18.22 ns/op | 0.21x | `bloom [fp=0.158800, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/bloom%20%5Bfp%3D0.158800%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloomfilter | 1.67 ms (1.66 ms, 1.68 ms) | 25.41 ns/op | 0.15x | `bloomfilter [fp=0.145890, fn=0]` | [plot](target/criterion/contains_mixed_large-65536-4bpi/bloomfilter%20%5Bfp%3D0.145890%2C%20fn%3D0%5D/report/index.html) |

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
| 1 | **broomfilter** | 96.60 µs (94.42 µs, 99.37 µs) | 1.47 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 494.06 µs (491.24 µs, 497.45 µs) | 7.54 ns/op | 0.20x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloomfilter | 595.85 µs (591.70 µs, 599.99 µs) | 9.09 ns/op | 0.16x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 834.42 µs (831.21 µs, 838.17 µs) | 12.73 ns/op | 0.12x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/build_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 103.79 µs (103.48 µs, 104.16 µs) | 1.58 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 508.68 µs (506.07 µs, 511.34 µs) | 7.76 ns/op | 0.20x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloomfilter | 591.90 µs (590.07 µs, 593.72 µs) | 9.03 ns/op | 0.18x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 753.84 µs (750.95 µs, 757.22 µs) | 11.50 ns/op | 0.14x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_member_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 270.46 µs (253.28 µs, 284.67 µs) | 4.13 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 956.81 µs (949.98 µs, 964.21 µs) | 14.60 ns/op | 0.28x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloomfilter | 1.13 ms (1.12 ms, 1.13 ms) | 17.18 ns/op | 0.24x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.44 ms (1.43 ms, 1.45 ms) | 21.96 ns/op | 0.19x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_absent_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. All filters use the same 131072-bit memory budget in this scenario. Build is normalized per inserted item; contains workloads are normalized per query over 65536 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 181.56 µs (176.99 µs, 186.89 µs) | 2.77 ns/op | 1.00x | `broomfilter [fp=0.393160, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/broomfilter%20%5Bfp%3D0.393160%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 727.46 µs (722.29 µs, 733.05 µs) | 11.10 ns/op | 0.25x | `fastbloom [fp=0.393950, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/fastbloom%20%5Bfp%3D0.393950%2C%20fn%3D0%5D/report/index.html) |
| 3 | bloomfilter | 840.16 µs (836.57 µs, 843.79 µs) | 12.82 ns/op | 0.22x | `bloomfilter [fp=0.394820, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/bloomfilter%20%5Bfp%3D0.394820%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 1.10 ms (1.09 ms, 1.11 ms) | 16.78 ns/op | 0.17x | `bloom [fp=0.400070, fn=0]` | [plot](target/criterion/contains_mixed_stress-65536-2bpi/bloom%20%5Bfp%3D0.400070%2C%20fn%3D0%5D/report/index.html) |

