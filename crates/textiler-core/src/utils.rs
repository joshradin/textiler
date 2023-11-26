//! Utils module

use std::hash::{Hash, Hasher};
use std::ops::Deref;

use heck::ToKebabCase;
use yew::Hook;

pub mod bounded_float;

pub static CSS_SELECTOR_OPERATORS: &[char] = &['.', '+', '>', '~', '&', ','];

/// Converts to css property

pub fn to_property(key: impl AsRef<str>) -> String {
    let key = key.as_ref();
    if (key.starts_with('[') && key.ends_with(']')) || key.starts_with(CSS_SELECTOR_OPERATORS) {
        key.to_string()
    } else {
        key.split_inclusive(CSS_SELECTOR_OPERATORS)
            .map(|key_part| {
                let selector_index = key_part.rfind(CSS_SELECTOR_OPERATORS);
                let selector_op = selector_index.as_ref().map(|index| &key_part[*index..]);

                let reworked = selector_index
                    .map(|index| &key_part[..index])
                    .unwrap_or(key_part)
                    .split('-')
                    .map(ToKebabCase::to_kebab_case)
                    .collect::<Vec<String>>()
                    .join("-");

                format!("{reworked}{}", selector_op.unwrap_or(""))
            })
            .collect::<String>()
    }
}
