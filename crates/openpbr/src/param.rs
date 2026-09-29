// Copyright 2026 the openpbr crate Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! The parameter reference: every OpenPBR parameter and its metadata.

use core::fmt;

use color::OpaqueColor;

use crate::LinearRgb;

/// One OpenPBR parameter.
///
/// Variants follow the order of the specification's parameter reference.
/// [`Param::identifier`] is the stable, spec-defined name; the enum may grow
/// when a later specification version adds parameters.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Param {
    /// `base_weight`
    BaseWeight,
    /// `base_color`
    BaseColor,
    /// `base_metalness`
    BaseMetalness,
    /// `base_diffuse_roughness`
    BaseDiffuseRoughness,
    /// `specular_weight`
    SpecularWeight,
    /// `specular_color`
    SpecularColor,
    /// `specular_roughness`
    SpecularRoughness,
    /// `specular_roughness_anisotropy`
    SpecularRoughnessAnisotropy,
    /// `specular_ior`
    SpecularIor,
    /// `transmission_weight`
    TransmissionWeight,
    /// `transmission_color`
    TransmissionColor,
    /// `transmission_depth`
    TransmissionDepth,
    /// `transmission_scatter`
    TransmissionScatter,
    /// `transmission_scatter_anisotropy`
    TransmissionScatterAnisotropy,
    /// `transmission_dispersion_scale`
    TransmissionDispersionScale,
    /// `transmission_dispersion_abbe_number`
    TransmissionDispersionAbbeNumber,
    /// `subsurface_weight`
    SubsurfaceWeight,
    /// `subsurface_color`
    SubsurfaceColor,
    /// `subsurface_radius`
    SubsurfaceRadius,
    /// `subsurface_radius_scale`
    SubsurfaceRadiusScale,
    /// `subsurface_scatter_anisotropy`
    SubsurfaceScatterAnisotropy,
    /// `coat_weight`
    CoatWeight,
    /// `coat_color`
    CoatColor,
    /// `coat_roughness`
    CoatRoughness,
    /// `coat_roughness_anisotropy`
    CoatRoughnessAnisotropy,
    /// `coat_ior`
    CoatIor,
    /// `coat_darkening`
    CoatDarkening,
    /// `fuzz_weight`
    FuzzWeight,
    /// `fuzz_color`
    FuzzColor,
    /// `fuzz_roughness`
    FuzzRoughness,
    /// `emission_luminance`
    EmissionLuminance,
    /// `emission_color`
    EmissionColor,
    /// `thin_film_weight`
    ThinFilmWeight,
    /// `thin_film_thickness`
    ThinFilmThickness,
    /// `thin_film_ior`
    ThinFilmIor,
    /// `geometry_opacity`
    GeometryOpacity,
    /// `geometry_thin_walled`
    GeometryThinWalled,
    /// `geometry_normal`
    GeometryNormal,
    /// `geometry_tangent`
    GeometryTangent,
    /// `geometry_coat_normal`
    GeometryCoatNormal,
    /// `geometry_coat_tangent`
    GeometryCoatTangent,
}

/// Parameter group, named by the identifier prefix its parameters share.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Group {
    /// The base substrate (`base_*`).
    Base,
    /// Dielectric and metallic specular reflection (`specular_*`).
    Specular,
    /// The translucent dielectric base (`transmission_*`).
    Transmission,
    /// The subsurface-scattering base (`subsurface_*`).
    Subsurface,
    /// The clear coat (`coat_*`).
    Coat,
    /// The fuzz (sheen) layer (`fuzz_*`).
    Fuzz,
    /// Emission (`emission_*`).
    Emission,
    /// Thin-film iridescence (`thin_film_*`).
    ThinFilm,
    /// Geometry: opacity, thin-walled mode and shading frames (`geometry_*`).
    Geometry,
}

