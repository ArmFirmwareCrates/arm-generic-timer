# Changelog

## Unreleased

### Breaking changes

- System register based timers must now be constructed with their unsafe `new`
  functions, whose safety contracts require exclusive access to the
  corresponding system registers.

### Improvements

- Added `remaining_time` to `Timer<T>`, to enable querying TVAL.
- Added a `Counter` abstraction and an implementation of the
  `embedded-hal-timer` `Timer` trait.

### Other changes

- Updated `arm-sysregs` to 0.4.0 and `embedded-hal-timer` to 0.2.0.
- Fixed `inconsistent_digit_grouping` Clippy warnings.
- Changed module documentation to use inner doc comments.

## 0.2.1

### Improvements

- Implemented the `embedded-hal` `DelayNs` trait for `Timer`.
- Enabled all supported features in docs.rs builds.

## 0.2.0

### Improvements

- Added a common `Timer` abstraction layer.
- Added system register based timers.
- Moved memory mapped timers into a dedicated module.

### Other changes

- Updated to the Rust 2024 edition. This increases the MSRV to 1.85.
- Updated dependencies.
- Configured Dependabot.

## 0.1.1

- Moved the repository under arm-firmware-crates.
- Updated dependencies.

## 0.1.0

Initial release.
