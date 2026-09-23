// Copyright 2026 the openpbr crate Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The constant-valued parameter set.

use core::fmt;

use color::{AcesCg, ColorSpace, OpaqueColor};

use crate::{Kind, Param, Value};

/// A complete set of constant OpenPBR parameter values, with colors in the
/// linear color space `CS`.
///
/// Field names are the specification's identifiers, so a field documents
/// itself by its [`Param`]. `CS` defaults to [`AcesCg`], the space OpenPBR
/// assumes when a material names none; renderers working in linear Rec. 709
/// use `Parameters<LinearSrgb>` and [`Parameters::convert`] between the two.
/// `CS` must be linear: using [`Parameters::DEFAULT`] or [`Default`] with a
/// non-linear space such as `Srgb` fails to compile. The check is a constant
/// assertion evaluated when the type is instantiated, so `cargo check` does
/// not report it; `cargo build` and `cargo test` do, with the message
/// "OpenPBR color parameters need a linear color space".
///
/// ```compile_fail
/// let encoded = openpbr::Parameters::<openpbr::color::Srgb>::DEFAULT;
/// ```
///
/// `subsurface_radius_scale` is typed `color3` by the specification but holds
/// per-channel factors of a length, not a color, so it is a plain `[f32; 3]`
/// and is not converted. Lengths (`subsurface_radius`, `transmission_depth`)
/// are in world-space units; `thin_film_thickness` is in micrometers; and
/// `emission_luminance` is in nits.
///
/// The four `vector3` geometry parameters (`geometry_normal`,
/// `geometry_tangent`, `geometry_coat_normal`, `geometry_coat_tangent`)
/// default to the surface's unperturbed frame and are normally driven by
/// textures, so they have no constant field here. [`Param`] still describes
/// them.
///
/// [`Parameters::DEFAULT`] holds the specification's defaults: a gray,
/// slightly glossy dielectric. With the `serde` feature, fields serialize
/// under their identifiers and colors as `[r, g, b]` arrays; missing fields
/// deserialize to their defaults, and unknown fields are an error, so a
/// material from a later specification version is not silently reduced.
#[derive(Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default, deny_unknown_fields, bound = ""))]
pub struct Parameters<CS: ColorSpace = AcesCg> {
    /// Base: scales the whole base substrate. Range `[0, 1]`, default 1.
    pub base_weight: f32,
    /// Base: diffuse albedo for dielectrics, normal-incidence reflectivity
    /// for metals. Default (0.8, 0.8, 0.8).
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_color"))]
    pub base_color: OpaqueColor<CS>,
    /// Base: fraction of the surface that is metallic. Range `[0, 1]`,
    /// default 0.
    pub base_metalness: f32,
    /// Base: roughness of the diffuse lobe; 0 is Lambertian. Range `[0, 1]`,
    /// default 0.
    pub base_diffuse_roughness: f32,

    /// Specular: multiplier of the specular reflection. Range `[0, ∞)`, norm
    /// `[0, 1]`, default 1.
    pub specular_weight: f32,
    /// Specular: tint of dielectric reflection; edge tint of metals. Default
    /// (1, 1, 1).
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_color"))]
    pub specular_color: OpaqueColor<CS>,
    /// Specular: perceptual roughness of the specular microfacets. Range
    /// `[0, 1]`, default 0.3.
    pub specular_roughness: f32,
    /// Specular: anisotropy of the specular roughness. Range `[0, 1]`,
    /// default 0.
    pub specular_roughness_anisotropy: f32,
    /// Specular: refractive index of the dielectric. Range `(0, ∞)`, norm
    /// `[1, 3]`, default 1.5.
    pub specular_ior: f32,

    /// Transmission: fraction of the dielectric base that is translucent.
    /// Range `[0, 1]`, default 0.
    pub transmission_weight: f32,
    /// Transmission: transmittance tint, reached at `transmission_depth`.
    /// Default (1, 1, 1).
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_color"))]
    pub transmission_color: OpaqueColor<CS>,
    /// Transmission: depth at which `transmission_color` is realized; 0 tints
    /// at the surface. Range `[0, ∞)` world units, norm `[0, 1]`, default 0.
    pub transmission_depth: f32,
    /// Transmission: scattering coefficient of the interior medium. Default
    /// (0, 0, 0).
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_color"))]
    pub transmission_scatter: OpaqueColor<CS>,
    /// Transmission: Henyey–Greenstein anisotropy of the interior medium.
    /// Range `[-1, 1]`, default 0.
    pub transmission_scatter_anisotropy: f32,
    /// Transmission: linear scale of dispersion. Range `[0, 1]`, default 0.
    pub transmission_dispersion_scale: f32,
    /// Transmission: Abbe number of the dielectric. Range `(0, ∞)`, norm
    /// `[9, 91]`, default 20.
    pub transmission_dispersion_abbe_number: f32,