impl Group {
    /// Every group, in specification order.
    pub const ALL: [Self; 9] = [
        Self::Base,
        Self::Specular,
        Self::Transmission,
        Self::Subsurface,
        Self::Coat,
        Self::Fuzz,
        Self::Emission,
        Self::ThinFilm,
        Self::Geometry,
    ];

    /// The identifier prefix shared by the group's parameters, without the
    /// trailing underscore.
    #[must_use]
    pub const fn prefix(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Specular => "specular",
            Self::Transmission => "transmission",
            Self::Subsurface => "subsurface",
            Self::Coat => "coat",
            Self::Fuzz => "fuzz",
            Self::Emission => "emission",
            Self::ThinFilm => "thin_film",
            Self::Geometry => "geometry",
        }
    }

    /// The group heading the specification's parameter reference uses.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Base => "Base",
            Self::Specular => "Specular",
            Self::Transmission => "Transmission",
            Self::Subsurface => "Subsurface",
            Self::Coat => "Coat",
            Self::Fuzz => "Fuzz",
            Self::Emission => "Emission",
            Self::ThinFilm => "Thin-film",
            Self::Geometry => "Geometry",
        }
    }
}

/// Value type of a parameter, as the specification names it.
///
/// A later specification version may add types.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Kind {
    /// `float`: one scalar.
    Float,
    /// `boolean`.
    Boolean,
    /// `color3`: three RGB channels. Most are colors in the parameter set's
    /// color space; see [`ParamInfo::color`].
    Color3,
    /// `vector3`: a raw three-component vector.
    Vector3,
}

impl Kind {
    /// The specification's type name.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Float => "float",
            Self::Boolean => "boolean",
            Self::Color3 => "color3",
            Self::Vector3 => "vector3",
        }
    }
}

/// One end of a [`Range`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Bound {
    /// The value may equal this bound.
    Inclusive(f32),
    /// The value must lie strictly beyond this bound.
    Exclusive(f32),
    /// No bound on this side.
    Unbounded,
}

/// A scalar interval, applied per channel for color parameters.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Range {
    /// Lower bound.
    pub min: Bound,
    /// Upper bound.
    pub max: Bound,
}

impl Range {
    /// The closed interval `[min, max]`.
    #[must_use]
    pub const fn closed(min: f32, max: f32) -> Self {
        Self {
            min: Bound::Inclusive(min),
            max: Bound::Inclusive(max),
        }
    }

    /// The interval `[min, ∞)`.
    #[must_use]
    pub const fn at_least(min: f32) -> Self {
        Self {
            min: Bound::Inclusive(min),
            max: Bound::Unbounded,
        }
    }

    /// The interval `(min, ∞)`.
    #[must_use]
    pub const fn greater_than(min: f32) -> Self {
        Self {
            min: Bound::Exclusive(min),
            max: Bound::Unbounded,
        }
    }

    /// Returns true when `value` is finite and lies in the interval.
    ///
    /// An unbounded interval admits arbitrarily large finite values, not
    /// IEEE infinities. NaN is never in range.
    #[must_use]
    pub fn contains(self, value: f32) -> bool {
        if !value.is_finite() {
            return false;
        }
        let above = match self.min {
            Bound::Inclusive(min) => value >= min,
            Bound::Exclusive(min) => value > min,
            Bound::Unbounded => true,
        };
        let below = match self.max {
            Bound::Inclusive(max) => value <= max,
            Bound::Exclusive(max) => value < max,
            Bound::Unbounded => true,
        };
        above && below
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.min {
            Bound::Inclusive(v) => write!(f, "[{v}, ")?,
            Bound::Exclusive(v) => write!(f, "({v}, ")?,
            Bound::Unbounded => f.write_str("(-inf, ")?,
        }
        match self.max {
            Bound::Inclusive(v) => write!(f, "{v}]"),
            Bound::Exclusive(v) => write!(f, "{v})"),
            Bound::Unbounded => f.write_str("inf)"),
        }
    }
}

