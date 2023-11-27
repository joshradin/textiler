//! Used for theming

use std::collections::HashMap;
use std::ops::Deref;

use once_cell::sync::Lazy;
use yew::Properties;

use crate::theme::breakpoint::Breakpoints;
pub use color::Color;
use regex::Regex;

use crate::theme::palette::Palette;
use crate::theme::typography::{TypographyLevel, TypographyScale};
use crate::utils::to_property;

pub mod color;

pub mod baseline;
pub mod breakpoint;
pub mod gradient;
pub mod palette;
pub mod parsing;
pub mod sx;
pub mod theme_mode;
pub mod typography;

#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    pub prefix: String,
    breakpoints: Breakpoints,
    palettes: HashMap<String, Palette>,
    typography: TypographyScale,
}

static DEFAULT_THEME: Lazy<Theme> = Lazy::new(|| {
    parsing::from_str(include_str!("./theme/theme.json")).expect("could not read default theme")
});

impl Default for Theme {
    fn default() -> Self {
        DEFAULT_THEME.clone()
    }
}

impl Theme {
    pub fn new() -> Self {
        Self::with_prefix("happy")
    }

    pub fn with_prefix(prefix: impl AsRef<str>) -> Self {
        Self {
            prefix: prefix.as_ref().to_string(),
            breakpoints: Default::default(),
            palettes: Default::default(),
            typography: Default::default(),
        }
    }

    /// Get a palette by name
    pub fn get_palette(&self, name: &str) -> Option<&Palette> {
        self.palettes.get(name)
    }

    /// Get a palette by name
    pub fn palette_mut(&mut self, name: &str) -> Option<&mut Palette> {
        self.palettes.get_mut(name)
    }

    /// Insert a palette into the theme
    pub fn insert_palette(&mut self, name: impl AsRef<str>, palette: Palette) {
        let _ = self.palettes.insert(name.as_ref().to_string(), palette);
    }

    /// Creates a new palette if not yet present, and returns a mutable reference to it.
    pub fn palette(&mut self, name: impl AsRef<str>) -> &mut Palette {
        self.palettes.entry(name.as_ref().to_string()).or_default()
    }

    pub fn palette_var(&self, palette: &str, selector: &str) -> String {
        to_property(format!("--{}-palette-{palette}-{selector}", self.prefix))
    }

    pub fn class_var(&self, class: &str, var_name: &str) -> String {
        to_property(format!("--{}-{class}-{var_name}", self.prefix))
    }

    /// Gets all palettes
    pub fn palettes(&self) -> impl Iterator<Item = (&str, &Palette)> {
        self.palettes.iter().map(|(key, value)| (&**key, value))
    }

    /// Gets a reference to the breakpoints object
    pub fn breakpoints(&self) -> &Breakpoints {
        &self.breakpoints
    }

    /// Gets a mutable reference to the breakpoints object
    pub fn breakpoints_mut(&mut self) -> &mut Breakpoints {
        &mut self.breakpoints
    }

    /// Gets the typography scale
    pub fn typography(&self) -> &TypographyScale {
        &self.typography
    }

    /// Gets the typography scale
    pub fn typography_mut(&mut self) -> &mut TypographyScale {
        &mut self.typography
    }

    pub fn system_class(&self) -> String {
        format!(".{}-system", self.prefix)
    }
}

pub static PALETTE_SELECTOR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"^(?<palette>[a-zA-Z_]\w*)\.(?<selector>\w+)$"#)
        .expect("could not create palette selector")
});
