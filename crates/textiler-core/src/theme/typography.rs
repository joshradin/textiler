use std::collections::HashMap;
/// Typography provides
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Deserializer};
use yew::html::IntoPropValue;

use crate::style::Size;
use crate::theme::sx::SxValue;
use crate::Sx;

/// The level for typography
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypographyLevel {
    H1,
    H2,
    H3,
    H4,
    Title { size: Size },
    Body { size: Size },
    Custom(String),
    Star,
}

impl<'de> Deserialize<'de> for TypographyLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let as_string = <&'de str as Deserialize<'de>>::deserialize(deserializer)?;
        let parsed = TypographyLevel::from(as_string);
        Ok(parsed)
    }
}

impl Display for TypographyLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TypographyLevel::H1 => {
                write!(f, "h1")
            }
            TypographyLevel::H2 => {
                write!(f, "h2")
            }
            TypographyLevel::H3 => {
                write!(f, "h3")
            }
            TypographyLevel::H4 => {
                write!(f, "h4")
            }
            TypographyLevel::Title { size } => {
                write!(f, "title-{size}")
            }
            TypographyLevel::Body { size } => {
                write!(f, "body-{size}")
            }
            TypographyLevel::Custom(s) => {
                write!(f, "{s}")
            }
            TypographyLevel::Star => {
                write!(f, "*")
            }
        }
    }
}

impl IntoPropValue<TypographyLevel> for &str {
    fn into_prop_value(self) -> TypographyLevel {
        TypographyLevel::from(self)
    }
}

impl From<&str> for TypographyLevel {
    fn from(value: &str) -> Self {
        match value {
            "*" => TypographyLevel::Star,
            "h1" => TypographyLevel::H1,
            "h2" => TypographyLevel::H2,
            "h3" => TypographyLevel::H3,
            "h4" => TypographyLevel::H4,
            title if title.starts_with("title-") => {
                let size = title.strip_prefix("title-").unwrap();
                let size = IntoPropValue::<Size>::into_prop_value(size);
                TypographyLevel::Title { size }
            }
            body if body.starts_with("body-") => {
                let size = body.strip_prefix("body-").unwrap();
                let size = IntoPropValue::<Size>::into_prop_value(size);
                TypographyLevel::Body { size }
            }
            other => TypographyLevel::Custom(other.to_string()),
        }
    }
}

impl Default for TypographyLevel {
    fn default() -> Self {
        TypographyLevel::Body { size: Size::Md }
    }
}

/// Provides the scale details for typography, giving weights, sizes, and margins for each level
#[derive(Debug, Default, Clone, PartialEq)]
pub struct TypographyScale {
    /// The scale details for given levels
    levels: HashMap<TypographyLevel, LevelScale>,
}

impl TypographyScale {
    /// Creates a new scale from
    pub fn new<I: IntoIterator<Item = (TypographyLevel, LevelScale)>>(levels: I) -> Self {
        Self {
            levels: levels.into_iter().collect(),
        }
    }

    /// Shortcut for getting an [`Sx`](Sx) instance for a given, and also merges it with the "*" sx level
    pub fn at(&self, level: &TypographyLevel) -> Option<Sx> {
        self.scale(level)
            .map(LevelScale::sx)
            .map(|sx| match self.scale(&TypographyLevel::Star) {
                None => sx,
                Some(star_sx) => {
                    let star_sx = star_sx.sx();
                    sx.merge(star_sx)
                }
            })
    }

    /// Gets the scale at the given level
    pub fn scale(&self, level: &TypographyLevel) -> Option<&LevelScale> {
        self.levels.get(level)
    }

    /// Gets a mutable reference to the scale at the given level
    pub fn scale_mut(&mut self, level: &TypographyLevel) -> Option<&mut LevelScale> {
        self.levels.get_mut(level)
    }

    /// Insert a scale for the given level
    pub fn insert(&mut self, level: TypographyLevel, level_sx: LevelScale) {
        let _ = self.levels.insert(level, level_sx);
    }
}

impl<'a> IntoIterator for &'a TypographyScale {
    type Item = (&'a TypographyLevel, &'a LevelScale);
    type IntoIter = <&'a HashMap<TypographyLevel, LevelScale> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.levels.iter()
    }
}

/// Details for a specific level within the [`TypographyScale`](TypographyScale)
#[derive(Debug, Clone, PartialEq)]
pub struct LevelScale {
    sx: Sx,
}

impl LevelScale {
    /// Creates a new level scale from an sx instance
    pub fn new(sx: Sx) -> Self {
        Self { sx }
    }

    /// Creates an [`Sx`](Sx) instance for this level scale
    pub fn sx(&self) -> Sx {
        self.sx.clone()
    }
}

impl From<Sx> for LevelScale {
    fn from(value: Sx) -> Self {
        Self::new(value)
    }
}
