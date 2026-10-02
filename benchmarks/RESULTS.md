# Benchmark results: digits 1.1.1 vs this branch

Machine: Intel(R) Xeon(R) Processor @ 2.10GHz, Linux x86_64, rustc 1.97.0 (2d8144b78 2026-07-07).  
Criterion medians with 95% confidence intervals, release builds, the same inputs for both versions.

### construct

`Digits::new` from an n-digit string

| n | 1.1.1 | new | speedup |
|---:|---:|---:|---:|
| 100 | 31.1 µs (31 µs – 31.2 µs) | 368 ns (362 ns – 372 ns) | 84.6× |
| 1000 | 952 µs (938 µs – 961 µs) | 2.47 µs (2.46 µs – 2.49 µs) | 386× |
| 4000 | 11 ms (11 ms – 11.1 ms) | 9.05 µs (9 µs – 9.17 µs) | 1,218× |

### to_s

`to_s` of an n-digit number

| n | 1.1.1 | new | speedup |
|---:|---:|---:|---:|
| 100 | 18.4 µs (18.4 µs – 18.4 µs) | 127 ns (125 ns – 129 ns) | 145× |
| 1000 | 261 µs (259 µs – 264 µs) | 949 ns (944 ns – 962 ns) | 275× |
| 4000 | 1.39 ms (1.38 ms – 1.4 ms) | 3.73 µs (3.71 µs – 3.76 µs) | 372× |

### count_1000

1,000 consecutive `succ` calls on a zero counter n digits wide

| n | 1.1.1 | new | speedup |
|---:|---:|---:|---:|
| 8 | 4.17 ms (4.13 ms – 4.18 ms) | 35.2 µs (34.8 µs – 36.5 µs) | 118× |
| 64 | 51.8 ms (51.7 ms – 52.1 ms) | 35.8 µs (35.7 µs – 36.5 µs) | 1,446× |
| 1000 | 1.18 s (1.17 s – 1.19 s) | 45.7 µs (44.7 µs – 45.9 µs) | 25,774× |

### succ_full_carry

one `succ` on n nines (carry through every digit)

| n | 1.1.1 | new | speedup |
|---:|---:|---:|---:|
| 10 | 28.6 µs (28.4 µs – 29.4 µs) | 46.5 ns (45.2 ns – 46.9 ns) | 615× |
| 100 | 3.44 ms (3.4 ms – 3.5 ms) | 149 ns (141 ns – 156 ns) | 23,083× |
| 1000 | 1.12 s (1.02 s – 1.14 s) | 743 ns (664 ns – 854 ns) | 1,503,810× |

### add

`add` of two n-digit numbers

| n | 1.1.1 | new | speedup |
|---:|---:|---:|---:|
| 10 | 16.5 µs (16.4 µs – 16.8 µs) | 53.7 ns (53.5 ns – 53.9 ns) | 307× |
| 100 | 1.59 ms (1.59 ms – 1.6 ms) | 248 ns (246 ns – 250 ns) | 6,442× |
| 1000 | 175 ms (174 ms – 176 ms) | 1.78 µs (1.77 µs – 1.79 µs) | 98,318× |

### mul

`mul` of two n-digit numbers

| n | 1.1.1 | new | speedup |
|---:|---:|---:|---:|
| 5 | 705 µs (698 µs – 721 µs) | 147 ns (145 ns – 149 ns) | 4,800× |
| 10 | 14.8 ms (14.7 ms – 15.2 ms) | 245 ns (239 ns – 254 ns) | 60,530× |
| 20 | 379 ms (371 ms – 388 ms) | 503 ns (488 ns – 537 ns) | 753,462× |

### pow_2_to_the

`pow`: 2 to the power n

| n | 1.1.1 | new | speedup |
|---:|---:|---:|---:|
| 25 | 1.12 ms (1.1 ms – 1.31 ms) | 700 ns (687 ns – 723 ns) | 1,602× |
| 50 | 18.2 ms (17.9 ms – 18.7 ms) | 920 ns (905 ns – 1.04 µs) | 19,772× |
| 100 | 369 ms (364 ms – 385 ms) | 1.4 µs (1.37 µs – 1.48 µs) | 263,213× |

### partial_cmp

`partial_cmp` of two n-digit numbers

| n | 1.1.1 | new | speedup |
|---:|---:|---:|---:|
| 10 | 19.5 µs (19.5 µs – 19.6 µs) | 13.4 ns (13.3 ns – 13.5 ns) | 1,457× |
| 100 | 2.97 ms (2.97 ms – 2.98 ms) | 44.7 ns (44.6 ns – 44.9 ns) | 66,434× |
| 1000 | 389 ms (384 ms – 394 ms) | 382 ns (381 ns – 383 ns) | 1,018,013× |

### decimal_to_hex

`hex()` of an n-digit decimal number (1.1.1 overflows above 19 digits)

| n | 1.1.1 | new | speedup |
|---:|---:|---:|---:|
| 6 | 31.9 µs (31.8 µs – 32 µs) | 512 ns (509 ns – 516 ns) | 62.3× |
| 12 | 259 µs (258 µs – 260 µs) | 540 ns (538 ns – 543 ns) | 480× |
| 18 | 1.05 ms (1.04 ms – 1.06 ms) | 571 ns (570 ns – 573 ns) | 1,833× |

### hex_to_decimal

`decimal()` of an n-digit hex number

| n | 1.1.1 | new | speedup |
|---:|---:|---:|---:|
| 4 | 99.3 µs (97.5 µs – 103 µs) | 436 ns (434 ns – 454 ns) | 228× |
| 8 | 2.34 ms (2.33 ms – 2.38 ms) | 453 ns (445 ns – 480 ns) | 5,163× |
| 16 | 82.9 ms (81.5 ms – 87.8 ms) | 597 ns (593 ns – 599 ns) | 138,890× |

### next_non_adjacent_1000

1,000 consecutive `next_non_adjacent(n)` steps from `00000000`

| n | 1.1.1 | new | speedup |
|---:|---:|---:|---:|
| 0 | 8.47 ms (8.38 ms – 8.63 ms) | 43.4 µs (42.8 µs – 45.9 µs) | 195× |
| 1 | 7.45 ms (7.4 ms – 7.57 ms) | 42 µs (41.5 µs – 42.9 µs) | 178× |

### large (new only)

Sizes 1.1.1 cannot reach in reasonable time or overflows on.

| workload | new |
|---|---:|
| construct/1000000 | 2.11 ms (2.09 ms – 2.13 ms) |
| decimal_to_hex/10000 | 31 ms (30.6 ms – 31.5 ms) |
| mul/2000 | 1.67 ms (1.67 ms – 1.68 ms) |
| pow_2_to_the/100000 | 126 ms (126 ms – 126 ms) |
| succ_full_carry/100000 | 48.3 µs (48.1 µs – 51.2 µs) |