/// Physical unit of a parameter.
///
/// A later specification version may add units.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum Unit {
    /// Dimensionless.
    None,
    /// A length in the scene's world-space units.
    Length,
    /// Luminance in nits (cd/m²).
    Nits,
    /// A length in micrometers, independent of scene units.
    Micrometers,
}

/// A constant parameter value, with colors in linear RGB space `CS`.
#[derive(Copy, Clone)]
pub enum Value<CS: LinearRgb> {
    /// A `float` value.
    Float(f32),
    /// A `boolean` value.
    Boolean(bool),
    /// A `color3` value that is a color ([`ParamInfo::color`]).
    Color(OpaqueColor<CS>),
    /// A `color3` value that holds per-channel factors rather than a color
    /// (`subsurface_radius_scale`).
    Channels([f32; 3]),
}

// Hand-written so `CS` needs no `Debug`; colors print as their components.
impl<CS: LinearRgb> fmt::Debug for Value<CS> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Float(v) => f.debug_tuple("Float").field(v).finish(),
            Self::Boolean(b) => f.debug_tuple("Boolean").field(b).finish(),
            Self::Color(c) => f.debug_tuple("Color").field(&c.components).finish(),
            Self::Channels(c) => f.debug_tuple("Channels").field(c).finish(),
        }
    }
}

// Hand-written so `CS` needs no `PartialEq`, as for `OpaqueColor`.
impl<CS: LinearRgb> PartialEq for Value<CS> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Float(a), Self::Float(b)) => a == b,
            (Self::Boolean(a), Self::Boolean(b)) => a == b,
            (Self::Color(a), Self::Color(b)) => a == b,
            (Self::Channels(a), Self::Channels(b)) => a == b,
            _ => false,
        }
    }
}

/// The specification's default for a parameter.
///
/// Color defaults are neutral (gray, white or black), so their components are
/// the same in every linear RGB color space that shares the conversion's
/// adapted white.
///
/// A later specification version may add kinds of default.
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ParamDefault {
    /// A `float` default.
    Float(f32),
    /// A `boolean` default.
    Boolean(bool),
    /// A `color3` default, as RGB components.
    Color3([f32; 3]),
    /// The surface's unperturbed shading normal (`geometry_normal`,
    /// `geometry_coat_normal`).
    UnperturbedNormal,
    /// The surface's unperturbed reference tangent (`geometry_tangent`,
    /// `geometry_coat_tangent`).
    UnperturbedTangent,
}

/// Metadata for one parameter, from the specification's parameter reference.
///
/// Only this crate constructs it; fields may be added as the specification
/// grows.
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ParamInfo {
    /// The parameter.
    pub param: Param,
    /// The unique, stable identifier, for example `"specular_ior"`.
    pub identifier: &'static str,
    /// The suggested (non-unique) UI label, for example `"IOR"`.
    pub label: &'static str,
    /// The group the identifier prefix names.
    pub group: Group,
    /// The value type.
    pub kind: Kind,
    /// The specification's allowed values (per channel for colors); `None`
    /// for `boolean` and `vector3` parameters. These bounds are unchanged by
    /// extended RGB storage: [`crate::Parameters::validate_spec_ranges`]
    /// checks them for colors, while [`crate::Parameters::validate`] only
    /// requires chromatic components to be finite.
    pub range: Option<Range>,
    /// The typically useful values when they differ from `range` (the
    /// specification's "Norm" column), for example `[1, 3]` for an IOR.
    pub norm: Option<Range>,
    /// The suggested default.
    pub default: ParamDefault,
    /// The physical unit.
    pub unit: Unit,
    /// True when the `color3` value is a color, which changes under color
    /// space conversion. False for every other kind and for
    /// `subsurface_radius_scale`, whose channels are factors of a length.
    pub color: bool,
    /// True when the parameter cannot vary over a surface. Only
    /// `geometry_thin_walled` is uniform, following the MaterialX reference
    /// implementation; every other parameter may be driven by a texture.
    pub uniform: bool,
}

