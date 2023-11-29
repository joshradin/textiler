//! Provides the style managing trait.
//!
//! Style managers are a bridge between [`Sx`](Sx) instances and the actual CSS that is made available
//! on web pages. Consider this the "backend" of the textiler framework.
//!
//!


use cfg_if::cfg_if;
use static_assertions::assert_impl_all;
use yew::Classes;
use crate::Sx;


#[cfg(feature = "stylist")]
pub mod stylist;
#[cfg(feature = "stylist")]
pub use stylist as platform;


cfg_if!(
    if #[cfg(not(any(feature="stylist")))] {
        compile_error!("most select a backend. Backends are [stylist]");
    }
);

pub use platform::{Manager as StyleManagerBackend, SxRef};
assert_impl_all!(SxRef: Into<Classes>);

pub type Css = String;

/// A style manager mounts [`Sx`](Sx) instances onto web pages for consumption of styled components
pub trait StyleManager {
    type Builder;
    type Error;

    /// Creates a new style manager
    fn builder() -> Self::Builder;

    /// Mounts the given [`Css`](Css) onto the web page
    fn mount(&self, css: &Css) -> Result<SxRef, Self::Error>;
}

/// Builds a style manager
pub trait StyleManagerBuilder {
    type Built: StyleManager;
    type Error;

    fn prefix<S : AsRef<str>>(self, prefix: S) -> Self;
    fn build(self) -> Result<Self::Built, Self::Error>;
}

