# Changelog

All notable changes to this project are documented in this file.

## [Unreleased]

### Changed
- JUCE checkout now uses a shallow clone (`depth=1`) to speed up project initialization and reduce download size.
- JUCE branch selection now follows `origin/HEAD` (default branch), instead of assuming `master`.
