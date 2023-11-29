use std::borrow::Cow;

use stylist::manager::StyleManager;
use stylist::Style;
use yew::Classes;

use crate::style_manager::{Css, StyleManagerBuilder};

use super::StyleManager as StyleManagerTrait;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Manager(StyleManager);

impl StyleManagerTrait for Manager {
    type Builder = Builder;
    type Error = stylist::Error;

    fn builder() -> Self::Builder {
        Builder(stylist::manager::StyleManagerBuilder::new())
    }

    fn mount(&self, css: &Css) -> Result<SxRef, Self::Error> {
        Style::new_with_manager(&**css, &self.0)
            .map(|style| SxRef::new(style))
    }
}

pub struct Builder(stylist::manager::StyleManagerBuilder);

impl StyleManagerBuilder for Builder {
    type Built = Manager;
    type Error = stylist::Error;

    fn prefix<S: AsRef<str>>(self, prefix: S) -> Self {
        Self(self.0.prefix(Cow::Owned(prefix.as_ref().to_string())))
    }

    fn build(self) -> Result<Self::Built, Self::Error> {
        Ok(Manager(self.0.build()?))
    }
}

/// A style ref can be used as a css class
#[derive(Debug, Clone)]
pub struct SxRef {
    style: Style,
}

impl SxRef {
    pub(crate) fn new(style: Style) -> Self {
        Self { style }
    }
}

impl From<SxRef> for Classes {
    fn from(value: SxRef) -> Self {
        Classes::from(value.style)
    }
}