    /// Subsurface: fraction of the dielectric base that scatters beneath the
    /// surface. Range `[0, 1]`, default 0.
    pub subsurface_weight: f32,
    /// Subsurface: observed reflection color of the medium. Default
    /// (0.8, 0.8, 0.8).
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_color"))]
    pub subsurface_color: OpaqueColor<CS>,
    /// Subsurface: length scale of the mean free path. Range `[0, ∞)` world
    /// units, norm `[0, 1]`, default 1.
    pub subsurface_radius: f32,
    /// Subsurface: per-channel multipliers of `subsurface_radius`. Default
    /// (1, 0.5, 0.25).
    pub subsurface_radius_scale: [f32; 3],
    /// Subsurface: Henyey–Greenstein anisotropy of the medium. Range
    /// `[-1, 1]`, default 0.
    pub subsurface_scatter_anisotropy: f32,

    /// Coat: coverage of the coat. Range `[0, 1]`, default 0.
    pub coat_weight: f32,
    /// Coat: square of the coat's normal-incidence transmittance. Default
    /// (1, 1, 1).
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_color"))]
    pub coat_color: OpaqueColor<CS>,
    /// Coat: perceptual roughness of the coat. Range `[0, 1]`, default 0.
    pub coat_roughness: f32,
    /// Coat: anisotropy of the coat roughness. Range `[0, 1]`, default 0.
    pub coat_roughness_anisotropy: f32,
    /// Coat: refractive index of the coat. Range `(0, ∞)`, norm `[1, 3]`,
    /// default 1.6.
    pub coat_ior: f32,
    /// Coat: modulates the physical darkening the coat causes. Range
    /// `[0, 1]`, default 1.
    pub coat_darkening: f32,

    /// Fuzz: coverage of the fuzz layer. Range `[0, 1]`, default 0.
    pub fuzz_weight: f32,
    /// Fuzz: color of the fuzz. Default (1, 1, 1).
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_color"))]
    pub fuzz_color: OpaqueColor<CS>,
    /// Fuzz: roughness of the fuzz. Range `[0, 1]`, default 0.5.
    pub fuzz_roughness: f32,

    /// Emission: luminance in nits (cd/m²). Range `[0, ∞)`, norm
    /// `[0, 1000]`, default 0.
    pub emission_luminance: f32,
    /// Emission: color multiplier. Range `[0, ∞)` per channel, default
    /// (1, 1, 1).
    #[cfg_attr(feature = "serde", serde(with = "crate::serde_color"))]
    pub emission_color: OpaqueColor<CS>,

    /// Thin film: coverage of the film. Range `[0, 1]`, default 0.
    pub thin_film_weight: f32,
    /// Thin film: thickness in micrometers. Range `[0, ∞)`, norm `[0, 1]`,
    /// default 0.5.
    pub thin_film_thickness: f32,
    /// Thin film: refractive index of the film. Range `(0, ∞)`, norm
    /// `[1, 3]`, default 1.4.
    pub thin_film_ior: f32,

    /// Geometry: coverage of the surface; 0 is absent. Range `[0, 1]`,
    /// default 1.
    pub geometry_opacity: f32,
    /// Geometry: treat the surface as a thin two-sided sheet rather than the
    /// boundary of a solid. Default false.
    pub geometry_thin_walled: bool,
}

impl<CS: ColorSpace> Parameters<CS> {
    const LINEAR: () = assert!(
        CS::IS_LINEAR,
        "OpenPBR color parameters need a linear color space"
    );

    /// The specification's default parameters.
    pub const DEFAULT: Self = {
        // Evaluating the assertion rejects non-linear color spaces.
        let () = Self::LINEAR;
        Self::SPEC_DEFAULT
    };

