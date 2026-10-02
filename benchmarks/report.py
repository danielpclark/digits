#!/usr/bin/env python3
"""Summarise `cargo bench` results from target/criterion as a markdown report.

Run from the benchmarks directory after `cargo bench`:

    python3 report.py > RESULTS.md
"""
import json
import pathlib
import platform
import subprocess
from collections import defaultdict

ROOT = pathlib.Path(__file__).resolve().parent / "target" / "criterion"
OLD, NEW = "1.1.1", "new"

DESCRIPTIONS = {
    "construct": "`Digits::new` from an n-digit string",
    "to_s": "`to_s` of an n-digit number",
    "count_1000": "1,000 consecutive `succ` calls on a zero counter n digits wide",
    "succ_full_carry": "one `succ` on n nines (carry through every digit)",
    "add": "`add` of two n-digit numbers",
    "mul": "`mul` of two n-digit numbers",
    "pow_2_to_the": "`pow`: 2 to the power n",
    "partial_cmp": "`partial_cmp` of two n-digit numbers",
    "decimal_to_hex": "`hex()` of an n-digit decimal number (1.1.1 overflows above 19 digits)",
    "hex_to_decimal": "`decimal()` of an n-digit hex number",
    "next_non_adjacent_1000": "1,000 consecutive `next_non_adjacent(n)` steps from `00000000`",
}


def fmt(ns):
    for unit, scale in (("s", 1e9), ("ms", 1e6), ("µs", 1e3)):
        if ns >= scale:
            return f"{ns / scale:.3g} {unit}"
    return f"{ns:.3g} ns"


def fmt_ratio(r):
    if r >= 100:
        return f"{r:,.0f}×"
    return f"{r:.3g}×"


def load():
    results = defaultdict(dict)
    for bench in ROOT.glob("**/new/benchmark.json"):
        meta = json.loads(bench.read_text())
        estimates = json.loads((bench.parent / "estimates.json").read_text())
        median = estimates["median"]
        results[meta["group_id"]][(meta["function_id"], meta["value_str"])] = (
            median["point_estimate"],
            median["confidence_interval"]["lower_bound"],
            median["confidence_interval"]["upper_bound"],
        )
    return results


def size_key(size):
    try:
        return (0, int(size))
    except (TypeError, ValueError):
        return (1, str(size))


def machine():
    cpu = "unknown CPU"
    try:
        for line in open("/proc/cpuinfo"):
            if line.startswith("model name"):
                cpu = line.split(":", 1)[1].strip()
                break
    except OSError:
        pass
    rustc = subprocess.run(["rustc", "--version"], capture_output=True, text=True).stdout.strip()
    return f"{cpu}, {platform.system()} {platform.machine()}, {rustc}"


def main():
    results = load()
    print("# Benchmark results: digits 1.1.1 vs this branch\n")
    print(f"Machine: {machine()}.  ")
    print("Criterion medians with 95% confidence intervals, release builds, the same inputs for both versions.\n")
    for group in DESCRIPTIONS:
        rows = results.get(group)
        if not rows:
            continue
        print(f"### {group}\n\n{DESCRIPTIONS[group]}\n")
        print(f"| n | {OLD} | {NEW} | speedup |")
        print("|---:|---:|---:|---:|")
        sizes = sorted({size for (_, size) in rows}, key=size_key)
        for size in sizes:
            old, new = rows.get((OLD, size)), rows.get((NEW, size))
            if not (old and new):
                continue
            print(
                f"| {size} | {fmt(old[0])} ({fmt(old[1])} – {fmt(old[2])}) "
                f"| {fmt(new[0])} ({fmt(new[1])} – {fmt(new[2])}) | {fmt_ratio(old[0] / new[0])} |"
            )
        print()
    large = results.get("large")
    if large:
        print("### large (new only)\n\nSizes 1.1.1 cannot reach in reasonable time or overflows on.\n")
        print(f"| workload | {NEW} |")
        print("|---|---:|")
        for (function, size), est in sorted(large.items(), key=lambda kv: kv[0][1] or kv[0][0]):
            label = size or function
            print(f"| {label} | {fmt(est[0])} ({fmt(est[1])} – {fmt(est[2])}) |")
        print()


if __name__ == "__main__":
    main()
