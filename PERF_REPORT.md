# Performance Report

Generated from the Criterion benchmark suite in `benches/basic.rs`. This report compares throughput-oriented timings with measured precision for `broomfilter` and the reference crates in the same scenarios.

- Overall report: [Criterion dashboard](target/criterion/report/index.html)
- Benchmarks covered: build, present lookups, absent lookups, and mixed lookups
- Precision covered: false negatives on inserted keys and false positives over 100,000 deterministic absent-key probes per scenario

## Scenario Summary

| Scenario | Most precise | Fastest build | Fastest present lookup | Fastest absent lookup | Fastest mixed lookup |
| --- | --- | --- | --- | --- | --- |
| `compact-128` | broomfilter | broomfilter | xorf | xorf | xorf |
| `scale-4096` | broomfilter | broomfilter | xorf | xorf | xorf |

## Scenario `compact-128`

- Inserted items: 128
- Query batch size: 4096
- Precision probes: 100000
- Target false-positive rate for configurable filters: 0.0050

### Precision

| Library | False negatives | False positives | FP rate | Config |
| --- | ---: | ---: | ---: | --- |
| **broomfilter** | 0 | 82 | 0.000820 | 2^11 bits |
| qfilter | 0 | 177 | 0.001770 | target fp=0.0050 |
| xorf | 0 | 379 | 0.003790 | BinaryFuse8 (~0.39% fp) |
| bloomfilter | 0 | 405 | 0.004050 | target fp=0.0050 |
| fastbloom | 0 | 460 | 0.004600 | target fp=0.0050 |
| ethbloom | 0 | 534 | 0.005340 | fixed 2048-bit bloom |
| bloom | 0 | 1363 | 0.013630 | target fp=0.0050 |
| cuckoofilter | 0 | 1591 | 0.015910 | capacity = 2x inserted |

### Build

