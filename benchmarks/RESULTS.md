# Benchmark results: digits 1.1.1 vs 2.0.0

Machine: Intel(R) Xeon(R) Processor @ 2.10GHz, Linux x86_64, rustc 1.97.0 (2d8144b78 2026-07-07).  
Criterion medians with 95% confidence intervals, release builds, the same inputs for both versions.

### construct

`Digits::new` from an n-digit string

| n | 1.1.1 | 2.0.0 | speedup |
|---:|---:|---:|---:|
| 100 | 26.3 µs (23.3 µs – 27.3 µs) | 1.12 µs (1.09 µs – 1.14 µs) | 23.5× |
| 1000 | 838 µs (810 µs – 860 µs) | 7.13 µs (6.99 µs – 7.27 µs) | 118× |
| 4000 | 7.05 ms (6.82 ms – 7.15 ms) | 33.5 µs (33.3 µs – 33.7 µs) | 210× |

### to_s

`to_s` of an n-digit number

| n | 1.1.1 | 2.0.0 | speedup |
|---:|---:|---:|---:|
| 100 | 11.5 µs (11.3 µs – 12.2 µs) | 116 ns (115 ns – 121 ns) | 98.7× |
| 1000 | 193 µs (192 µs – 194 µs) | 744 ns (735 ns – 787 ns) | 259× |
| 4000 | 1.25 ms (1.23 ms – 1.26 ms) | 3.54 µs (3.51 µs – 3.59 µs) | 352× |

### count_1000

1,000 consecutive `succ` calls on a zero counter n digits wide

| n | 1.1.1 | 2.0.0 | speedup |
|---:|---:|---:|---:|
| 8 | 2.73 ms (2.72 ms – 2.77 ms) | 2.12 µs (2.09 µs – 2.14 µs) | 1,287× |
| 64 | 36.5 ms (34.8 ms – 38.2 ms) | 2.13 µs (2.12 µs – 2.14 µs) | 17,151× |
| 1000 | 1.18 s (1.16 s – 1.22 s) | 2.14 µs (2.13 µs – 2.16 µs) | 554,442× |

### succ_full_carry

one `succ` on n nines (carry through every digit)

| n | 1.1.1 | 2.0.0 | speedup |
|---:|---:|---:|---:|
| 10 | 23.9 µs (23 µs – 24.9 µs) | 19.1 ns (18 ns – 21.9 ns) | 1,250× |
| 100 | 2.66 ms (2.41 ms – 3.45 ms) | 104 ns (102 ns – 105 ns) | 25,533× |
| 1000 | 1.14 s (1.02 s – 1.22 s) | 700 ns (607 ns – 756 ns) | 1,624,319× |

### add

`add` of two n-digit numbers

| n | 1.1.1 | 2.0.0 | speedup |
|---:|---:|---:|---:|
| 10 | 13.6 µs (13.5 µs – 13.8 µs) | 41.6 ns (41.3 ns – 42.6 ns) | 328× |
| 100 | 1.05 ms (925 µs – 1.1 ms) | 197 ns (197 ns – 198 ns) | 5,323× |
| 1000 | 134 ms (126 ms – 141 ms) | 1.51 µs (1.47 µs – 1.53 µs) | 88,374× |

### mul

`mul` of two n-digit numbers

| n | 1.1.1 | 2.0.0 | speedup |
|---:|---:|---:|---:|
| 5 | 601 µs (599 µs – 610 µs) | 149 ns (141 ns – 151 ns) | 4,039× |
| 10 | 12.3 ms (11.1 ms – 12.7 ms) | 182 ns (177 ns – 197 ns) | 67,767× |
| 20 | 298 ms (269 ms – 320 ms) | 417 ns (415 ns – 433 ns) | 715,014× |

### pow_2_to_the

`pow`: 2 to the power n

| n | 1.1.1 | 2.0.0 | speedup |
|---:|---:|---:|---:|
| 25 | 818 µs (715 µs – 896 µs) | 757 ns (704 ns – 837 ns) | 1,080× |
| 50 | 15.5 ms (13.8 ms – 16.8 ms) | 907 ns (880 ns – 940 ns) | 17,050× |
| 100 | 300 ms (257 ms – 364 ms) | 1.23 µs (1.12 µs – 1.34 µs) | 243,183× |

### partial_cmp

`partial_cmp` of two n-digit numbers

| n | 1.1.1 | 2.0.0 | speedup |
|---:|---:|---:|---:|
| 10 | 10.9 µs (10.7 µs – 11.3 µs) | 9.6 ns (8.88 ns – 9.69 ns) | 1,135× |
| 100 | 2.2 ms (2.12 ms – 2.22 ms) | 46 ns (45.3 ns – 46.9 ns) | 47,931× |
| 1000 | 321 ms (307 ms – 333 ms) | 295 ns (293 ns – 298 ns) | 1,086,989× |

### decimal_to_hex

`hex()` of an n-digit decimal number (1.1.1 overflows above 19 digits)

| n | 1.1.1 | 2.0.0 | speedup |
|---:|---:|---:|---:|
| 6 | 27.1 µs (26.9 µs – 27.6 µs) | 748 ns (743 ns – 757 ns) | 36.2× |
| 12 | 221 µs (220 µs – 222 µs) | 832 ns (811 ns – 884 ns) | 265× |
| 18 | 877 µs (873 µs – 879 µs) | 820 ns (797 ns – 833 ns) | 1,069× |

### hex_to_decimal

`decimal()` of an n-digit hex number

| n | 1.1.1 | 2.0.0 | speedup |
|---:|---:|---:|---:|
| 4 | 83.6 µs (78.4 µs – 87.6 µs) | 491 ns (468 ns – 543 ns) | 170× |
| 8 | 2 ms (1.82 ms – 2.07 ms) | 578 ns (559 ns – 597 ns) | 3,458× |
| 16 | 68.6 ms (64.4 ms – 73.8 ms) | 585 ns (513 ns – 713 ns) | 117,331× |

### next_non_adjacent_1000

1,000 consecutive `next_non_adjacent(n)` steps from `00000000`

| n | 1.1.1 | 2.0.0 | speedup |
|---:|---:|---:|---:|
| 0 | 7 ms (6.72 ms – 7.25 ms) | 12.7 µs (11.9 µs – 15.1 µs) | 553× |
| 1 | 6.83 ms (6.23 ms – 7.26 ms) | 11.4 µs (10.3 µs – 11.8 µs) | 596× |

### large (2.0.0 only)

Sizes 1.1.1 cannot reach in reasonable time or overflows on.

| workload | 2.0.0 |
|---|---:|
| construct/1000000 | 8.02 ms (6.69 ms – 8.29 ms) |
| decimal_to_hex/10000 | 25.9 ms (24.3 ms – 26.8 ms) |
| mul/2000 | 1.5 ms (1.42 ms – 1.58 ms) |
| pow_2_to_the/100000 | 112 ms (111 ms – 119 ms) |
| succ_full_carry/100000 | 42.9 µs (37.4 µs – 48.2 µs) |

