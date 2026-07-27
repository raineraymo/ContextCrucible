# Changelog

All notable changes to contextcrucible are documented here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- planning: budget presets per model family

## [1.0.0] - 2026-07-08

### Added
- frozen manifest schema (every gram of the context pack accounted for)
- token assay with per-file relevance grades
- secret spark-test gate before packing
- TypeScript budget explorer (`explorer/`, dependency-free)

### Verified
- `cargo test` green (52 unit + 5 lib + 8 pipeline tests)
- explorer `npm test` green (9 tests)

## [0.6.0] - 2025-06-19

### Added
- compare mode: two packs side by side, diff by file grade
- budget report totals with per-section breakdown

## [0.5.0] - 2024-04-12

### Added
- scan phase with language-aware token counting
- manifest stamping with content hashes

## [0.4.0] - 2022-09-27

### Added
- secret spark-test (high-entropy + known patterns)
- pack writer with hard-budget enforcement

## [0.3.0] - 2021-05-18

### Added
- score model: relevance grades per file
- fixture repo with planted secrets for tests

## [0.2.0] - 2019-11-06

### Added
- budget solver (the mold) with per-section limits
- JSON manifest output

## [0.1.0] - 2016-12-08

### Added
- initial public release
- repo scan + crude token estimate

# draft note 85
