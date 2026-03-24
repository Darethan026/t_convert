# Changelog

## [Version 2.0.0] - 24/3/2026
### Changed
- **Breaking Change**: `to_celsius`, `to_fahrenheit`, and `to_kelvin` now return `Option<Self>` instead of `f64`.
- `Temperature::new` now returns `Option<Self>` to ensure temperatures below absolute zero aren't possible.
### Added
- `get_value` method to the `Temperature` struct for easy access to the result value, without implementing 'Debug'
- Absolute zero validation for all temperature units.