//! Controls the general style and approach of components

use serde::{Deserialize, Serialize};
use std::env::var;
use std::fmt::{Display, Formatter};
use strum::{AsRefStr, EnumIter, IntoEnumIterator};
use yew::html::{ImplicitClone, IntoPropValue};

/// The variant of this component to use
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, AsRefStr, EnumIter)]
pub enum Variant {
    /// Plain theme
    #[default]
    Plain,
    /// Outlined theme
    Outlined,
    /// A softer theme
    Soft,
    /// A solid theme
    Solid,
}

impl ImplicitClone for Variant {}

impl Display for Variant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref().to_lowercase())
    }
}

/// The main color scheme of this component to use
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, AsRefStr, EnumIter)]
pub enum Color {
    #[default]
    Neutral,
    Primary,
    Success,
    Fatal,
    Warn,
}

impl ImplicitClone for Color {}
impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref().to_lowercase())
    }
}

/// General size descriptions
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, AsRefStr, EnumIter, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum Size {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl ImplicitClone for Size {}

impl IntoPropValue<Size> for &str {
    fn into_prop_value(self) -> Size {
        Size::try_from(self).unwrap_or_else(|_| panic!("{self:?} is not a known color"))
    }
}

impl TryFrom<&str> for Size {
    type Error = UnknownSize;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        for size in Size::iter() {
            if size.as_ref().to_lowercase() == value {
                return Ok(size);
            }
        }
        Err(UnknownSize(value.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("No known size {0:?}")]
pub struct UnknownSize(pub String);

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref().to_lowercase())
    }
}
