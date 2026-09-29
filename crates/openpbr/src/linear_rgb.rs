// Copyright 2026 the openpbr crate Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use color::{Aces2065_1, AcesCg, ColorSpace, LinearSrgb};

/// A linear-light RGB space suitable for OpenPBR parameter values.
///
/// Implemented for [`AcesCg`], [`Aces2065_1`], and [`LinearSrgb`]. Encoded RGB
/// spaces and XYZ are excluded, even though XYZ is linear. This bound applies
/// to the types themselves, so unsupported spaces fail during `cargo check`,
/// including when constructing values directly or converting between spaces.
///
/// # Implementing a custom space
///
/// Implementors must use linear-light RGB coordinates with
/// [`ColorSpace::IS_LINEAR`] set to true, white at `[1, 1, 1]`, and neutral
/// grays at `[v, v, v]`. Conversions must preserve extended coordinates
/// (including negative components and components above one) without clipping
/// or gamut mapping. These semantic requirements are the implementor's
/// responsibility; the marker does not verify a custom conversion's math.
///
/// ```compile_fail
/// fn encoded(_: openpbr::Parameters<openpbr::color::Srgb>) {}
/// ```
///
/// ```compile_fail
/// fn tristimulus(_: openpbr::Parameters<openpbr::color::XyzD65>) {}
/// ```
///
/// ```compile_fail
/// let p = openpbr::Parameters::<openpbr::color::AcesCg>::DEFAULT;
/// let encoded = p.convert::<openpbr::color::Srgb>();
/// ```
///
/// ```compile_fail
/// let p = openpbr::Parameters::<openpbr::color::AcesCg>::DEFAULT;
/// let tristimulus = p.convert::<openpbr::color::XyzD65>();
/// ```
pub trait LinearRgb: ColorSpace {}

impl LinearRgb for AcesCg {}
impl LinearRgb for Aces2065_1 {}
impl LinearRgb for LinearSrgb {}