    const SPEC_DEFAULT: Self = Self {
        base_weight: 1.0,
        base_color: OpaqueColor::new([0.8; 3]),
        base_metalness: 0.0,
        base_diffuse_roughness: 0.0,

        specular_weight: 1.0,
        specular_color: OpaqueColor::new([1.0; 3]),
        specular_roughness: 0.3,
        specular_roughness_anisotropy: 0.0,
        specular_ior: 1.5,

        transmission_weight: 0.0,
        transmission_color: OpaqueColor::new([1.0; 3]),
        transmission_depth: 0.0,
        transmission_scatter: OpaqueColor::new([0.0; 3]),
        transmission_scatter_anisotropy: 0.0,
        transmission_dispersion_scale: 0.0,
        transmission_dispersion_abbe_number: 20.0,

        subsurface_weight: 0.0,
        subsurface_color: OpaqueColor::new([0.8; 3]),
        subsurface_radius: 1.0,
        subsurface_radius_scale: [1.0, 0.5, 0.25],
        subsurface_scatter_anisotropy: 0.0,

        coat_weight: 0.0,
        coat_color: OpaqueColor::new([1.0; 3]),
        coat_roughness: 0.0,
        coat_roughness_anisotropy: 0.0,
        coat_ior: 1.6,
        coat_darkening: 1.0,

        fuzz_weight: 0.0,
        fuzz_color: OpaqueColor::new([1.0; 3]),
        fuzz_roughness: 0.5,

        emission_luminance: 0.0,
        emission_color: OpaqueColor::new([1.0; 3]),

        thin_film_weight: 0.0,
        thin_film_thickness: 0.5,
        thin_film_ior: 1.4,

        geometry_opacity: 1.0,
        geometry_thin_walled: false,
    };

    /// Returns the value of `param`, or `None` for the `vector3` geometry
    /// parameters, which have no constant field.
    #[must_use]
    pub fn get(&self, param: Param) -> Option<Value<CS>> {
        use Value::{Boolean, Channels, Color, Float};
        Some(match param {
            Param::BaseWeight => Float(self.base_weight),
            Param::BaseColor => Color(self.base_color),
            Param::BaseMetalness => Float(self.base_metalness),
            Param::BaseDiffuseRoughness => Float(self.base_diffuse_roughness),
            Param::SpecularWeight => Float(self.specular_weight),
            Param::SpecularColor => Color(self.specular_color),
            Param::SpecularRoughness => Float(self.specular_roughness),
            Param::SpecularRoughnessAnisotropy => Float(self.specular_roughness_anisotropy),
            Param::SpecularIor => Float(self.specular_ior),
            Param::TransmissionWeight => Float(self.transmission_weight),
            Param::TransmissionColor => Color(self.transmission_color),
            Param::TransmissionDepth => Float(self.transmission_depth),
            Param::TransmissionScatter => Color(self.transmission_scatter),
            Param::TransmissionScatterAnisotropy => Float(self.transmission_scatter_anisotropy),
            Param::TransmissionDispersionScale => Float(self.transmission_dispersion_scale),
            Param::TransmissionDispersionAbbeNumber => {
                Float(self.transmission_dispersion_abbe_number)
            }
            Param::SubsurfaceWeight => Float(self.subsurface_weight),
            Param::SubsurfaceColor => Color(self.subsurface_color),
            Param::SubsurfaceRadius => Float(self.subsurface_radius),
            Param::SubsurfaceRadiusScale => Channels(self.subsurface_radius_scale),
            Param::SubsurfaceScatterAnisotropy => Float(self.subsurface_scatter_anisotropy),
            Param::CoatWeight => Float(self.coat_weight),
            Param::CoatColor => Color(self.coat_color),
            Param::CoatRoughness => Float(self.coat_roughness),
            Param::CoatRoughnessAnisotropy => Float(self.coat_roughness_anisotropy),
            Param::CoatIor => Float(self.coat_ior),
            Param::CoatDarkening => Float(self.coat_darkening),
            Param::FuzzWeight => Float(self.fuzz_weight),
            Param::FuzzColor => Color(self.fuzz_color),
            Param::FuzzRoughness => Float(self.fuzz_roughness),
            Param::EmissionLuminance => Float(self.emission_luminance),
            Param::EmissionColor => Color(self.emission_color),
            Param::ThinFilmWeight => Float(self.thin_film_weight),
            Param::ThinFilmThickness => Float(self.thin_film_thickness),
            Param::ThinFilmIor => Float(self.thin_film_ior),
            Param::GeometryOpacity => Float(self.geometry_opacity),
            Param::GeometryThinWalled => Boolean(self.geometry_thin_walled),
            Param::GeometryNormal
            | Param::GeometryTangent
            | Param::GeometryCoatNormal
            | Param::GeometryCoatTangent => return None,
        })
    }

