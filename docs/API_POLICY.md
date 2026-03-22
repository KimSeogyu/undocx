# API Stability Policy

## Scope

This policy applies to the public Rust API exposed from `undocx`:

- `DocxToMarkdown`
- `ConvertOptions`
- `ImageHandling`
- public modules under `adapters`, `core`, and `render`

## SemVer Rules

- Patch (`x.y.Z`):
  - bug fixes
  - performance improvements
  - additive tests/docs
  - no breaking changes to existing public signatures
- Minor (`x.Y.z`):
  - additive public API
  - new optional behavior behind defaults
- Major (`X.y.z`):
  - breaking changes
  - behavior removals or semantic flips

## Dependency Injection Contracts

`DocxToMarkdown::with_components` accepts custom:

- `adapters::docx::AstExtractor`
- `render::Renderer`

These trait contracts are treated as public integration points. Any incompatible change
to these trait method signatures requires a major version bump.

## Deprecation Process

For public API replacements:

1. Mark old API with `#[deprecated]` and migration note.
2. Keep old API for at least one minor release.
3. Remove only in next major release.

## Quality Gates

All release candidates should satisfy:

- `cargo clippy --all-features --tests -- -D warnings`
- `cargo test --all-features`
- golden snapshot tests in `tests/golden_snapshot_test.rs`
- benchmark script `scripts/run_perf_benchmark.sh`
