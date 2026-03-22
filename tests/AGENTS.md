<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-22 | Updated: 2026-03-22 -->

# tests

## Purpose
Integration and end-to-end test suites for the undocx converter. Includes golden snapshot comparison, regression tests, property-based testing, and parser feature validation.

## Key Files

| File | Description |
|------|-------------|
| `integration_test.rs` | End-to-end conversion tests — converts DOCX files and validates Markdown output |
| `golden_snapshot_test.rs` | Golden file tests — compares converter output against expected `.md` files in `golden/` |
| `generated_regression_test.rs` | Regression tests using generated DOCX documents |
| `invariant_randomized_test.rs` | Property-based randomized tests — verifies invariants hold across random inputs |
| `parser_feature_test.rs` | Tests for specific DOCX parsing features |

## Subdirectories

| Directory | Purpose |
|-----------|---------|
| `pandoc/` | Pandoc reference DOCX files (~36 files) — comprehensive feature coverage |
| `golden/` | Expected Markdown output files for golden snapshot comparison |
| `output/` | Generated test output files (~35 `.md` files) — gitignored or regenerated |
| `aaa/` | Simple test DOCX documents for basic validation |

## For AI Agents

### Working In This Directory
- Golden tests are the primary correctness gate — update expected files in `golden/` when output intentionally changes
- Test DOCX files in `pandoc/` cover: block quotes, tables, lists, comments, track changes, footnotes, images, and more
- Output files in `output/` are generated during test runs — do not manually edit
- Use `pretty_assertions` crate for readable diff output in test failures

### Testing Requirements
- Run `cargo test` to execute all test suites
- When changing converter behavior, check if golden snapshots need updating
- New DOCX features should get both a test document and a golden expected file

### Common Patterns
- Tests follow the pattern: load DOCX → convert → compare output
- `pretty_assertions::assert_eq!` for readable diffs
- Regression tests prevent previously fixed bugs from recurring

## Dependencies

### Internal
- `undocx` — the library under test

### External
- `pretty_assertions` — enhanced assertion diffs
- `hard-xml` — XML parsing for test setup

<!-- MANUAL: -->
