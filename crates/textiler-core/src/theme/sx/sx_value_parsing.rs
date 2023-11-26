use std::num::ParseIntError;
use std::ops::Deref;

use cssparser::{BasicParseError, BasicParseErrorKind, Parser, ParserInput, ToCss, Token};

use crate::theme::sx::sx_value::SxValue;
use crate::theme::Color;

type Result<T> = std::result::Result<T, ParseSxValueError>;

pub(super) fn parse_sx_value(input: &str) -> Result<SxValue> {
    let mut input = ParserInput::new(input);
    let mut parser = Parser::new(&mut input);

    let value = parse(&mut parser)?;
    Ok(match (value, parser.next().ok()) {
        (value, None) => value,
        (SxValue::CssLiteral(literal), Some(Token::Delim('.'))) => {
            let next = parser.next()?;
            let Token::Ident(ident) = next else {
                return Err(ParseSxValueError::UnexpectedToken(next.to_css_string()));
            };
            SxValue::ThemeToken {
                palette: literal,
                selector: ident.to_string(),
            }
        }
        (value, _) => value,
    })
}

fn parse(parser: &mut Parser) -> Result<SxValue> {
    let sx_value = match parser.next()? {
        Token::Ident(ident) => {
            if let Some((color, ..)) =
                cssparser::color::all_named_colors().find(|(color, ..)| *color == &**ident)
            {
                SxValue::Color(Color::CSSLiteral(color.to_string()))
            } else {
                SxValue::CssLiteral(ident.to_string())
            }
        }
        Token::Hash(hash) => SxValue::Color(Color::hex_code(u32::from_str_radix(&*hash, 16)?)),
        Token::QuotedString(quoted_str) => {
            let split = quoted_str.split(".").collect::<Vec<_>>();
            if split.len() == 2 {
                let palette = split[0].to_string();
                let selector = split[1].to_string();

                SxValue::ThemeToken { palette, selector }
            } else {
                SxValue::String(quoted_str.to_string())
            }
        }
        Token::Number {
            has_sign: _,
            value,
            int_value,
        } => {
            if let Some(int_value) = int_value {
                SxValue::Integer(*int_value)
            } else {
                SxValue::Float(*value)
            }
        }
        Token::Percentage {
            has_sign: _,
            unit_value,
            int_value,
        } => {
            if let &Some(int_value) = int_value {
                SxValue::Percent(int_value as f32 / 100.0)
            } else {
                SxValue::Percent(*unit_value)
            }
        }
        Token::Dimension {
            has_sign: _,
            value,
            int_value,
            unit,
        } => match int_value {
            None => SxValue::FloatDimension {
                value: *value,
                unit: unit.to_string(),
            },
            Some(value) => SxValue::Dimension {
                value: *value,
                unit: unit.to_string(),
            },
        },
        _tok => {
            return Err(ParseSxValueError::UnexpectedToken(_tok.to_css_string()));
        }
    };

    Ok(sx_value)
}

/// An error occurred while trying to parse this value
#[derive(Debug, thiserror::Error)]
pub enum ParseSxValueError {
    #[error("Unexpected token in input: {0:?}")]
    UnexpectedToken(String),
    #[error("An error occurred while trying to parse css")]
    CssParseError,
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
}

impl<'a> From<BasicParseError<'a>> for ParseSxValueError {
    fn from(value: BasicParseError<'a>) -> Self {
        match &value.kind {
            BasicParseErrorKind::UnexpectedToken(tok) => {
                ParseSxValueError::UnexpectedToken(tok.to_css_string())
            }
            _err => ParseSxValueError::CssParseError,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::theme::sx::sx_value::SxValue;
    use crate::theme::sx::sx_value_parsing::parse_sx_value;
    use crate::theme::Color;

    #[test]
    fn parse_theme_token() {
        let test = r#" "background.body" "#;

        let value = parse_sx_value(test).expect("could not parse");
        let SxValue::ThemeToken { palette, selector } = &value else {
            panic!("wrong sx value kind: {value:#?}");
        };

        assert_eq!(palette, "background");
        assert_eq!(selector, "body");
    }

    #[test]
    fn parse_colors() {
        let hex = "#0f0f0f";

        let value = parse_sx_value(hex).expect("could not parse");
        let SxValue::Color(color) = &value else {
            panic!("wrong sx value kind: {value:#?}");
        };
        assert!(matches!(color, crate::theme::Color::Hex(0x0f0f0f)));

        let red = "red";

        let value = parse_sx_value(hex).expect("could not parse");
        let SxValue::Color(color) = &value else {
            panic!("wrong sx value kind: {value:#?}");
        };
        let SxValue::Color(Color::CSSLiteral(color)) = value else {
            panic!("should be a color")
        };
        assert_eq!(color, "red");
    }

    #[test]
    fn parse_percent() {
        let percent = "15%";
        let SxValue::Percent(percent) = parse_sx_value(percent).expect("parse error") else {
            panic!("should parse percent");
        };

        assert_eq!(percent, 0.15);

        let percent = "15.3%";
        let SxValue::Percent(percent) = parse_sx_value(percent).expect("parse error") else {
            panic!("should parse percent");
        };

        assert_eq!(percent, 0.153);

        let percent = "-15.3%";
        let SxValue::Percent(percent) = parse_sx_value(percent).expect("parse error") else {
            panic!("should parse percent");
        };

        assert_eq!(percent, -0.153);
    }

    #[test]
    fn parse_dimension() {
        let width = "5px";

        let SxValue::Dimension { value, unit } = parse_sx_value(width).expect("parse error") else {
            panic!("should parse dimension");
        };

        assert_eq!(value, 5);
        assert_eq!(unit, "px");
    }
}