const fn float(
    param: Param,
    identifier: &'static str,
    label: &'static str,
    group: Group,
    range: Range,
    norm: Option<Range>,
    default: f32,
    unit: Unit,
) -> ParamInfo {
    ParamInfo {
        param,
        identifier,
        label,
        group,
        kind: Kind::Float,
        range: Some(range),
        norm,
        default: ParamDefault::Float(default),
        unit,
        color: false,
        uniform: false,
    }
}

const fn color(
    param: Param,
    identifier: &'static str,
    label: &'static str,
    group: Group,
    range: Range,
    default: [f32; 3],
) -> ParamInfo {
    ParamInfo {
        param,
        identifier,
        label,
        group,
        kind: Kind::Color3,
        range: Some(range),
        norm: None,
        default: ParamDefault::Color3(default),
        unit: Unit::None,
        color: true,
        uniform: false,
    }
}

const fn vector(
    param: Param,
    identifier: &'static str,
    label: &'static str,
    default: ParamDefault,
) -> ParamInfo {
    ParamInfo {
        param,
        identifier,
        label,
        group: Geometry,
        kind: Kind::Vector3,
        range: None,
        norm: None,
        default,
        unit: Unit::None,
        color: false,
        uniform: false,
    }
}

const UNIT: Range = Range::closed(0.0, 1.0);
const SIGNED_UNIT: Range = Range::closed(-1.0, 1.0);
const NON_NEGATIVE: Range = Range::at_least(0.0);
const POSITIVE: Range = Range::greater_than(0.0);
const IOR_NORM: Option<Range> = Some(Range::closed(1.0, 3.0));

use Group::{Base, Coat, Emission, Fuzz, Geometry, Specular, Subsurface, ThinFilm, Transmission};
use Param as P;

