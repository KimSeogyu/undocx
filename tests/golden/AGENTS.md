<!-- Parent: ../AGENTS.md -->
<!-- Generated: 2026-03-23 | Updated: 2026-03-23 -->

# golden

## Purpose
Expected output files for golden snapshot tests. Each `*_expected.md` file is the reference output that `tests/golden_snapshot_test.rs` compares against.

## Key Files

| File | Description |
|------|-------------|
| `deep_list_expected.md` | Expected output for deeply nested list indentation |
| `notes_comments_expected.md` | Expected output for footnotes, endnotes, and comments |

## For AI Agents

### Working In This Directory
- Update these files **only** when intentionally changing converter output
- Run `cargo test golden_snapshot` to verify changes
- Review diffs carefully — every character matters

### Testing Requirements
- `cargo test golden_snapshot` compares converter output against these files
- If a test fails after an intentional change, update the expected file and commit

## Dependencies

### Internal
- Referenced by `tests/golden_snapshot_test.rs`

<!-- MANUAL: -->
