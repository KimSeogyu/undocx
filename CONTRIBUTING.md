# Contributing to undocx

Thank you for your interest in contributing. This guide covers everything you need to get started.

## 1. Getting Started

### Prerequisites

- Rust 1.75 or later (MSRV)
- `cargo` (comes with Rust via [rustup](https://rustup.rs))
- For Python bindings: Python 3.12+, [maturin](https://github.com/PyO3/maturin) (`pip install maturin`)

### Clone and Build

```sh
git clone https://github.com/KimSeogyu/undocx.git
cd undocx

# Core library and CLI
cargo build

# With Python bindings (PyO3/maturin)
cargo build --features python
```

### Run Tests

```sh
cargo test --all-features
```

---

## 2. Development Workflow

1. Fork the repository and create a feature branch off `main`:
   ```sh
   git checkout -b feat/your-feature
   ```
2. Make your changes (see [Code Style](#3-code-style) below).
3. Add or update tests as appropriate.
4. Run the full test and lint suite locally before pushing.
5. Open a pull request against `main`.

---

## 3. Code Style

- **Formatting**: run `cargo fmt` before committing. CI enforces this.
- **Linting**: the project uses clippy with zero-warning policy:
  ```sh
  cargo clippy --all-features --tests -- -D warnings
  ```
- Do not suppress warnings with `#[allow(...)]` unless there is a strong justification and a comment explaining it.
- Follow existing naming conventions and module structure.

---

## 4. Testing

### Unit Tests

Inline unit tests live alongside the code they test. Run all tests with:

```sh
cargo test --all-features
```

### Golden Snapshot Tests

Golden snapshot tests are in `tests/golden_snapshot_test.rs`. They compare converter output against checked-in `.md` reference files. When your change intentionally alters output:

1. Delete or update the relevant golden file in `tests/`.
2. Run `cargo test --all-features` to regenerate it.
3. Review the diff and commit the updated snapshot.

Do not update golden files to paper over regressions — only update them when the new output is demonstrably better.

### Integration Tests

End-to-end conversion tests live under `tests/` and use sample `.docx` files from `samples/`. Add a sample file and a corresponding expected output file when introducing support for a new DOCX feature.

### Benchmarks

Performance benchmarks are in `benches/`. To run the full benchmark suite:

```sh
./scripts/run_perf_benchmark.sh
```

Include benchmark results in your PR description when a change is likely to affect performance.

---

## 5. Architecture Overview

The conversion pipeline is composed of two pluggable traits:

```
DOCX file
   │
   ▼
AstExtractor   (src/extractor/)
   │  Parses DOCX XML into an intermediate AST
   ▼
AST (document model)
   │
   ▼
Renderer       (src/renderer/)
   │  Walks the AST and emits Markdown
   ▼
Markdown output
```

- **`AstExtractor` trait** — converts a raw DOCX into the internal document AST. Implement this trait to support new input formats.
- **AST types** — defined in `src/ast/`. Represent paragraphs, runs, tables, images, and other document elements in a format-neutral way.
- **`Renderer` trait** — consumes the AST and produces a string output. Implement this trait to support new output formats.
- **Python bindings** — thin PyO3 wrapper over the core library, enabled with `--features python` and built as a wheel via `maturin`.

When adding support for a new DOCX construct, add it to the AST first, then update the extractor to populate it and the renderer to emit it.

---

## 6. Submitting a PR

A good pull request includes:

- **A clear description** of what the change does and why.
- **Tests** — new behavior covered by unit or integration tests; golden snapshots updated if output changes.
- **No new clippy warnings** — `cargo clippy --all-features --tests -- -D warnings` must pass.
- **Formatted code** — `cargo fmt` applied.
- **Benchmark data** (if performance-sensitive).

### Review Process

1. CI runs automatically: tests, clippy, and cross-platform builds (Linux, macOS, Windows).
2. A maintainer will review and may request changes.
3. Once approved and CI passes, the PR will be merged into `main`.

### Release Process

Releases are fully automated. Pushing a version tag (e.g. `v0.5.0`) triggers the CI pipeline which builds wheels for all supported platforms and publishes to both crates.io and PyPI. Do not publish manually.

---

## Questions?

Open a [GitHub Issue](https://github.com/KimSeogyu/undocx/issues) or start a discussion. We're happy to help.