/// The parameter reference, indexed by `Param as usize`.
static TABLE: [ParamInfo; Param::COUNT] = [
    float(
        P::BaseWeight,
        "base_weight",
        "Weight",
        Base,
        UNIT,
        None,
        1.0,
        Unit::None,
    ),
    color(P::BaseColor, "base_color", "Color", Base, UNIT, [0.8; 3]),
    float(
        P::BaseMetalness,
        "base_metalness",
        "Metalness",
        Base,
        UNIT,
        None,
        0.0,
        Unit::None,
    ),
    float(
        P::BaseDiffuseRoughness,
        "base_diffuse_roughness",
        "Diffuse Roughness",
        Base,
        UNIT,
        None,
        0.0,
        Unit::None,
    ),
    float(
        P::SpecularWeight,
        "specular_weight",
        "Weight",
        Specular,
        NON_NEGATIVE,
        Some(UNIT),
        1.0,
        Unit::None,
    ),
    color(
        P::SpecularColor,
        "specular_color",
        "Color",
        Specular,
        UNIT,
        [1.0; 3],
    ),
    float(
        P::SpecularRoughness,
        "specular_roughness",
        "Roughness",
        Specular,
        UNIT,
        None,
        0.3,
        Unit::None,
    ),
    float(
        P::SpecularRoughnessAnisotropy,
        "specular_roughness_anisotropy",
        "Anisotropy",
        Specular,
        UNIT,
        None,
        0.0,
        Unit::None,
    ),
    float(
        P::SpecularIor,
        "specular_ior",
        "IOR",
        Specular,
        POSITIVE,
        IOR_NORM,
        1.5,
        Unit::None,
    ),
    float(
        P::TransmissionWeight,
        "transmission_weight",
        "Weight",
        Transmission,
        UNIT,
        None,
        0.0,
        Unit::None,
    ),
    color(
        P::TransmissionColor,
        "transmission_color",
        "Color",
        Transmission,
        UNIT,
        [1.0; 3],
    ),
    float(
        P::TransmissionDepth,
        "transmission_depth",
        "Depth",
        Transmission,
        NON_NEGATIVE,
        Some(UNIT),
        0.0,
        Unit::Length,
    ),
    color(
        P::TransmissionScatter,
        "transmission_scatter",
        "Scatter",
        Transmission,
        UNIT,
        [0.0; 3],
    ),
    float(
        P::TransmissionScatterAnisotropy,
        "transmission_scatter_anisotropy",
        "Anisotropy",
        Transmission,
        SIGNED_UNIT,
        None,
        0.0,
        Unit::None,
    ),
    float(
        P::TransmissionDispersionScale,
        "transmission_dispersion_scale",
        "Dispersion scale",
        Transmission,
        UNIT,
        None,
        0.0,
        Unit::None,
    ),
    float(
        P::TransmissionDispersionAbbeNumber,
        "transmission_dispersion_abbe_number",
        "Abbe number",
        Transmission,
        POSITIVE,
        Some(Range::closed(9.0, 91.0)),
        20.0,
        Unit::None,
    ),
    float(
        P::SubsurfaceWeight,
        "subsurface_weight",
        "Weight",
        Subsurface,
        UNIT,
        None,
        0.0,
        Unit::None,
    ),
    color(
        P::SubsurfaceColor,
        "subsurface_color",
        "Color",
        Subsurface,
        UNIT,
        [0.8; 3],
    ),
    float(
        P::SubsurfaceRadius,
        "subsurface_radius",
        "Radius",
        Subsurface,
        NON_NEGATIVE,
        Some(UNIT),
        1.0,
        Unit::Length,
    ),
    ParamInfo {
        color: false,
        ..color(
            P::SubsurfaceRadiusScale,
            "subsurface_radius_scale",
            "Radius scale",
            Subsurface,
            UNIT,
            [1.0, 0.5, 0.25],
        )
    },
    float(
        P::SubsurfaceScatterAnisotropy,
        "subsurface_scatter_anisotropy",
        "Anisotropy",
        Subsurface,
        SIGNED_UNIT,
        None,
        0.0,
        Unit::None,
    ),
    float(
        P::CoatWeight,
        "coat_weight",
        "Weight",
        Coat,
        UNIT,
        None,
        0.0,
        Unit::None,
    ),
    color(P::CoatColor, "coat_color", "Color", Coat, UNIT, [1.0; 3]),
    float(
        P::CoatRoughness,
        "coat_roughness",
        "Roughness",
        Coat,
        UNIT,
        None,
        0.0,
        Unit::None,
    ),
    float(
        P::CoatRoughnessAnisotropy,
        "coat_roughness_anisotropy",
        "Anisotropy",
        Coat,
        UNIT,
        None,
        0.0,
        Unit::None,
    ),
    float(
        P::CoatIor,
        "coat_ior",
        "IOR",
        Coat,
        POSITIVE,
        IOR_NORM,
        1.6,
        Unit::None,
    ),
    float(
        P::CoatDarkening,
        "coat_darkening",
        "Darkening",
        Coat,
        UNIT,
        None,
        1.0,
        Unit::None,
    ),
    float(
        P::FuzzWeight,
        "fuzz_weight",
        "Weight",
        Fuzz,
        UNIT,
        None,
        0.0,
        Unit::None,
    ),
    color(P::FuzzColor, "fuzz_color", "Color", Fuzz, UNIT, [1.0; 3]),
    float(
        P::FuzzRoughness,
        "fuzz_roughness",
        "Roughness",
        Fuzz,
        UNIT,
        None,
        0.5,
        Unit::None,
    ),
    float(
        P::EmissionLuminance,
        "emission_luminance",
        "Luminance",
        Emission,
        NON_NEGATIVE,
        Some(Range::closed(0.0, 1000.0)),
        0.0,
        Unit::Nits,
    ),
    color(
        P::EmissionColor,
        "emission_color",
        "Color",
        Emission,
        NON_NEGATIVE,
        [1.0; 3],
    ),
    float(
        P::ThinFilmWeight,
        "thin_film_weight",
        "Weight",
        ThinFilm,
        UNIT,
        None,
        0.0,
        Unit::None,
    ),
    float(
        P::ThinFilmThickness,
        "thin_film_thickness",
        "Thickness",
        ThinFilm,
        NON_NEGATIVE,
        Some(UNIT),
        0.5,
        Unit::Micrometers,
    ),
    float(
        P::ThinFilmIor,
        "thin_film_ior",
        "IOR",
        ThinFilm,
        POSITIVE,
        IOR_NORM,
        1.4,
        Unit::None,
    ),
    float(
        P::GeometryOpacity,
        "geometry_opacity",
        "Opacity",
        Geometry,
        UNIT,
        None,
        1.0,
        Unit::None,
    ),
    ParamInfo {
        param: P::GeometryThinWalled,
        identifier: "geometry_thin_walled",
        label: "Thin walled",
        group: Geometry,
        kind: Kind::Boolean,
        range: None,
        norm: None,
        default: ParamDefault::Boolean(false),
        unit: Unit::None,
        color: false,
        uniform: true,
    },
    vector(
        P::GeometryNormal,
        "geometry_normal",
        "Normal",
        ParamDefault::UnperturbedNormal,
    ),
    vector(
        P::GeometryTangent,
        "geometry_tangent",
        "Tangent",
        ParamDefault::UnperturbedTangent,
    ),
    vector(
        P::GeometryCoatNormal,
        "geometry_coat_normal",
        "Coat Normal",
        ParamDefault::UnperturbedNormal,
    ),
    vector(
        P::GeometryCoatTangent,
        "geometry_coat_tangent",
        "Coat Tangent",
        ParamDefault::UnperturbedTangent,
    ),
];

