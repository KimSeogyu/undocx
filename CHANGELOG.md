# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- CI: auto-tag workflow that triggers on version changes in Cargo.toml

### Changed
- Updated demo screenshots in README for better clarity and consistent sizing

## [0.4.0] - 2026-03-23

### Added
- Expanded test coverage across conversion scenarios
- Hierarchical `AGENTS.md` documentation across the codebase

### Changed
- Package renamed from `dm2xcod` to `undocx`
- Significant conversion quality improvements for complex documents
- README revamped with updated demo screenshots and rendered Markdown previews
- Locked release permissions to owner approvals only
- Release artifact builds now run only on version tags

### Fixed
- Security, bug, and performance improvements identified by static analysis

## [0.3.14] - 2026-02-24

### Changed
- Refreshed README with updated content
- Ignored `.omx` workspace artifacts in version control

## [0.3.13] - 2026-02-24

### Fixed
- Handle additional body, run, and table docx content types that were previously ignored

## [0.3.12] - 2026-02-15

### Changed
- Hardened quality gates and release operations
- Added latest performance benchmark record to documentation

## [0.3.10] - 2026-02-14

### Added
- Python 3.13 wheel support
- ABI3 universal wheel support for Python 3.12+ (single wheel covers all minor versions)

### Fixed
- Bold and italic Markdown conversion now uses HTML `<strong>`/`<em>` tags for broader compatibility
- Allow dirty working directory during `cargo publish`

### Changed
- Simplified localization module internals

## [0.3.4] - 2026-01-28

### Fixed
- Updated SDT bookmark test to expect `id` attribute correctly
- Changed `rs-docx` dependency from git path to crates.io registry

## [0.3.2] - 2026-01-28

### Added
- Anchor/bookmark improvements for internal document links

## [0.3.1] - 2026-01-28

### Changed
- Updated `rs-docx` dependency to v0.1.9

## [0.3.0] - 2026-01-26

### Changed
- Fresh architecture start; stabilized core conversion pipeline

## [0.2.5] - 2026-01-26

### Changed
- Hardened Python package inclusion to ensure all required files are bundled

## [0.2.4] - 2026-01-26

### Changed
- Hardened package inclusion configuration

## [0.2.2] - 2026-01-23

### Fixed
- Use HTML `<strong>` tags for bold text and handle page breaks in formatted segments correctly

## [0.2.1] - 2026-01-23

### Added
- Support for reading `.docx` files from raw bytes in Python bindings

## [0.2.0] - 2026-01-23

### Added
- Python bindings via maturin/PyO3 (initial release of `undocx` on PyPI)

## [0.1.9] - 2026-01-26

### Added
- Support for reading `.docx` content from bytes in Python bindings

## [0.1.7] - 2026-01-23

### Added
- Support for nested tables

### Changed
- Updated `rs-docx` dependency to v0.1.7

## [0.1.6] - 2026-01-23

### Fixed
- Include test data files in git repository so CI tests pass reliably

## [0.1.5] - 2026-01-23

### Changed
- Updated `rs-docx` to v0.1.4

## [0.1.3] - 2026-01-23

### Added
- Support for track changes, comments, and footnotes

## [0.1.2] - 2026-01-23

### Fixed
- Windows CI shell configuration

## [0.1.1] - 2026-01-23

### Added
- crates.io publishing enabled in CI

## [0.1.0] - 2026-01-23

### Added
- Initial release as `dm2xcod` (later renamed to `undocx`)
- Core `.docx` to Markdown conversion engine built on `rs-docx`
- CLI interface with localization support
- CI/CD pipeline for crates.io and PyPI auto-deployment
- Image extraction support
- Parser tests

[Unreleased]: https://github.com/KimSeogyu/undocx/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/KimSeogyu/undocx/compare/v0.3.14...v0.4.0
[0.3.14]: https://github.com/KimSeogyu/undocx/compare/v0.3.13...v0.3.14
[0.3.13]: https://github.com/KimSeogyu/undocx/compare/v0.3.12...v0.3.13
[0.3.12]: https://github.com/KimSeogyu/undocx/compare/v0.3.10...v0.3.12
[0.3.10]: https://github.com/KimSeogyu/undocx/compare/v0.3.4...v0.3.10
[0.3.4]: https://github.com/KimSeogyu/undocx/compare/v0.3.2...v0.3.4
[0.3.2]: https://github.com/KimSeogyu/undocx/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/KimSeogyu/undocx/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/KimSeogyu/undocx/compare/v0.2.5...v0.3.0
[0.2.5]: https://github.com/KimSeogyu/undocx/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/KimSeogyu/undocx/compare/v0.2.2...v0.2.4
[0.2.2]: https://github.com/KimSeogyu/undocx/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/KimSeogyu/undocx/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/KimSeogyu/undocx/compare/v0.1.9...v0.2.0
[0.1.9]: https://github.com/KimSeogyu/undocx/compare/v0.1.7...v0.1.9
[0.1.7]: https://github.com/KimSeogyu/undocx/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/KimSeogyu/undocx/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/KimSeogyu/undocx/compare/v0.1.3...v0.1.5
[0.1.3]: https://github.com/KimSeogyu/undocx/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/KimSeogyu/undocx/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/KimSeogyu/undocx/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/KimSeogyu/undocx/releases/tag/v0.1.0
