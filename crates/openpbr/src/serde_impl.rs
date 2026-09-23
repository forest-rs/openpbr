// Copyright 2026 the openpbr crate Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Serde support: identifiers for [`Param`], `[r, g, b]` arrays for colors.

use core::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Param;

impl Serialize for Param {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.identifier())
    }
}

impl<'de> Deserialize<'de> for Param {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ParamVisitor;

        impl Visitor<'_> for ParamVisitor {
            type Value = Param;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an OpenPBR parameter identifier")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Param, E> {
                Param::from_identifier(value)
                    .ok_or_else(|| E::invalid_value(de::Unexpected::Str(value), &self))
            }
        }

        deserializer.deserialize_str(ParamVisitor)
    }
}

/// Serializes an [`OpaqueColor`](::color::OpaqueColor) as its `[r, g, b]`
/// components; the color space is carried by the containing type.
pub(crate) mod color {
    use color::{ColorSpace, OpaqueColor};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub(crate) fn serialize<S: Serializer, CS: ColorSpace>(
        color: &OpaqueColor<CS>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        color.components.serialize(serializer)
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>, CS: ColorSpace>(
        deserializer: D,
    ) -> Result<OpaqueColor<CS>, D::Error> {
        <[f32; 3]>::deserialize(deserializer).map(OpaqueColor::new)
    }
}

#[cfg(test)]
mod tests {
    use alloc::format;
    use alloc::vec::Vec;

    use color::{AcesCg, OpaqueColor};
    use serde_test::{Token, assert_de_tokens, assert_de_tokens_error, assert_tokens};

    use crate::{Param, Parameters, Value};

    #[test]
    fn param_is_its_identifier() {
        assert_tokens(&Param::SpecularIor, &[Token::Str("specular_ior")]);
        assert_tokens(
            &Param::GeometryCoatTangent,
            &[Token::Str("geometry_coat_tangent")],
        );
        assert_de_tokens_error::<Param>(
            &[Token::Str("specular_haze")],
            "invalid value: string \"specular_haze\", expected an OpenPBR parameter identifier",
        );
    }

    /// The parameters with a constant field, in specification order.
    fn constant_params() -> Vec<Param> {
        Param::ALL
            .into_iter()
            .filter(|p| Parameters::<AcesCg>::DEFAULT.get(*p).is_some())
            .collect()
    }

    fn rgb(tokens: &mut Vec<Token>, c: [f32; 3]) {
        tokens.push(Token::Tuple { len: 3 });
        tokens.extend(c.map(Token::F32));
        tokens.push(Token::TupleEnd);
    }

    #[test]
    fn parameters_use_identifiers_and_rgb_arrays() {
        let params = Parameters::<AcesCg> {
            base_color: OpaqueColor::new([0.4, 0.02, 0.01]),
            coat_weight: 1.0,
            geometry_thin_walled: true,
            ..Parameters::DEFAULT
        };
        let fields = constant_params();
        let mut tokens = Vec::new();
        tokens.push(Token::Struct {
            name: "Parameters",
            len: fields.len(),
        });
        for param in fields {
            tokens.push(Token::Str(param.identifier()));
            match params.get(param).unwrap() {
                Value::Float(v) => tokens.push(Token::F32(v)),
                Value::Boolean(b) => tokens.push(Token::Bool(b)),
                Value::Color(c) => rgb(&mut tokens, c.components),
                Value::Channels(c) => rgb(&mut tokens, c),
            }
        }
        tokens.push(Token::StructEnd);
        assert_tokens(&params, &tokens);
    }

    #[test]
    fn missing_fields_take_their_defaults() {
        let expected = Parameters::<AcesCg> {
            specular_color: OpaqueColor::new([0.9, 0.7, 0.3]),
            emission_luminance: 500.0,
            ..Parameters::DEFAULT
        };
        let mut tokens = alloc::vec![
            Token::Struct {
                name: "Parameters",
                len: 2,
            },
            Token::Str("emission_luminance"),
            Token::F32(500.0),
            Token::Str("specular_color"),
        ];
        rgb(&mut tokens, [0.9, 0.7, 0.3]);
        tokens.push(Token::StructEnd);
        assert_de_tokens(&expected, &tokens);
        assert_de_tokens(
            &Parameters::<AcesCg>::DEFAULT,
            &[
                Token::Struct {
                    name: "Parameters",
                    len: 0,
                },
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn unknown_fields_are_rejected() {
        let known: Vec<_> = constant_params()
            .into_iter()
            .map(|p| format!("`{p}`"))
            .collect();
        assert_de_tokens_error::<Parameters>(
            &[
                Token::Struct {
                    name: "Parameters",
                    len: 1,
                },
                Token::Str("emission_weight"),
                Token::F32(1.0),
            ],
            &format!(
                "unknown field `emission_weight`, expected one of {}",
                known.join(", ")
            ),
        );
    }
}
