//! Contains the definition of the `Sx` type and the `sx!` macro
//!
//!
use cssparser::ToCss;
use gloo::history::query::FromQuery;
use heck::{ToKebabCase, ToLowerCamelCase, ToTrainCase};
use indexmap::map::Entry;
use indexmap::IndexMap;
use serde::Deserialize;
use std::fmt::Debug;
use std::ops::Index;
use std::str::FromStr;
use stylist::ast::{Sheet, ToStyleStr};
use stylist::Style;
use yew::Classes;

pub use crate::theme::sx;
use crate::theme::sx::sx_to_css::sx_to_css;
use crate::theme::theme_mode::ThemeMode;
use crate::theme::Theme;

mod sx_to_css;
mod sx_value;
mod sx_value_parsing;
use crate::system_props::{CssPropertyTranslator, SYSTEM_PROPERTIES};
use crate::utils::to_property;
pub use sx_value::*;

/// Contains CSS definition with some customization
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Sx {
    props: IndexMap<String, SxValue>,
}

pub type Css = Sheet;

impl Sx {
    /// Sets a css property
    pub fn insert<K: AsRef<str>, V: Into<SxValue>>(&mut self, key: K, value: V) {
        let translated = SYSTEM_PROPERTIES.translate(key.as_ref());
        let value = value.into();
        for translated in translated {
            self.props.insert(to_property(translated), value.clone());
        }
    }

    /// Merges this Sx with another Sx. Uses the left's values for conflicting keys.
    pub fn merge(self, other: Self) -> Self {
        let mut sx = self;

        for (prop, value) in other.props {
            match sx.props.entry(prop) {
                Entry::Occupied(mut occ) => match occ.get_mut() {
                    SxValue::Nested(old_sx) => {
                        if let SxValue::Nested(sx) = value {
                            *old_sx = old_sx.clone().merge(sx);
                        }
                    }
                    _ => {}
                },
                Entry::Vacant(v) => {
                    v.insert(value);
                }
            }
        }

        sx
    }

    pub fn to_css(self, mode: &ThemeMode, theme: &Theme) -> Css {
        let css = sx_to_css(self, mode, theme, None).expect("invalid sx");
        Sheet::from_str(&css).unwrap()
    }

    /// Gets the properties set in this sx
    pub fn properties(&self) -> impl IntoIterator<Item = &str> {
        self.props.keys().map(|s| s.as_ref())
    }
}

impl Index<&str> for Sx {
    type Output = SxValue;

    fn index(&self, index: &str) -> &Self::Output {
        &self.props[index]
    }
}

impl From<SxRef> for Classes {
    fn from(value: SxRef) -> Self {
        Classes::from(value.style)
    }
}

/// Creates [`Sx`][Sx] instances
#[macro_export]
macro_rules! sx {
    (
        $($json:tt)*
    ) => {
        $crate::sx_internal!({ $($json)* })
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! sx_internal {

    // TT parser for objects

    // done
    (@object $object:ident () () ()) => {
    };

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$key:ident] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert((stringify!($key)).trim(), sx_internal!($value));
        sx_internal!(@object $object () ($($rest)*) ($($rest)*));
    };

    // Insert the current entry followed by trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $object.insert(($($key)+), $value);
        sx_internal!(@object $object () ($($rest)*) ($($rest)*));
    };



     // Next value is a map.
    (@object $object:ident ($($key:tt)+) (: {$($map:tt)*} $($rest:tt)*) $copy:tt) => {
       sx_internal!(@object $object [$($key)+] (sx_internal!({$($map)*})) $($rest)*);
    };

     // Next value is a callback
    (@object $object:ident ($($key:tt)+) (: |$theme:ident| $func:expr , $($rest:tt)*) $copy:tt) => {
       sx_internal!(@object $object [$($key)+] (sx_internal!(|$theme| $func)) $($rest)*);
    };

    // Next value is a callback with no rest
    (@object $object:ident ($($key:tt)+) (: |$theme:ident| $func:expr) $copy:tt) => {
       sx_internal!(@object $object [$($key)+] (sx_internal!(|$theme| $func)));
    };

    // Next value is an expression followed by comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        sx_internal!(@object $object [$($key)+] (sx_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@object $object:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        sx_internal!(@object $object [$($key)+] ( sx_internal!($value) ) );
    };

     // Insert the last entry without trailing comma.
    (@object $object:ident [$key:ident] ($value:expr)) => {
        let _ = $object.insert((stringify!($key)).trim(), sx_internal!($value));
    };

     // Insert the last entry without trailing comma.
    (@object $object:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $object.insert(($($key)+), sx_internal!($value));
    };


    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@object $object:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        sx_internal!(@object $object ($key) (: $($rest)*) (: $($rest)*));
    };

    // Refuse to absorb colon token into key expression.
    (@object $object:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        compile_error!("unexpected colon")
    };

    // Munch a token into the current key.
    (@object $object:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        sx_internal!(@object $object ($($key)* $tt) ($($rest)*) ($($rest)*));
    };


    // main implementation
    ({}) => {
        crate::theme::sx::Sx::default()
    };

    ({ $($tt:tt)+ }) => {
        {
            use $crate::theme::sx::*;
            use $crate::{sx, sx_internal};

            let mut sx: Sx = Sx::default();
            sx_internal!(@object sx () ($($tt)+) ($($tt)+));
            sx
        }
    };

    (|$theme:ident| $expr:expr) => {
        SxValue::Callback(FnSxValue::new(|$theme| $expr))
    };

    ($expr:expr) => {
        SxValue::try_from($expr).expect("could not create sxvalue")
    };


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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sx_with_macro() {
        let sx = sx! {
            width: "123.5%",
            p: "background.body",
        };
        assert_eq!(
            sx["p"],
            SxValue::ThemeToken {
                palette: "background".to_string(),
                selector: "body".to_string()
            }
        )
    }

    #[test]
    fn merge_sx() {
        let base = sx! {
            "bgcolor": "background.level1",
        };
        let merged = base.clone().merge(sx! {
            "bgcolor": SxValue::var("sheet", "background-color", None)
        });

        assert_eq!(
            &base["bgcolor"],
            &SxValue::ThemeToken {
                palette: "background".to_string(),
                selector: "level1".to_string(),
            }
        );
    }

    #[test]
    fn to_css() {
        let theme = Theme::default();

        let sx = sx! {
            padding: "15px",
            color: "background.body"
        };

        let style = sx.to_css(&ThemeMode::default(), &theme);
        println!("style: {style:#?}");
    }

    #[test]
    fn breakpoints_create_media_queries() {
        let theme = Theme::new();

        let sx = sx! {
            padding: "15px",
            md: {
                padding: "20px"
            }
        };

        let style = sx.to_css(&ThemeMode::default(), &theme);
        println!("style: {style:#?}");
    }

    #[test]
    fn sub_class() {
        let theme = Theme::new();

        let sx = sx! {
            ".box": {
                "p": "10px"
            }
        };

        let style = sx.to_css(&ThemeMode::default(), &theme);
        println!("style: {style:#?}");
    }
}
