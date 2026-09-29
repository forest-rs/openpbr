<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

# Changelog

## [Unreleased]

This release has an [MSRV][] of 1.88.

### Added

- The OpenPBR 1.1.1 parameter set: `Parameters<CS>` with the specification's
  defaults, dynamic access, and conversion between linear RGB color spaces
  through the `color` crate.
- Validation preserving finite extended RGB colors, with optional checks
  against the specification's color bounds.
- `Param` and `ParamInfo` for all 41 parameters of the parameter reference.
- `std`, `libm`, and `serde` features.

[MSRV]: crates/openpbr/README.md#minimum-supported-rust-version-msrv
