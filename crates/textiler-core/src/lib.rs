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

pub use crate::theme::hooks::use_sx;
pub use theme::context::*;
pub use theme::sx::Sx;

pub mod system_props;
pub mod utils;

pub mod style;

/// The prelude
pub mod prelude {
    pub use crate::style::*;
    pub use crate::sx;
    pub use crate::theme::{sx::Sx, Theme};
}
