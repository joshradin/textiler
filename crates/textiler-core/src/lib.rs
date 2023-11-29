//! # Happiness
//!
//! A MUI inspired yew library

#[macro_use]
extern crate log;
mod components;
mod error;
pub use components::*;
pub mod theme;
pub use error::Error;

pub use theme::sx::Sx;

pub mod system_props;
pub mod utils;

pub mod style;
pub mod context;
pub mod hooks;
pub mod style_manager;

/// The prelude
pub mod prelude {
    pub use crate::style::*;
    pub use crate::sx;
    pub use crate::theme::{sx::Sx, Theme, theme_mode::ThemeMode};
    pub use crate::context::{CssBaseline, StyleManagerContext, ThemeContext, ThemeModeContext, ThemeProvider};
    pub use crate::hooks::*;
    pub use crate::surfaces::Sheet;
    pub use crate::system::StylingBox;
    pub use crate::typography::Typography;
    pub use crate::style::*;

}
