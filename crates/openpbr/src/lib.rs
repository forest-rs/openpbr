// Copyright 2026 the openpbr crate Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The [OpenPBR Surface] parameter set as plain Rust data.
//!
//! OpenPBR is an open, physically based über-shader specification from the
//! Academy Software Foundation. This crate implements its
//! [parameter reference] for version [`SPEC_VERSION`]:
//!
//! - [`Parameters`]: one constant value for every non-geometric parameter,
//!   with the specification's defaults ([`Parameters::DEFAULT`]), range
//!   validation, dynamic access by [`Param`], and color-space conversion.
//! - [`Param`] and [`ParamInfo`]: every parameter, including the `vector3`
//!   geometry inputs, with its stable identifier, label, group, type, allowed
//!   and typical ranges, default, unit, and whether it is a color. Tools that
//!   pack parameters into textures or build editors iterate [`Param::ALL`].
//!
//! It evaluates no BSDF and knows no file format: renderers, texture
//! generators and exporters share the vocabulary and bring their own
//! evaluation. Mapping to glTF PBR is lossy and belongs in an adapter crate.
//!
//! # Color spaces
//!
//! Colors are [`color::OpaqueColor`] values tagged with their color space at
//! the type level. [`Parameters`] defaults to [`color::AcesCg`], the space
//! OpenPBR assumes when a material names none; a renderer working in linear
//! Rec. 709 uses `Parameters<LinearSrgb>`, and [`Parameters::convert`] moves
//! between them. [`LinearRgb`] restricts parameters to linear RGB spaces;
//! encoded RGB and XYZ are excluded. The [`color`] crate is re-exported so
//! the versions match.
//!
//! ```
//! use openpbr::color::{AcesCg, LinearSrgb, OpaqueColor};
//! use openpbr::{Param, Parameters, Value};
//!
//! // A lacquered red in the specification's default space.
//! let lacquer = Parameters::<AcesCg> {
//!     base_color: OpaqueColor::new([0.4, 0.02, 0.01]),
//!     coat_weight: 1.0,
//!     coat_roughness: 0.1,
//!     ..Parameters::DEFAULT
//! };
//! assert_eq!(lacquer.validate(), Ok(()));
//! assert_eq!(lacquer.get(Param::CoatWeight), Some(Value::Float(1.0)));
//!
//! // The same material for a renderer that shades in linear Rec. 709.
//! let rec709: Parameters<LinearSrgb> = lacquer.convert();
//! assert_eq!(rec709.coat_weight, 1.0);
//!
//! // Metadata drives generic tools.
//! let info = Param::SpecularIor.info();
//! assert_eq!(info.identifier, "specular_ior");
//! assert!(!info.range.unwrap().contains(0.0));
//! ```
//!
//! # Validation and extended RGB
//!
//! [`Parameters::validate`] requires every numeric component to be finite.
//! Scalars and non-color triples also obey their specification ranges.
//! Chromatic colors may have negative components or components above one:
//! these coordinates can result from conversion between RGB gamuts.
//! [`Parameters::validate_spec_ranges`] additionally checks color components
//! against the specification bounds in the current RGB space. Neither check
//! enforces typical ("norm") ranges or guarantees renderer support.
//!
//! ```
//! use openpbr::{Parameters, color::{AcesCg, LinearSrgb, OpaqueColor}};
//!
//! let red = Parameters::<AcesCg> {
//!     base_color: OpaqueColor::new([1.0, 0.0, 0.0]),
//!     ..Parameters::DEFAULT
//! };
//! assert_eq!(red.validate_spec_ranges(), Ok(()));
//! let converted: Parameters<LinearSrgb> = red.convert();
//! assert!(converted.base_color.components[0] > 1.0);
//! assert!(converted.base_color.components[1] < 0.0);
//! assert_eq!(converted.validate(), Ok(()));
//! assert!(converted.validate_spec_ranges().is_err());
//! ```
//!
//! Conversion preserves extended coordinates without clipping or gamut
//! mapping. The caller owns any gamut policy needed before shading. Public
//! fields, [`Parameters::set`], and deserialization are unchecked; validate
//! before use. Serde payloads carry no color-space identifier, specification
//! version, or scene-unit scale; the containing format supplies that context.
//!
//! # Features
//!
//! - `std` (default): enables `color/std`.
//! - `libm`: enables `color/libm` for `no_std` targets. One of `std` or
//!   `libm` is required.
//! - `serde`: serializes [`Parameters`] with the specification's identifiers
//!   as field names and colors as `[r, g, b]` arrays, and [`Param`] as its
//!   identifier. Missing fields take their defaults; unknown fields are an
//!   error.
//!
//! [OpenPBR Surface]: https://academysoftwarefoundation.github.io/OpenPBR/
//! [parameter reference]: https://academysoftwarefoundation.github.io/OpenPBR/#parameterreference

#![no_std]

#[cfg(test)]
extern crate alloc;

#[cfg(not(any(feature = "std", feature = "libm")))]
compile_error!("openpbr requires either the `std` or `libm` feature");

pub use color;

mod linear_rgb;
mod param;
mod parameters;
#[cfg(feature = "serde")]
mod serde_impl;

pub use linear_rgb::LinearRgb;
pub use param::{Bound, Group, Kind, Param, ParamDefault, ParamInfo, Range, Unit, Value};
pub use parameters::{Parameters, SetError, ValidationError};

#[cfg(feature = "serde")]
pub(crate) use serde_impl::color as serde_color;

/// The OpenPBR specification version this crate implements.
pub const SPEC_VERSION: &str = "1.1.1";
