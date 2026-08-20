# Changelog

All notable changes to MetaLens are documented here. The project follows
[Semantic Versioning](https://semver.org/).

## [1.0.6] - 2026-08-20

### Changed

- Separate pull-request CI from stable, tag-only release publishing.
- Verify release tags against crate version metadata before publishing.
- Produce SHA-256 checksums with every GitHub release.
- Build and test with the committed Cargo lockfile on all supported platforms.
- Correct repository and release links in the installation documentation.

### Fixed

- Align crate version metadata with the next release after `v1.0.5`.
