# Changelog

## [Version 2.1.0] - 13/6/2026

### Changed
- Changed the public functions to allow usage with constants

### Added
- `#![no_std]` support for flexibility to ensure compatibility in low-level/embedded systems

## [Version 2.0.0] - 24/3/2026
### Changed
- **Breaking Change**: `to_celsius`, `to_fahrenheit`, and `to_kelvin` now return `Option<Self>` instead of `f64`.
- `Temperature::new` now returns `Option<Self>` to ensure temperatures below absolute zero aren't possible.
### Added
- `get_value` method to the `Temperature` struct for easy access to the result value, without implementing 'Debug'
- Absolute zero validation for all temperature units.