impl Param {
    /// Number of parameters in the specification.
    pub const COUNT: usize = 41;

    /// Every parameter, in specification order.
    pub const ALL: [Self; Self::COUNT] = [
        Self::BaseWeight,
        Self::BaseColor,
        Self::BaseMetalness,
        Self::BaseDiffuseRoughness,
        Self::SpecularWeight,
        Self::SpecularColor,
        Self::SpecularRoughness,
        Self::SpecularRoughnessAnisotropy,
        Self::SpecularIor,
        Self::TransmissionWeight,
        Self::TransmissionColor,
        Self::TransmissionDepth,
        Self::TransmissionScatter,
        Self::TransmissionScatterAnisotropy,
        Self::TransmissionDispersionScale,
        Self::TransmissionDispersionAbbeNumber,
        Self::SubsurfaceWeight,
        Self::SubsurfaceColor,
        Self::SubsurfaceRadius,
        Self::SubsurfaceRadiusScale,
        Self::SubsurfaceScatterAnisotropy,
        Self::CoatWeight,
        Self::CoatColor,
        Self::CoatRoughness,
        Self::CoatRoughnessAnisotropy,
        Self::CoatIor,
        Self::CoatDarkening,
        Self::FuzzWeight,
        Self::FuzzColor,
        Self::FuzzRoughness,
        Self::EmissionLuminance,
        Self::EmissionColor,
        Self::ThinFilmWeight,
        Self::ThinFilmThickness,
        Self::ThinFilmIor,
        Self::GeometryOpacity,
        Self::GeometryThinWalled,
        Self::GeometryNormal,
        Self::GeometryTangent,
        Self::GeometryCoatNormal,
        Self::GeometryCoatTangent,
    ];

    /// The parameters whose values are colors ([`ParamInfo::color`]), in
    /// specification order.
    ///
    /// `subsurface_radius_scale` is typed `color3` but holds per-channel
    /// factors of a length, so it is not a color.
    pub const COLORS: [Self; 8] = [
        Self::BaseColor,
        Self::SpecularColor,
        Self::TransmissionColor,
        Self::TransmissionScatter,
        Self::SubsurfaceColor,
        Self::CoatColor,
        Self::FuzzColor,
        Self::EmissionColor,
    ];