Lower is better. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 1.07 µs (1.06 µs, 1.08 µs) | 8.35 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/build_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 2.01 µs (1.99 µs, 2.04 µs) | 15.73 ns/op | 0.53x | `fastbloom [fp=0.004600, fn=0]` | [plot](target/criterion/build_compact-128/fastbloom%20%5Bfp%3D0.004600%2C%20fn%3D0%5D/report/index.html) |
| 3 | xorf | 3.06 µs (3.06 µs, 3.06 µs) | 23.92 ns/op | 0.35x | `xorf [fp=0.003790, fn=0]` | [plot](target/criterion/build_compact-128/xorf%20%5Bfp%3D0.003790%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 3.25 µs (3.23 µs, 3.27 µs) | 25.38 ns/op | 0.33x | `bloom [fp=0.013630, fn=0]` | [plot](target/criterion/build_compact-128/bloom%20%5Bfp%3D0.013630%2C%20fn%3D0%5D/report/index.html) |
| 5 | cuckoofilter | 3.40 µs (3.33 µs, 3.47 µs) | 26.54 ns/op | 0.31x | `cuckoofilter [fp=0.015910, fn=0]` | [plot](target/criterion/build_compact-128/cuckoofilter%20%5Bfp%3D0.015910%2C%20fn%3D0%5D/report/index.html) |
| 6 | qfilter | 4.14 µs (4.09 µs, 4.18 µs) | 32.32 ns/op | 0.26x | `qfilter [fp=0.001770, fn=0]` | [plot](target/criterion/build_compact-128/qfilter%20%5Bfp%3D0.001770%2C%20fn%3D0%5D/report/index.html) |
| 7 | bloomfilter | 5.32 µs (5.26 µs, 5.37 µs) | 41.52 ns/op | 0.20x | `bloomfilter [fp=0.004050, fn=0]` | [plot](target/criterion/build_compact-128/bloomfilter%20%5Bfp%3D0.004050%2C%20fn%3D0%5D/report/index.html) |
| 8 | ethbloom | 22.84 µs (22.78 µs, 22.91 µs) | 178.47 ns/op | 0.05x | `ethbloom [fp=0.005340, fn=0]` | [plot](target/criterion/build_compact-128/ethbloom%20%5Bfp%3D0.005340%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | xorf | 6.68 µs (6.68 µs, 6.68 µs) | 1.63 ns/op | 4.34x | `xorf [fp=0.003790, fn=0]` | [plot](target/criterion/contains_member_compact-128/xorf%20%5Bfp%3D0.003790%2C%20fn%3D0%5D/report/index.html) |
| 2 | **broomfilter** | 29.01 µs (28.82 µs, 29.20 µs) | 7.08 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_member_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 56.44 µs (56.06 µs, 56.82 µs) | 13.78 ns/op | 0.51x | `fastbloom [fp=0.004600, fn=0]` | [plot](target/criterion/contains_member_compact-128/fastbloom%20%5Bfp%3D0.004600%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 100.74 µs (100.73 µs, 100.74 µs) | 24.59 ns/op | 0.29x | `bloom [fp=0.013630, fn=0]` | [plot](target/criterion/contains_member_compact-128/bloom%20%5Bfp%3D0.013630%2C%20fn%3D0%5D/report/index.html) |
| 5 | cuckoofilter | 103.59 µs (102.30 µs, 104.87 µs) | 25.29 ns/op | 0.28x | `cuckoofilter [fp=0.015910, fn=0]` | [plot](target/criterion/contains_member_compact-128/cuckoofilter%20%5Bfp%3D0.015910%2C%20fn%3D0%5D/report/index.html) |
| 6 | bloomfilter | 137.85 µs (137.12 µs, 138.58 µs) | 33.66 ns/op | 0.21x | `bloomfilter [fp=0.004050, fn=0]` | [plot](target/criterion/contains_member_compact-128/bloomfilter%20%5Bfp%3D0.004050%2C%20fn%3D0%5D/report/index.html) |
| 7 | qfilter | 154.45 µs (153.77 µs, 155.13 µs) | 37.71 ns/op | 0.19x | `qfilter [fp=0.001770, fn=0]` | [plot](target/criterion/contains_member_compact-128/qfilter%20%5Bfp%3D0.001770%2C%20fn%3D0%5D/report/index.html) |
| 8 | ethbloom | 1.07 ms (1.07 ms, 1.08 ms) | 261.76 ns/op | 0.03x | `ethbloom [fp=0.005340, fn=0]` | [plot](target/criterion/contains_member_compact-128/ethbloom%20%5Bfp%3D0.005340%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | xorf | 6.71 µs (6.68 µs, 6.74 µs) | 1.64 ns/op | 1.53x | `xorf [fp=0.003790, fn=0]` | [plot](target/criterion/contains_absent_compact-128/xorf%20%5Bfp%3D0.003790%2C%20fn%3D0%5D/report/index.html) |
| 2 | **broomfilter** | 10.28 µs (10.23 µs, 10.33 µs) | 2.51 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_absent_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 38.21 µs (37.47 µs, 38.95 µs) | 9.33 ns/op | 0.27x | `fastbloom [fp=0.004600, fn=0]` | [plot](target/criterion/contains_absent_compact-128/fastbloom%20%5Bfp%3D0.004600%2C%20fn%3D0%5D/report/index.html) |
| 4 | cuckoofilter | 100.73 µs (100.65 µs, 100.82 µs) | 24.59 ns/op | 0.10x | `cuckoofilter [fp=0.015910, fn=0]` | [plot](target/criterion/contains_absent_compact-128/cuckoofilter%20%5Bfp%3D0.015910%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 112.62 µs (109.69 µs, 115.55 µs) | 27.50 ns/op | 0.09x | `bloomfilter [fp=0.004050, fn=0]` | [plot](target/criterion/contains_absent_compact-128/bloomfilter%20%5Bfp%3D0.004050%2C%20fn%3D0%5D/report/index.html) |
| 6 | qfilter | 117.02 µs (115.94 µs, 118.11 µs) | 28.57 ns/op | 0.09x | `qfilter [fp=0.001770, fn=0]` | [plot](target/criterion/contains_absent_compact-128/qfilter%20%5Bfp%3D0.001770%2C%20fn%3D0%5D/report/index.html) |
| 7 | bloom | 118.70 µs (116.62 µs, 120.78 µs) | 28.98 ns/op | 0.09x | `bloom [fp=0.013630, fn=0]` | [plot](target/criterion/contains_absent_compact-128/bloom%20%5Bfp%3D0.013630%2C%20fn%3D0%5D/report/index.html) |
| 8 | ethbloom | 834.20 µs (815.03 µs, 853.37 µs) | 203.66 ns/op | 0.01x | `ethbloom [fp=0.005340, fn=0]` | [plot](target/criterion/contains_absent_compact-128/ethbloom%20%5Bfp%3D0.005340%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. Build is normalized per inserted item; contains workloads are normalized per query over 4096 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | xorf | 6.77 µs (6.76 µs, 6.78 µs) | 1.65 ns/op | 3.48x | `xorf [fp=0.003790, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/xorf%20%5Bfp%3D0.003790%2C%20fn%3D0%5D/report/index.html) |
| 2 | **broomfilter** | 23.56 µs (23.42 µs, 23.69 µs) | 5.75 ns/op | 1.00x | `broomfilter [fp=0.000820, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/broomfilter%20%5Bfp%3D0.000820%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 69.84 µs (69.30 µs, 70.37 µs) | 17.05 ns/op | 0.34x | `fastbloom [fp=0.004600, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/fastbloom%20%5Bfp%3D0.004600%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 117.93 µs (117.81 µs, 118.06 µs) | 28.79 ns/op | 0.20x | `bloom [fp=0.013630, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/bloom%20%5Bfp%3D0.013630%2C%20fn%3D0%5D/report/index.html) |
| 5 | cuckoofilter | 128.82 µs (127.49 µs, 130.15 µs) | 31.45 ns/op | 0.18x | `cuckoofilter [fp=0.015910, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/cuckoofilter%20%5Bfp%3D0.015910%2C%20fn%3D0%5D/report/index.html) |
| 6 | bloomfilter | 139.20 µs (138.78 µs, 139.62 µs) | 33.98 ns/op | 0.17x | `bloomfilter [fp=0.004050, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/bloomfilter%20%5Bfp%3D0.004050%2C%20fn%3D0%5D/report/index.html) |
| 7 | qfilter | 144.14 µs (141.98 µs, 146.31 µs) | 35.19 ns/op | 0.16x | `qfilter [fp=0.001770, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/qfilter%20%5Bfp%3D0.001770%2C%20fn%3D0%5D/report/index.html) |
| 8 | ethbloom | 961.34 µs (958.97 µs, 963.71 µs) | 234.70 ns/op | 0.02x | `ethbloom [fp=0.005340, fn=0]` | [plot](target/criterion/contains_mixed_compact-128/ethbloom%20%5Bfp%3D0.005340%2C%20fn%3D0%5D/report/index.html) |

## Scenario `scale-4096`

- Inserted items: 4096
- Query batch size: 16384
- Precision probes: 100000
- Target false-positive rate for configurable filters: 0.0050

### Precision

| Library | False negatives | False positives | FP rate | Config |
| --- | ---: | ---: | ---: | --- |
| **broomfilter** | 0 | 47 | 0.000470 | 2^16 bits |
| qfilter | 0 | 186 | 0.001860 | target fp=0.0050 |
| xorf | 0 | 388 | 0.003880 | BinaryFuse8 (~0.39% fp) |
| bloomfilter | 0 | 473 | 0.004730 | target fp=0.0050 |
| fastbloom | 0 | 484 | 0.004840 | target fp=0.0050 |
| bloom | 0 | 1095 | 0.010950 | target fp=0.0050 |
| cuckoofilter | 0 | 1548 | 0.015480 | capacity = 2x inserted |
| ethbloom | 0 | 99730 | 0.997300 | fixed 2048-bit bloom |

### Build

Lower is better. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | **broomfilter** | 33.64 µs (33.63 µs, 33.64 µs) | 8.21 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/build_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 2 | fastbloom | 56.31 µs (55.79 µs, 56.84 µs) | 13.75 ns/op | 0.60x | `fastbloom [fp=0.004840, fn=0]` | [plot](target/criterion/build_scale-4096/fastbloom%20%5Bfp%3D0.004840%2C%20fn%3D0%5D/report/index.html) |
| 3 | xorf | 78.72 µs (77.89 µs, 79.54 µs) | 19.22 ns/op | 0.43x | `xorf [fp=0.003880, fn=0]` | [plot](target/criterion/build_scale-4096/xorf%20%5Bfp%3D0.003880%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 104.36 µs (104.19 µs, 104.54 µs) | 25.48 ns/op | 0.32x | `bloom [fp=0.010950, fn=0]` | [plot](target/criterion/build_scale-4096/bloom%20%5Bfp%3D0.010950%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloomfilter | 133.38 µs (132.04 µs, 134.72 µs) | 32.56 ns/op | 0.25x | `bloomfilter [fp=0.004730, fn=0]` | [plot](target/criterion/build_scale-4096/bloomfilter%20%5Bfp%3D0.004730%2C%20fn%3D0%5D/report/index.html) |
| 6 | cuckoofilter | 134.67 µs (132.50 µs, 136.84 µs) | 32.88 ns/op | 0.25x | `cuckoofilter [fp=0.015480, fn=0]` | [plot](target/criterion/build_scale-4096/cuckoofilter%20%5Bfp%3D0.015480%2C%20fn%3D0%5D/report/index.html) |
| 7 | qfilter | 145.62 µs (145.18 µs, 146.07 µs) | 35.55 ns/op | 0.23x | `qfilter [fp=0.001860, fn=0]` | [plot](target/criterion/build_scale-4096/qfilter%20%5Bfp%3D0.001860%2C%20fn%3D0%5D/report/index.html) |
| 8 | ethbloom | 733.45 µs (732.55 µs, 734.35 µs) | 179.07 ns/op | 0.05x | `ethbloom [fp=0.997300, fn=0]` | [plot](target/criterion/build_scale-4096/ethbloom%20%5Bfp%3D0.997300%2C%20fn%3D0%5D/report/index.html) |

### Contains present keys

Lower is better. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | xorf | 27.21 µs (26.97 µs, 27.46 µs) | 1.66 ns/op | 4.35x | `xorf [fp=0.003880, fn=0]` | [plot](target/criterion/contains_member_scale-4096/xorf%20%5Bfp%3D0.003880%2C%20fn%3D0%5D/report/index.html) |
| 2 | **broomfilter** | 118.36 µs (117.87 µs, 118.85 µs) | 7.22 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_member_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 232.25 µs (229.67 µs, 234.83 µs) | 14.18 ns/op | 0.51x | `fastbloom [fp=0.004840, fn=0]` | [plot](target/criterion/contains_member_scale-4096/fastbloom%20%5Bfp%3D0.004840%2C%20fn%3D0%5D/report/index.html) |
| 4 | bloom | 403.73 µs (402.88 µs, 404.58 µs) | 24.64 ns/op | 0.29x | `bloom [fp=0.010950, fn=0]` | [plot](target/criterion/contains_member_scale-4096/bloom%20%5Bfp%3D0.010950%2C%20fn%3D0%5D/report/index.html) |
| 5 | cuckoofilter | 494.58 µs (493.65 µs, 495.52 µs) | 30.19 ns/op | 0.24x | `cuckoofilter [fp=0.015480, fn=0]` | [plot](target/criterion/contains_member_scale-4096/cuckoofilter%20%5Bfp%3D0.015480%2C%20fn%3D0%5D/report/index.html) |
| 6 | bloomfilter | 534.21 µs (530.87 µs, 537.56 µs) | 32.61 ns/op | 0.22x | `bloomfilter [fp=0.004730, fn=0]` | [plot](target/criterion/contains_member_scale-4096/bloomfilter%20%5Bfp%3D0.004730%2C%20fn%3D0%5D/report/index.html) |
| 7 | qfilter | 644.55 µs (635.85 µs, 653.26 µs) | 39.34 ns/op | 0.18x | `qfilter [fp=0.001860, fn=0]` | [plot](target/criterion/contains_member_scale-4096/qfilter%20%5Bfp%3D0.001860%2C%20fn%3D0%5D/report/index.html) |
| 8 | ethbloom | 4.38 ms (4.30 ms, 4.46 ms) | 267.53 ns/op | 0.03x | `ethbloom [fp=0.997300, fn=0]` | [plot](target/criterion/contains_member_scale-4096/ethbloom%20%5Bfp%3D0.997300%2C%20fn%3D0%5D/report/index.html) |

### Contains absent keys

Lower is better. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | xorf | 27.28 µs (27.22 µs, 27.33 µs) | 1.66 ns/op | 5.08x | `xorf [fp=0.003880, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/xorf%20%5Bfp%3D0.003880%2C%20fn%3D0%5D/report/index.html) |
| 2 | **broomfilter** | 138.52 µs (138.09 µs, 138.96 µs) | 8.45 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 241.58 µs (239.62 µs, 243.54 µs) | 14.74 ns/op | 0.57x | `fastbloom [fp=0.004840, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/fastbloom%20%5Bfp%3D0.004840%2C%20fn%3D0%5D/report/index.html) |
| 4 | cuckoofilter | 419.02 µs (416.98 µs, 421.05 µs) | 25.57 ns/op | 0.33x | `cuckoofilter [fp=0.015480, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/cuckoofilter%20%5Bfp%3D0.015480%2C%20fn%3D0%5D/report/index.html) |
| 5 | qfilter | 467.36 µs (466.91 µs, 467.81 µs) | 28.53 ns/op | 0.30x | `qfilter [fp=0.001860, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/qfilter%20%5Bfp%3D0.001860%2C%20fn%3D0%5D/report/index.html) |
| 6 | bloom | 547.41 µs (543.09 µs, 551.73 µs) | 33.41 ns/op | 0.25x | `bloom [fp=0.010950, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/bloom%20%5Bfp%3D0.010950%2C%20fn%3D0%5D/report/index.html) |
| 7 | bloomfilter | 549.65 µs (546.39 µs, 552.91 µs) | 33.55 ns/op | 0.25x | `bloomfilter [fp=0.004730, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/bloomfilter%20%5Bfp%3D0.004730%2C%20fn%3D0%5D/report/index.html) |
| 8 | ethbloom | 4.32 ms (4.31 ms, 4.33 ms) | 263.69 ns/op | 0.03x | `ethbloom [fp=0.997300, fn=0]` | [plot](target/criterion/contains_absent_scale-4096/ethbloom%20%5Bfp%3D0.997300%2C%20fn%3D0%5D/report/index.html) |

### Contains mixed workload

Lower is better. Build is normalized per inserted item; contains workloads are normalized per query over 16384 operations.

| Rank | Library | Mean (95% CI) | Normalized cost | Vs broomfilter | Precision tag | Plot |
| ---: | --- | --- | ---: | ---: | --- | --- |
| 1 | xorf | 27.66 µs (27.60 µs, 27.73 µs) | 1.69 ns/op | 4.45x | `xorf [fp=0.003880, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/xorf%20%5Bfp%3D0.003880%2C%20fn%3D0%5D/report/index.html) |
| 2 | **broomfilter** | 123.22 µs (123.14 µs, 123.30 µs) | 7.52 ns/op | 1.00x | `broomfilter [fp=0.000470, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/broomfilter%20%5Bfp%3D0.000470%2C%20fn%3D0%5D/report/index.html) |
| 3 | fastbloom | 290.01 µs (288.28 µs, 291.73 µs) | 17.70 ns/op | 0.42x | `fastbloom [fp=0.004840, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/fastbloom%20%5Bfp%3D0.004840%2C%20fn%3D0%5D/report/index.html) |
| 4 | cuckoofilter | 478.46 µs (465.52 µs, 491.40 µs) | 29.20 ns/op | 0.26x | `cuckoofilter [fp=0.015480, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/cuckoofilter%20%5Bfp%3D0.015480%2C%20fn%3D0%5D/report/index.html) |
| 5 | bloom | 490.82 µs (490.21 µs, 491.43 µs) | 29.96 ns/op | 0.25x | `bloom [fp=0.010950, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/bloom%20%5Bfp%3D0.010950%2C%20fn%3D0%5D/report/index.html) |
| 6 | bloomfilter | 567.99 µs (566.93 µs, 569.05 µs) | 34.67 ns/op | 0.22x | `bloomfilter [fp=0.004730, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/bloomfilter%20%5Bfp%3D0.004730%2C%20fn%3D0%5D/report/index.html) |
| 7 | qfilter | 602.81 µs (601.09 µs, 604.52 µs) | 36.79 ns/op | 0.20x | `qfilter [fp=0.001860, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/qfilter%20%5Bfp%3D0.001860%2C%20fn%3D0%5D/report/index.html) |
| 8 | ethbloom | 4.32 ms (4.28 ms, 4.36 ms) | 263.74 ns/op | 0.03x | `ethbloom [fp=0.997300, fn=0]` | [plot](target/criterion/contains_mixed_scale-4096/ethbloom%20%5Bfp%3D0.997300%2C%20fn%3D0%5D/report/index.html) |

