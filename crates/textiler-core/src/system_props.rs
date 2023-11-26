//! System properties are exclusive, and provide translations to "real" css properties

use std::borrow::Cow;
use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::theme::breakpoint::Breakpoints;

/// Contains standard system properties and their translations, should only exist as a
/// singleton instance [`SYSTEM_PROPERTIES`](SYSTEM_PROPERTIES).
#[derive(Debug, Clone)]
pub struct SystemProperties {
    mappings: HashMap<String, Vec<String>>,
}

static SYSTEM_PROPS_MAP: &[(&str, &[&str])] = &[
    ("p", &["padding"]),
    ("pl", &["paddingLeft"]),
    ("pr", &["paddingRight"]),
    ("pt", &["paddingTop"]),
    ("pb", &["paddingBottom"]),
    ("pX", &["paddingLeft", "paddingRight"]),
    ("pY", &["paddingTop", "paddingBottom"]),
    ("bgcolor", &["backgroundColor"]),
    ("bg", &["background"]),
    ("marginX", &["margin-left", "margin-right"]),
    ("marginY", &["margin-top", "margin-bottom"]),
];

impl SystemProperties {
    /// Create a new system properties instance
    fn new() -> Self {

        Self {
            mappings:
            SYSTEM_PROPS_MAP
            .iter()
            .map(|(k, v): &(&str, &[&str])| {
                (
                    k.to_string(),
                    v.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
                )
            })
            .collect(),
        }
    }
}

impl CssPropertyTranslator for SystemProperties {
    fn translate<'a>(&self, query: &'a str) -> Vec<Cow<'a, str>> {
        self.mappings
            .get(query)
            .map(|result| {
                result
                    .iter()
                    .map(|s| Cow::<str>::Owned(s.clone()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_else(move || vec![Cow::Borrowed(query)])
    }
}

pub static SYSTEM_PROPERTIES: Lazy<SystemProperties> = Lazy::new(|| SystemProperties::new());

/// attempts to translate a given css query into a modified one
#[derive(Debug)]
pub struct TranslationUnit {
    props: SystemProperties,
    bps: Breakpoints,
}

impl TranslationUnit {
    pub fn new(bps: &Breakpoints) -> Self {
        Self {
            props: SYSTEM_PROPERTIES.clone(),
            bps: bps.clone(),
        }
    }
}

impl CssPropertyTranslator for TranslationUnit {
    fn translate<'a>(&self, query: &'a str) -> Vec<Cow<'a, str>> {
        if self.props.mappings.contains_key(query) {
            self.props.translate(query)
        } else if let Some(breakpoint) = self.bps.get(query) {
            vec![Cow::Owned(format!(
                "@media (min-width: {}px)",
                breakpoint.width()
            ))]
        } else {
            vec![Cow::Borrowed(query)]
        }
    }
}

/// Translate a given property into something else
pub trait CssPropertyTranslator {
    /// Translates
    fn translate<'a>(&self, query: &'a str) -> Vec<Cow<'a, str>>;
}
