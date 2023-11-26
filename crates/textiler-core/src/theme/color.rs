use nom::branch::alt;
use nom::bytes::complete::{tag, take_while_m_n};
use nom::combinator::{map_res, opt};
use nom::error::Error;
use nom::sequence::tuple;
use nom::{ErrorConvert, Finish, IResult};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

/// Html compatible
#[derive(Debug, Clone, PartialEq, Deserialize, Hash, Eq, Serialize)]
#[serde(untagged)]
pub enum Color {
    /// CSS literal value
    CSSLiteral(String),
    /// A hex color
    Hex(u32),
    /// Rgb color
    Rgb { r: u8, g: u8, b: u8 },
    /// Rgba color
    Rgba { r: u8, g: u8, b: u8, a: u8 },
    /// hsl color
    Hsl { h: u16, s: u8, l: u8 },
    /// hsla color
    Hsla { h: u16, s: u8, l: u8, a: u8 },
    Var {
        var: String,
        fallback: Option<Box<Color>>,
    },
}

/// HSL are constrained to `[0, 1]`
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> [u8; 3] {
    let mut r: f32 = 0.;
    let mut g: f32 = 0.;
    let mut b: f32 = 0.;

    if s == 0.0 {
        r = l;
        g = l;
        b = l;
    } else {
        let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };
        let p = 2. * l - q;
        r = hue_to_rgb(p, q, h + (1. / 3.));
        g = hue_to_rgb(p, q, h);
        b = hue_to_rgb(p, q, h - (1. / 3.));
    }
    [
        (r * 255.0).round() as u8,
        (g * 255.0).round() as u8,
        (b * 255.0).round() as u8,
    ]
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t = t + 1.;
    }
    if t > 1.0 {
        t = t - 1.;
    }
    if t < 1.0 / 6.0 {
        p + (q - p) * 6. * t
    } else if t < 0.5 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2. / 3. - t) * 6.
    } else {
        p
    }
}

pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> [f32; 3] {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;

    let vmax = r.max(g).max(b);
    let vmin = r.min(g).min(b);

    let h: f64;
    let s: f64;
    let l: f64 = (vmax + vmin) / 2.0;
    if vmax == vmin {
        return [0., 0., l as f32];
    }

    let d = vmax - vmin;
    s = if l > 0.5 {
        d / (2.0 - vmax - vmin)
    } else {
        d / (vmax + vmin)
    };
    if vmax == r {
        h = (g - b) / d + (if g < b { 6.0 } else { 0.0 });
    } else if vmax == g {
        h = (b - r) / d + 2.0;
    } else {
        h = (r - g) / d + 4.0;
    }
    let h = h / 6.0;

    [h as f32, s as f32, l as f32]
}

#[derive(Debug)]
pub(super) enum SimpleColor {
    Rgba(u8, u8, u8, u8),
    Hsla(f32, f32, f32, f32),
}

impl Color {
    /// Creates a new color with a name
    pub fn named<S: AsRef<str>>(name: S) -> Self {
        Self::CSSLiteral(name.as_ref().to_string())
    }

    /// Creates a new color by hex
    pub fn hex_code(value: u32) -> Self {
        Self::Hex(value)
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb { r, g, b }
    }

    pub fn hsl(h: u16, s: u8, l: u8) -> Self {
        Self::Hsl { h, s, l }
    }

    /// Gets the color as an HSLA value, where each is a value \[0,1], if it
    /// can be converted
    pub fn to_hsla(&self) -> Result<[f32; 4], TransformColorError> {
        let simple = self.to_simple()?;

        match simple {
            SimpleColor::Rgba(r, g, b, a) => {
                let [h, s, l] = rgb_to_hsl(r, g, b);
                Ok([h, s, l, a as f32 / 255.0])
            }
            SimpleColor::Hsla(h, s, l, a) => Ok([h, s, l, a]),
        }
    }

    /// Gets the color as an HSLA value, where each is a value \[0,1], if it
    /// can be converted
    pub fn to_hsla_color(&self) -> Result<Color, TransformColorError> {
        let [h, s, l, a] = self.to_hsla()?;
        Ok(Self::Hsla {
            h: (h * 360.0).round() as u16,
            s: (s * 100.0).round() as u8,
            l: (l * 100.0).round() as u8,
            a: (a * 100.0).round() as u8,
        })
    }

    /// Gets the color as an RGBA value, where each is a value \[0,256], if it
    /// can be converted
    pub fn to_rgba(&self) -> Result<[u8; 4], TransformColorError> {
        let simple = self.to_simple()?;

        match simple {
            SimpleColor::Rgba(r, g, b, a) => Ok([r, g, b, a]),
            SimpleColor::Hsla(h, s, l, a) => {
                let [r, g, b] = hsl_to_rgb(h, s, l);
                Ok([r, g, b, (a * 255.0) as u8])
            }
        }
    }

    /// Gets the color as an HSLA value, where each is a value \[0,1], if it
    /// can be converted
    pub fn to_rgba_color(&self) -> Result<Color, TransformColorError> {
        let [r, g, b, a] = self.to_rgba()?;
        Ok(Self::Rgba { r, g, b, a })
    }

