use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};

use crate::theme::sx::sx_value_parsing::{parse_sx_value, ParseSxValueError};
use crate::theme::{Color, Theme, PALETTE_SELECTOR_REGEX};
use crate::Sx;

/// An sx value
#[derive(Debug, PartialEq, Clone)]
pub enum SxValue {
    Integer(BigDecimal),
    Float(BigDecimal),
    Percent(BigDecimal),
    FloatDimension {
        value: BigDecimal,
        unit: String,
    },
    Dimension {
        value: BigDecimal,
        unit: String,
    },
    CssLiteral(String),
    String(String),
    Color(Color),
    ThemeToken {
        palette: String,
        selector: String,
    },
    ClassVar {
        class: String,
        var: String,
        fallback: Option<Box<SxValue>>,
    },
    Callback(FnSxValue),
    Nested(Sx),
}

impl SxValue {
    pub fn var(class: &str, var: &str, fallback: impl Into<Option<SxValue>>) -> Self {
        Self::ClassVar {
            class: class.to_string(),
            var: var.to_string(),
            fallback: fallback.into().map(|fallback| Box::new(fallback)),
        }
    }

    pub fn to_css(self) -> Option<String> {
        Some(match self {
            SxValue::Integer(i) => {
                format!("{i}")
            }
            SxValue::Float(f) => {
                format!("{f}")
            }
            SxValue::Percent(p) => {
                format!("{}%", (p * BigDecimal::from(100_u8)))
            }
            SxValue::FloatDimension { value, unit } => {
                format!("{value}{unit}")
            }
            SxValue::Dimension { value, unit } => {
                format!("{value}{unit}")
            }
            SxValue::CssLiteral(lit) => {
                format!("{lit}")
            }
            SxValue::String(s) => {
                format!("\"{s}\"")
            }
            SxValue::Color(c) => c.to_string(),
            _other => return None,
        })
    }
}

impl From<i32> for SxValue {
    fn from(value: i32) -> Self {
        Self::Integer(value.into())
    }
}

impl From<f32> for SxValue {
    fn from(value: f32) -> Self {
        Self::Float(BigDecimal::from_f32(value).expect("not representable by big decimal"))
    }
}

impl From<&str> for SxValue {
    fn from(quoted_str: &str) -> Self {
        if let Some(matched) = PALETTE_SELECTOR_REGEX.captures(quoted_str) {
            let palette = matched["palette"].to_string();
            let selector = matched["selector"].to_string();

            SxValue::ThemeToken { palette, selector }
        } else if quoted_str.contains(char::is_whitespace) {
            SxValue::CssLiteral(quoted_str.to_string())
        } else {
            quoted_str.parse().unwrap()
        }
    }
}

impl From<Sx> for SxValue {
    fn from(value: Sx) -> Self {
        Self::Nested(value)
    }
}

impl FromStr for SxValue {
    type Err = ParseSxValueError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_sx_value(s)
    }
}

/// An sx value derived from a function
#[derive(Clone)]
pub struct FnSxValue {
    id: u64,
    callback: Arc<Mutex<dyn Fn(&Theme) -> SxValue + Send>>,
}

impl PartialEq for FnSxValue {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl FnSxValue {
    pub fn new<R, F: Fn(&Theme) -> R + Send + 'static>(callback: F) -> Self
    where
        R: Into<SxValue>,
    {
        Self {
            id: rand::random(),
            callback: Arc::new(Mutex::new(move |theme: &Theme| (callback)(theme).into())),
        }
    }

    pub fn apply(&self, theme: &Theme) -> SxValue {
        let callback = self.callback.lock().expect("callback is poisoned");
        (callback)(theme)
    }
}

impl Debug for FnSxValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(&Theme) => SxValue")
    }
}
