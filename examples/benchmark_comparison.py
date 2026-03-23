#!/usr/bin/env python3
"""Benchmark: undocx vs pandoc vs markitdown for DOCX → Markdown conversion.

Usage:
    python examples/benchmark_comparison.py [docx_dir] [iterations]

Defaults:
    docx_dir   = ./tests/pandoc
    iterations = 10
"""

import json
import os
import statistics
import subprocess
import sys
import time
from pathlib import Path

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------
DOCX_DIR = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("./tests/pandoc")
ITERATIONS = int(sys.argv[2]) if len(sys.argv) > 2 else 10
DOCX_FILES = sorted(DOCX_DIR.glob("*.docx"))

if not DOCX_FILES:
    print(f"No .docx files found in {DOCX_DIR}")
    sys.exit(1)

print(f"Benchmark: {len(DOCX_FILES)} files × {ITERATIONS} iterations")
print(f"Directory: {DOCX_DIR}\n")


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------
def bench(name: str, run_fn) -> dict:
    """Run a converter function and collect timing stats."""
    times = []
    errors = 0
    for _ in range(ITERATIONS):
        for f in DOCX_FILES:
            try:
                t0 = time.perf_counter()
                run_fn(f)
                elapsed = (time.perf_counter() - t0) * 1000  # ms
                times.append(elapsed)
            except Exception as e:
                errors += 1
                if errors == 1:
                    print(f"  [{name}] error on {f.name}: {e}")

    if not times:
        return {"name": name, "error": "all failed"}

    return {
        "name": name,
        "files": len(DOCX_FILES),
        "iterations": ITERATIONS,
        "samples": len(times),
        "errors": errors,
        "avg_ms": round(statistics.mean(times), 3),
        "median_ms": round(statistics.median(times), 3),
        "min_ms": round(min(times), 3),
        "max_ms": round(max(times), 3),
        "stdev_ms": round(statistics.stdev(times), 3) if len(times) > 1 else 0,
    }


# ---------------------------------------------------------------------------
# 1. undocx (Python binding)
# ---------------------------------------------------------------------------
def run_undocx(path: Path):
    import undocx
    undocx.convert_docx(str(path))


# ---------------------------------------------------------------------------
# 2. pandoc (subprocess)
# ---------------------------------------------------------------------------
def run_pandoc(path: Path):
    subprocess.run(
        ["pandoc", str(path), "-t", "markdown", "-o", "/dev/null"],
        capture_output=True,
        check=True,
    )


# ---------------------------------------------------------------------------
# 3. markitdown (Microsoft)
# ---------------------------------------------------------------------------
def run_markitdown(path: Path):
    from markitdown import MarkItDown
    md = MarkItDown()
    md.convert(str(path))


# ---------------------------------------------------------------------------
# Run benchmarks
# ---------------------------------------------------------------------------
results = []

# undocx
try:
    import undocx  # noqa: F401
    print("Running undocx ...")
    results.append(bench("undocx", run_undocx))
except ImportError:
    print("undocx not installed — building with maturin ...")
    subprocess.run(["maturin", "develop", "--features", "python"], check=True,
                    capture_output=True)
    print("Running undocx ...")
    results.append(bench("undocx", run_undocx))

# pandoc
try:
    subprocess.run(["pandoc", "--version"], capture_output=True, check=True)
    print("Running pandoc ...")
    results.append(bench("pandoc", run_pandoc))
except FileNotFoundError:
    print("pandoc not found — skipping")

# markitdown
try:
    from markitdown import MarkItDown  # noqa: F401
    print("Running markitdown ...")
    results.append(bench("markitdown", run_markitdown))
except ImportError:
    print("markitdown not installed — skipping")

# ---------------------------------------------------------------------------
# Report
# ---------------------------------------------------------------------------
print("\n" + "=" * 72)
print(f"{'Tool':<15} {'Avg (ms)':>10} {'Median':>10} {'Min':>10} {'Max':>10} {'Errors':>8}")
print("-" * 72)
for r in results:
    if "error" in r:
        print(f"{r['name']:<15} {'FAILED':>10}")
    else:
        print(
            f"{r['name']:<15} {r['avg_ms']:>10.2f} {r['median_ms']:>10.2f} "
            f"{r['min_ms']:>10.2f} {r['max_ms']:>10.2f} {r['errors']:>8}"
        )
print("=" * 72)

# Speedup vs slowest
if len([r for r in results if "error" not in r]) > 1:
    valid = [r for r in results if "error" not in r]
    slowest = max(valid, key=lambda r: r["avg_ms"])
    fastest = min(valid, key=lambda r: r["avg_ms"])
    print(f"\n{fastest['name']} is {slowest['avg_ms'] / fastest['avg_ms']:.1f}x faster than {slowest['name']}")

# Save JSON
out_path = Path("output_tests/perf/comparison.json")
out_path.parent.mkdir(parents=True, exist_ok=True)
with open(out_path, "w") as f:
    json.dump(results, f, indent=2)
print(f"\nResults saved to {out_path}")