    /// Sets `param` to `value`.
    ///
    /// The value is stored as given; use [`Parameters::validate`] to check
    /// ranges.
    ///
    /// # Errors
    ///
    /// Returns [`SetError::ValueMismatch`] when `value` does not fit the
    /// parameter (see [`Value`]), and [`SetError::NotConstant`] for the
    /// `vector3` geometry parameters.
    pub fn set(&mut self, param: Param, value: Value<CS>) -> Result<(), SetError> {
        match (self.field_mut(param), value) {
            (Field::None, _) => return Err(SetError::NotConstant { param }),
            (Field::Float(field), Value::Float(v)) => *field = v,
            (Field::Color(field), Value::Color(c)) => *field = c,
            (Field::Channels(field), Value::Channels(c)) => *field = c,
            (Field::Boolean(field), Value::Boolean(b)) => *field = b,
            _ => return Err(SetError::ValueMismatch { param }),
        }
        Ok(())
    }

    fn field_mut(&mut self, param: Param) -> Field<'_, CS> {
        use Field::{Boolean, Channels, Color, Float};
        match param {
            Param::BaseWeight => Float(&mut self.base_weight),
            Param::BaseColor => Color(&mut self.base_color),
            Param::BaseMetalness => Float(&mut self.base_metalness),
            Param::BaseDiffuseRoughness => Float(&mut self.base_diffuse_roughness),
            Param::SpecularWeight => Float(&mut self.specular_weight),
            Param::SpecularColor => Color(&mut self.specular_color),
            Param::SpecularRoughness => Float(&mut self.specular_roughness),
            Param::SpecularRoughnessAnisotropy => Float(&mut self.specular_roughness_anisotropy),
            Param::SpecularIor => Float(&mut self.specular_ior),
            Param::TransmissionWeight => Float(&mut self.transmission_weight),
            Param::TransmissionColor => Color(&mut self.transmission_color),
            Param::TransmissionDepth => Float(&mut self.transmission_depth),
            Param::TransmissionScatter => Color(&mut self.transmission_scatter),
            Param::TransmissionScatterAnisotropy => {
                Float(&mut self.transmission_scatter_anisotropy)
            }
            Param::TransmissionDispersionScale => Float(&mut self.transmission_dispersion_scale),
            Param::TransmissionDispersionAbbeNumber => {
                Float(&mut self.transmission_dispersion_abbe_number)
            }
            Param::SubsurfaceWeight => Float(&mut self.subsurface_weight),
            Param::SubsurfaceColor => Color(&mut self.subsurface_color),
            Param::SubsurfaceRadius => Float(&mut self.subsurface_radius),
            Param::SubsurfaceRadiusScale => Channels(&mut self.subsurface_radius_scale),
            Param::SubsurfaceScatterAnisotropy => Float(&mut self.subsurface_scatter_anisotropy),
            Param::CoatWeight => Float(&mut self.coat_weight),
            Param::CoatColor => Color(&mut self.coat_color),
            Param::CoatRoughness => Float(&mut self.coat_roughness),
            Param::CoatRoughnessAnisotropy => Float(&mut self.coat_roughness_anisotropy),
            Param::CoatIor => Float(&mut self.coat_ior),
            Param::CoatDarkening => Float(&mut self.coat_darkening),
            Param::FuzzWeight => Float(&mut self.fuzz_weight),
            Param::FuzzColor => Color(&mut self.fuzz_color),
            Param::FuzzRoughness => Float(&mut self.fuzz_roughness),
            Param::EmissionLuminance => Float(&mut self.emission_luminance),
            Param::EmissionColor => Color(&mut self.emission_color),
            Param::ThinFilmWeight => Float(&mut self.thin_film_weight),
            Param::ThinFilmThickness => Float(&mut self.thin_film_thickness),
            Param::ThinFilmIor => Float(&mut self.thin_film_ior),
            Param::GeometryOpacity => Float(&mut self.geometry_opacity),
            Param::GeometryThinWalled => Boolean(&mut self.geometry_thin_walled),
            Param::GeometryNormal
            | Param::GeometryTangent
            | Param::GeometryCoatNormal
            | Param::GeometryCoatTangent => Field::None,
        }
    }

    /// Returns every value outside its parameter's range, in parameter
    /// order and then channel order. NaN is never in range.
    ///
    /// Ranges are the specification's allowed values, not its "norm"
    /// (typically useful) ranges: an IOR of 4 is valid.
    pub fn violations(&self) -> impl Iterator<Item = RangeError> + '_ {
        Param::ALL.into_iter().flat_map(move |param| {
            let mut found = [None; 3];
            if let (Some(range), Some(value)) = (param.info().range, self.get(param)) {
                match value {
                    Value::Float(v) if !range.contains(v) => {
                        found[0] = Some(RangeError {
                            param,
                            channel: None,
                            value: v,
                        });
                    }
                    Value::Color(OpaqueColor { components: c, .. }) | Value::Channels(c) => {
                        for (channel, v) in (0_u8..).zip(c) {
                            if !range.contains(v) {
                                found[usize::from(channel)] = Some(RangeError {
                                    param,
                                    channel: Some(channel),
                                    value: v,
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
            found.into_iter().flatten()
        })
    }

    /// Checks every value against its parameter's range.
    ///
    /// # Errors
    ///
    /// Returns the first entry of [`Parameters::violations`].
    pub fn validate(&self) -> Result<(), RangeError> {
        self.violations().next().map_or(Ok(()), Err)
    }

    /// Converts the color parameters ([`Param::COLORS`]) to color space `T`.
    ///
    /// Every other value, including `subsurface_radius_scale`, is copied
    /// unchanged. Saturated colors can leave `[0, 1]` in a smaller gamut;
    /// validate afterwards when that matters.
    #[must_use]
    pub fn convert<T: ColorSpace>(self) -> Parameters<T> {
        Parameters {
            base_weight: self.base_weight,
            base_color: self.base_color.convert(),
            base_metalness: self.base_metalness,
            base_diffuse_roughness: self.base_diffuse_roughness,
            specular_weight: self.specular_weight,
            specular_color: self.specular_color.convert(),
            specular_roughness: self.specular_roughness,
            specular_roughness_anisotropy: self.specular_roughness_anisotropy,
            specular_ior: self.specular_ior,
            transmission_weight: self.transmission_weight,
            transmission_color: self.transmission_color.convert(),
            transmission_depth: self.transmission_depth,
            transmission_scatter: self.transmission_scatter.convert(),
            transmission_scatter_anisotropy: self.transmission_scatter_anisotropy,
            transmission_dispersion_scale: self.transmission_dispersion_scale,
            transmission_dispersion_abbe_number: self.transmission_dispersion_abbe_number,
            subsurface_weight: self.subsurface_weight,
            subsurface_color: self.subsurface_color.convert(),
            subsurface_radius: self.subsurface_radius,
            subsurface_radius_scale: self.subsurface_radius_scale,
            subsurface_scatter_anisotropy: self.subsurface_scatter_anisotropy,
            coat_weight: self.coat_weight,
            coat_color: self.coat_color.convert(),
            coat_roughness: self.coat_roughness,
            coat_roughness_anisotropy: self.coat_roughness_anisotropy,
            coat_ior: self.coat_ior,
            coat_darkening: self.coat_darkening,
            fuzz_weight: self.fuzz_weight,
            fuzz_color: self.fuzz_color.convert(),
            fuzz_roughness: self.fuzz_roughness,
            emission_luminance: self.emission_luminance,
            emission_color: self.emission_color.convert(),
            thin_film_weight: self.thin_film_weight,
            thin_film_thickness: self.thin_film_thickness,
            thin_film_ior: self.thin_film_ior,
            geometry_opacity: self.geometry_opacity,
            geometry_thin_walled: self.geometry_thin_walled,
        }
    }

    /// Returns true when every constant parameter equals its specification
    /// default.
    #[must_use]
    pub fn is_default(&self) -> bool {
        *self == Self::DEFAULT
    }
}

impl<CS: ColorSpace> Default for Parameters<CS> {
    fn default() -> Self {
        Self::DEFAULT
    }
}

// Hand-written so `CS` needs no `Debug`; colors print as their components.
impl<CS: ColorSpace> fmt::Debug for Parameters<CS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parameters")
            .field("base_weight", &self.base_weight)
            .field("base_color", &self.base_color.components)
            .field("base_metalness", &self.base_metalness)
            .field("base_diffuse_roughness", &self.base_diffuse_roughness)
            .field("specular_weight", &self.specular_weight)
            .field("specular_color", &self.specular_color.components)
            .field("specular_roughness", &self.specular_roughness)
            .field(
                "specular_roughness_anisotropy",
                &self.specular_roughness_anisotropy,
            )
            .field("specular_ior", &self.specular_ior)
            .field("transmission_weight", &self.transmission_weight)
            .field("transmission_color", &self.transmission_color.components)
            .field("transmission_depth", &self.transmission_depth)
            .field(
                "transmission_scatter",
                &self.transmission_scatter.components,
            )
            .field(
                "transmission_scatter_anisotropy",
                &self.transmission_scatter_anisotropy,
            )
            .field(
                "transmission_dispersion_scale",
                &self.transmission_dispersion_scale,
            )
            .field(
                "transmission_dispersion_abbe_number",
                &self.transmission_dispersion_abbe_number,
            )
            .field("subsurface_weight", &self.subsurface_weight)
            .field("subsurface_color", &self.subsurface_color.components)
            .field("subsurface_radius", &self.subsurface_radius)
            .field("subsurface_radius_scale", &self.subsurface_radius_scale)
            .field(
                "subsurface_scatter_anisotropy",
                &self.subsurface_scatter_anisotropy,
            )
            .field("coat_weight", &self.coat_weight)
            .field("coat_color", &self.coat_color.components)
            .field("coat_roughness", &self.coat_roughness)
            .field("coat_roughness_anisotropy", &self.coat_roughness_anisotropy)
            .field("coat_ior", &self.coat_ior)
            .field("coat_darkening", &self.coat_darkening)
            .field("fuzz_weight", &self.fuzz_weight)
            .field("fuzz_color", &self.fuzz_color.components)
            .field("fuzz_roughness", &self.fuzz_roughness)
            .field("emission_luminance", &self.emission_luminance)
            .field("emission_color", &self.emission_color.components)
            .field("thin_film_weight", &self.thin_film_weight)
            .field("thin_film_thickness", &self.thin_film_thickness)
            .field("thin_film_ior", &self.thin_film_ior)
            .field("geometry_opacity", &self.geometry_opacity)
            .field("geometry_thin_walled", &self.geometry_thin_walled)
            .finish()
    }
}

// Hand-written so `CS` needs no `PartialEq`, as for `OpaqueColor`.
impl<CS: ColorSpace> PartialEq for Parameters<CS> {
    fn eq(&self, other: &Self) -> bool {
        self.base_weight == other.base_weight
            && self.base_color == other.base_color
            && self.base_metalness == other.base_metalness
            && self.base_diffuse_roughness == other.base_diffuse_roughness
            && self.specular_weight == other.specular_weight
            && self.specular_color == other.specular_color
            && self.specular_roughness == other.specular_roughness
            && self.specular_roughness_anisotropy == other.specular_roughness_anisotropy
            && self.specular_ior == other.specular_ior
            && self.transmission_weight == other.transmission_weight
            && self.transmission_color == other.transmission_color
            && self.transmission_depth == other.transmission_depth
            && self.transmission_scatter == other.transmission_scatter
            && self.transmission_scatter_anisotropy == other.transmission_scatter_anisotropy
            && self.transmission_dispersion_scale == other.transmission_dispersion_scale
            && self.transmission_dispersion_abbe_number == other.transmission_dispersion_abbe_number
            && self.subsurface_weight == other.subsurface_weight
            && self.subsurface_color == other.subsurface_color
            && self.subsurface_radius == other.subsurface_radius
            && self.subsurface_radius_scale == other.subsurface_radius_scale
            && self.subsurface_scatter_anisotropy == other.subsurface_scatter_anisotropy
            && self.coat_weight == other.coat_weight
            && self.coat_color == other.coat_color
            && self.coat_roughness == other.coat_roughness
            && self.coat_roughness_anisotropy == other.coat_roughness_anisotropy
            && self.coat_ior == other.coat_ior
            && self.coat_darkening == other.coat_darkening
            && self.fuzz_weight == other.fuzz_weight
            && self.fuzz_color == other.fuzz_color
            && self.fuzz_roughness == other.fuzz_roughness
            && self.emission_luminance == other.emission_luminance
            && self.emission_color == other.emission_color
            && self.thin_film_weight == other.thin_film_weight
            && self.thin_film_thickness == other.thin_film_thickness
            && self.thin_film_ior == other.thin_film_ior
            && self.geometry_opacity == other.geometry_opacity
            && self.geometry_thin_walled == other.geometry_thin_walled
    }
}

enum Field<'a, CS: ColorSpace> {
    Float(&'a mut f32),
    Color(&'a mut OpaqueColor<CS>),
    Channels(&'a mut [f32; 3]),
    Boolean(&'a mut bool),
    None,
}

/// A parameter value outside the parameter's allowed range.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RangeError {
    /// The parameter.
    pub param: Param,
    /// The color channel (0 = R, 1 = G, 2 = B), or `None` for a `float`.
    pub channel: Option<u8>,
    /// The rejected value.
    pub value: f32,
}

impl fmt::Display for RangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let range = self
            .param
            .info()
            .range
            .expect("only ranged parameters report range errors");
        match self.channel {
            Some(channel) => write!(
                f,
                "{}[{channel}] = {} is outside {range}",
                self.param, self.value
            ),
            None => write!(f, "{} = {} is outside {range}", self.param, self.value),
        }
    }
}

impl core::error::Error for RangeError {}

/// Failure from [`Parameters::set`].
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SetError {
    /// The value's variant does not fit the parameter: a `float` needs
    /// [`Value::Float`], a color [`Value::Color`], and
    /// `subsurface_radius_scale` [`Value::Channels`].
    ValueMismatch {
        /// The parameter.
        param: Param,
    },
    /// The parameter has no constant value in [`Parameters`].
    NotConstant {
        /// The parameter.
        param: Param,
    },
}

impl fmt::Display for SetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValueMismatch { param } => {
                write!(
                    f,
                    "value does not fit {} parameter {param}",
                    kind_name(*param)
                )
            }
            Self::NotConstant { param } => {
                write!(f, "{param} has no constant value")
            }
        }
    }
}

