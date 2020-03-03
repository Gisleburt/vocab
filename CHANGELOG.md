# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2020-03-03
### Added
- Import / Export

### Fix
- Error message when vocab is not initialised gave incorrect advice
- Remove build dependencies that the build actually didn't depend on (instead use cargo script for doc building)

## [0.1.1] - 2020-02-25
### Added
- Installation instructions
- Makefile for clarity on running tests

### Fix
- Endless mode now actually endless
- `Your guess: ` now on same line as guess

## [0.1.0] - 2020-02-25
### Added
- Initialise with `vocab init`
- Add vocabulary with `vocab add <local> <foreign>`
- Single Mode `vocab single`
- Endless Mode `vocab`