    pub(super) fn to_simple(&self) -> Result<SimpleColor, TransformColorError> {
        let mut color: Cow<Color> = Cow::Borrowed(self);
        if let Color::CSSLiteral(literal) = self {
            *color.to_mut() = Color::from_str(literal)?;
        }

        let simple = match &*color {
            &Color::Hex(hex) => {
                let [r, g, b] = u32_to_rgb(hex);
                SimpleColor::Rgba(r, g, b, 0)
            }
            &Color::Rgb { r, g, b } => SimpleColor::Rgba(r, g, b, 255),
            &Color::Rgba { r, g, b, a } => SimpleColor::Rgba(r, g, b, a),
            &Color::Hsl { h, s, l } => {
                SimpleColor::Hsla(h as f32 / 360.0, s as f32 / 100.0, l as f32 / 100.0, 1.0)
            }
            &Color::Hsla { h, s, l, a } => SimpleColor::Hsla(
                h as f32 / 360.0,
                s as f32 / 100.0,
                l as f32 / 100.0,
                a as f32 / 100.0,
            ),
            color => return Err(TransformColorError::NonEligibleColor(color.clone())),
        };
        Ok(simple)
    }
}

/// An error occurred trying to convert HSLA
#[derive(Debug, Error)]
pub enum TransformColorError {
    #[error("{0:?} can not be converted because it's non-eligible")]
    NonEligibleColor(Color),
    #[error(transparent)]
    ParseError(#[from] ParseColorError),
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_color(s).finish().map(|ok| ok.1).map_err(|e| {
            let Error { input, code } = e;

            Error {
                input: input.to_string(),
                code,
            }
            .into()
        })
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::CSSLiteral(n) => {
                write!(f, "{n}")
            }
            Color::Hex(hex) => {
                write!(f, "#{hex:06X}")
            }
            Color::Rgb { r, g, b } | Color::Rgba { r, g, b, a: 255 } => {
                write!(f, "#{r:02X}{g:02X}{b:02X}")
            }
            Color::Rgba { r, g, b, a } => {
                write!(f, "#{r:02X}{g:02X}{b:02X}{a:02X}")
            }
            Color::Hsl { h, s, l } | Color::Hsla { h, s, l, a: 100 } => {
                write!(f, "hsla({h}, {s}%, {l}%)")
            }
            Color::Hsla { h, s, l, a } => {
                write!(f, "hsla({h}, {s}%, {l}%, {:1.2})", *a as f32 / 100.0)
            }
            Color::Var {
                var: name,
                fallback,
            } => match fallback {
                None => {
                    write!(f, "var({name})")
                }
                Some(fallback) => {
                    write!(f, "var({name}, {fallback})")
                }
            },
        }
    }
}

pub fn parse_color(color: &str) -> IResult<&str, Color> {
    alt((parse_hex_color,))(color)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn parse_hex_color(color: &str) -> IResult<&str, Color> {
    let (rest, _) = tag("#")(color)?;
    let (rest, (r, g, b, a)) =
        tuple((hex_primary, hex_primary, hex_primary, opt(hex_primary)))(rest)?;

    Ok((
        rest,
        match a {
            None => Color::Rgb { r, g, b },
            Some(a) => Color::Rgba { r, g, b, a },
        },
    ))
}

fn u32_to_rgb(value: u32) -> [u8; 3] {
    let r = (value >> 16) & 0xFF;
    let g = (value >> 8) & 0xFF;
    let b = value & 0xFF;
    [r as u8, g as u8, b as u8]
}

/// An error occurred while trying to parse a color
#[derive(Debug, thiserror::Error)]
pub enum ParseColorError {
    #[error(transparent)]
    NomError(#[from] nom::error::Error<String>),
}

#[cfg(test)]
mod tests {
    use crate::theme::Color;
    use nom::Finish;

    use crate::theme::color::{hsl_to_rgb, parse_color, rgb_to_hsl};
    use crate::theme::gradient::Gradient;
    use crate::utils::bounded_float::BoundedFloat;

    #[test]
    fn parse_hex_color() {
        let hex = "#0f125f";
        let (_, parsed) = parse_color(hex).finish().unwrap();
        println!("{parsed}")
    }

    #[test]
    fn parse_hex_alpha_color() {
        let hex = "#01f1257f";
        let (_, parsed) = parse_color(hex).finish().unwrap();
        println!("{parsed}")
    }

    #[test]
    fn hsl_to_rgb_correctness() {
        let (h, s, l) = (126.0 / 360., 0.46, 0.63);
        let [r, g, b] = hsl_to_rgb(h, s, l);
        assert_eq!(r, 117);
        assert_eq!(g, 204);
        assert_eq!(b, 126);
    }

    #[test]
    fn rgb_to_hsl_correctness() {
        const CORRECTNESS: f32 = 0.001;
        let (r, g, b) = (11, 13, 14);
        let [h, s, l] = rgb_to_hsl(r, g, b);

        assert!(
            h - 200. / 360. < CORRECTNESS,
            "error on h ({h}) is >= {CORRECTNESS}. Expected is ~200 deg"
        );
        assert!(
            s - 0.12 < CORRECTNESS,
            "error on s ({s}) is >= {CORRECTNESS}. Expected is ~0.12"
        );
        assert!(
            l - 0.05 < CORRECTNESS,
            "error on l ({l}) is >= {CORRECTNESS}. Expected is ~0.05"
        );
    }
}