impl core::error::Error for SetError {}

fn kind_name(param: Param) -> &'static str {
    let info = param.info();
    match info.kind {
        Kind::Color3 if !info.color => "per-channel color3",
        kind => kind.name(),
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;

    use color::LinearSrgb;

    use super::*;
    use crate::ParamDefault;

    fn default_value(param: Param) -> Option<Value<AcesCg>> {
        let info = param.info();
        match info.default {
            ParamDefault::Float(v) => Some(Value::Float(v)),
            ParamDefault::Boolean(b) => Some(Value::Boolean(b)),
            ParamDefault::Color3(c) if info.color => Some(Value::Color(OpaqueColor::new(c))),
            ParamDefault::Color3(c) => Some(Value::Channels(c)),
            ParamDefault::UnperturbedNormal | ParamDefault::UnperturbedTangent => None,
        }
    }

    /// A linear color space that is neither `Debug` nor `PartialEq`:
    /// `ColorSpace` requires only `Clone + Copy`.
    #[derive(Clone, Copy)]
    struct Plain;

    impl ColorSpace for Plain {
        const IS_LINEAR: bool = true;
        const WHITE_COMPONENTS: [f32; 3] = [1.0; 3];

        fn to_linear_srgb(src: [f32; 3]) -> [f32; 3] {
            src
        }

        fn from_linear_srgb(src: [f32; 3]) -> [f32; 3] {
            src
        }

        fn clip(src: [f32; 3]) -> [f32; 3] {
            src
        }
    }

    #[test]
    fn trait_impls_need_only_color_space() {
        let p = Parameters::<Plain>::DEFAULT;
        assert_eq!(p, p);
        let text = alloc::format!("{p:?}");
        assert!(
            text.starts_with("Parameters { base_weight: 1.0, base_color: [0.8, 0.8, 0.8],"),
            "{text}"
        );
        assert!(text.ends_with("geometry_thin_walled: false }"), "{text}");
        assert_eq!(
            alloc::format!("{:?}", p.get(Param::BaseColor).unwrap()),
            "Color([0.8, 0.8, 0.8])"
        );
    }

    #[test]
    fn defaults_match_the_parameter_reference() {
        let defaults = Parameters::<AcesCg>::DEFAULT;
        assert!(defaults.is_default());
        assert_eq!(defaults.validate(), Ok(()));
        for param in Param::ALL {
            assert_eq!(defaults.get(param), default_value(param), "{param}");
        }
    }

    #[test]
    fn spot_check_defaults_against_the_specification() {
        let d = Parameters::<AcesCg>::default();
        assert_eq!(d.base_color.components, [0.8; 3]);
        assert_eq!(d.specular_roughness, 0.3);
        assert_eq!(d.specular_ior, 1.5);
        assert_eq!(d.coat_ior, 1.6);
        assert_eq!(d.fuzz_roughness, 0.5);
        assert_eq!(d.subsurface_radius_scale, [1.0, 0.5, 0.25]);
        assert_eq!(d.transmission_dispersion_abbe_number, 20.0);
        assert_eq!(d.thin_film_thickness, 0.5);
        assert_eq!(d.thin_film_ior, 1.4);
    }

    #[test]
    fn set_round_trips_and_checks_kinds() {
        let mut p = Parameters::<AcesCg>::DEFAULT;
        p.set(Param::CoatWeight, Value::Float(0.5)).unwrap();
        p.set(
            Param::FuzzColor,
            Value::Color(OpaqueColor::new([0.1, 0.2, 0.3])),
        )
        .unwrap();
        p.set(Param::GeometryThinWalled, Value::Boolean(true))
            .unwrap();
        assert_eq!(p.coat_weight, 0.5);
        assert_eq!(p.fuzz_color.components, [0.1, 0.2, 0.3]);
        assert!(p.geometry_thin_walled);
        assert!(!p.is_default());
        assert_eq!(
            p.set(Param::CoatWeight, Value::Channels([1.0; 3])),
            Err(SetError::ValueMismatch {
                param: Param::CoatWeight
            })
        );
        assert_eq!(
            p.set(
                Param::SubsurfaceRadiusScale,
                Value::Color(OpaqueColor::WHITE)
            ),
            Err(SetError::ValueMismatch {
                param: Param::SubsurfaceRadiusScale
            })
        );
        assert_eq!(
            p.set(Param::GeometryNormal, Value::Float(0.0)),
            Err(SetError::NotConstant {
                param: Param::GeometryNormal
            })
        );
        for param in Param::ALL {
            if let Some(value) = Parameters::<AcesCg>::DEFAULT.get(param) {
                p.set(param, value).unwrap();
            }
        }
        assert_eq!(p, Parameters::DEFAULT, "set reaches every stored field");
    }

    #[test]
    fn violations_report_every_offending_value() {
        let p = Parameters::<AcesCg> {
            base_weight: 1.5,
            specular_ior: 0.0,
            emission_color: OpaqueColor::new([2.0, -1.0, 1.0]),
            specular_weight: 4.0,
            thin_film_ior: 5.0,
            ..Parameters::DEFAULT
        };
        let found: Vec<_> = p.violations().collect();
        assert_eq!(
            found,
            [
                RangeError {
                    param: Param::BaseWeight,
                    channel: None,
                    value: 1.5
                },
                RangeError {
                    param: Param::SpecularIor,
                    channel: None,
                    value: 0.0
                },
                RangeError {
                    param: Param::EmissionColor,
                    channel: Some(1),
                    value: -1.0
                },
            ][..],
            "unbounded weights, norm excesses and bright emission are valid"
        );
        // NaN compares unequal, so check it separately.
        let nan_only = Parameters::<AcesCg> {
            emission_color: OpaqueColor::new([1.0, 1.0, f32::NAN]),
            ..Parameters::DEFAULT
        };
        let error = nan_only.validate().unwrap_err();
        assert_eq!(error.param, Param::EmissionColor);
        assert_eq!(error.channel, Some(2));
        assert!(error.value.is_nan());
        assert_eq!(
            alloc::format!("{}", p.validate().unwrap_err()),
            "base_weight = 1.5 is outside [0, 1]"
        );
    }

    #[test]
    fn color_conversion_leaves_non_colors_and_neutrals_alone() {
        let p = Parameters::<LinearSrgb> {
            base_color: OpaqueColor::new([0.5, 0.25, 0.125]),
            ..Parameters::DEFAULT
        };
        let converted: Parameters<AcesCg> = p.convert();
        assert_ne!(converted.base_color.components, p.base_color.components);
        assert_eq!(converted.subsurface_radius_scale, [1.0, 0.5, 0.25]);
        for c in converted.specular_color.components {
            assert!((c - 1.0).abs() < 1e-6, "white stays white");
        }
        for c in converted.subsurface_color.components {
            assert!((c - 0.8).abs() < 1e-6, "the gray default stays gray");
        }
        let back: Parameters<LinearSrgb> = converted.convert();
        for (a, b) in back
            .base_color
            .components
            .iter()
            .zip(p.base_color.components)
        {
            assert!((a - b).abs() < 1e-5, "{a} vs {b}");
        }
    }
}
