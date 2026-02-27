# Changelog

All notable changes to this project are documented in this file.

## [Unreleased]

## [0.1.7] - 2026-02-27

### Added
- Changelog tracking in `CHANGELOG.md`.
- Managed source markers in generated CMake templates (`# JUMAKE_SOURCES_BEGIN` / `# JUMAKE_SOURCES_END`) to make later source insertion more reliable.

### Changed
- `new` and `add` command enums now use strict clap `ValueEnum` parsing for earlier CLI validation.
- Project creation and git initialization now use fail-fast `Result` propagation instead of continuing after partial failures.
- JUCE checkout now uses a shallow clone (`depth=1`) to speed up project initialization and reduce download size.
- JUCE branch selection now follows `origin/HEAD` (default branch), instead of assuming `master`.

### Fixed
- Removed shell-based executable discovery in `run` and replaced it with Rust-native path scanning for safer execution.
- Improved executable selection logic across platforms and tightened Windows executable checks to `.exe` files.
- Hardened `add_class` source insertion into `src/CMakeLists.txt` with marker-first insertion and robust fallbacks.
- Addressed clippy issues (`&PathBuf` to `&Path`, collapsible branches, useless formatting) and formatting inconsistencies.