    /// Returns the parameter's metadata.
    #[must_use]
    pub fn info(self) -> &'static ParamInfo {
        &TABLE[self as usize]
    }

    /// Returns the spec identifier, for example `"specular_ior"`.
    #[must_use]
    pub fn identifier(self) -> &'static str {
        self.info().identifier
    }

    /// Looks a parameter up by its spec identifier.
    #[must_use]
    pub fn from_identifier(identifier: &str) -> Option<Self> {
        TABLE
            .iter()
            .find(|info| info.identifier == identifier)
            .map(|info| info.param)
    }
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.identifier())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_is_indexed_by_discriminant() {
        for (index, param) in Param::ALL.into_iter().enumerate() {
            assert_eq!(param as usize, index, "{param:?} is out of order in ALL");
            assert_eq!(
                param.info().param,
                param,
                "{param:?} has the wrong table row"
            );
        }
    }

    #[test]
    fn identifiers_are_unique_prefixed_and_round_trip() {
        for param in Param::ALL {
            let info = param.info();
            assert!(
                info.identifier
                    .strip_prefix(info.group.prefix())
                    .is_some_and(|rest| rest.starts_with('_')),
                "{} is not prefixed by its group",
                info.identifier
            );
            assert_eq!(Param::from_identifier(info.identifier), Some(param));
        }
        assert_eq!(Param::from_identifier("specular_anisotropy_rotation"), None);
    }

    #[test]
    fn groups_appear_in_specification_order() {
        let groups: alloc::vec::Vec<Group> = Param::ALL.iter().map(|p| p.info().group).collect();
        let mut expected = groups.clone();
        expected.sort();
        assert_eq!(groups, expected, "parameters must be grouped in spec order");
        for group in Group::ALL {
            assert!(groups.contains(&group), "{group:?} has no parameters");
        }
    }

    #[test]
    fn ranges_match_kinds_and_contain_defaults() {
        for param in Param::ALL {
            let info = param.info();
            match (info.kind, info.range, info.default) {
                (Kind::Float, Some(range), ParamDefault::Float(v)) => {
                    assert!(range.contains(v), "{param} default {v} outside {range}");
                }
                (Kind::Color3, Some(range), ParamDefault::Color3(c)) => {
                    assert!(c.iter().all(|&v| range.contains(v)), "{param} default");
                }
                (Kind::Boolean, None, ParamDefault::Boolean(_))
                | (
                    Kind::Vector3,
                    None,
                    ParamDefault::UnperturbedNormal | ParamDefault::UnperturbedTangent,
                ) => {}
                other => panic!("{param} has inconsistent metadata: {other:?}"),
            }
            if let (Some(norm), ParamDefault::Float(v)) = (info.norm, info.default) {
                assert!(norm.contains(v), "{param} default {v} outside norm {norm}");
            }
        }
    }

    #[test]
    fn colors_are_the_color_flagged_color3_parameters() {
        let flagged: alloc::vec::Vec<Param> =
            Param::ALL.into_iter().filter(|p| p.info().color).collect();
        assert_eq!(flagged, Param::COLORS);
        for param in Param::ALL {
            let info = param.info();
            let expected = info.kind == Kind::Color3 && param != Param::SubsurfaceRadiusScale;
            assert_eq!(info.color, expected, "{param}");
        }
    }

    #[test]
    fn ranges_follow_their_bounds() {
        assert!(POSITIVE.contains(f32::MIN_POSITIVE));
        assert!(!POSITIVE.contains(0.0));
        assert!(NON_NEGATIVE.contains(0.0));
        assert!(!NON_NEGATIVE.contains(f32::INFINITY));
        assert!(!NON_NEGATIVE.contains(f32::NEG_INFINITY));
        assert!(NON_NEGATIVE.contains(f32::MAX));
        assert!(!NON_NEGATIVE.contains(-0.1));
        assert!(!UNIT.contains(f32::NAN));
        assert!(!NON_NEGATIVE.contains(f32::NAN));
        assert_eq!(alloc::format!("{POSITIVE}"), "(0, inf)");
        assert_eq!(alloc::format!("{UNIT}"), "[0, 1]");
    }
}
