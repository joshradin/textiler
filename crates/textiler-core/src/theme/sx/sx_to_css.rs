//! Contains the sx to css algorithm

use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::io::Write;

use heck::ToKebabCase;

use crate::system_props::{CssPropertyTranslator, TranslationUnit};
use crate::theme::sx::sx_value::SxValue;
use crate::theme::theme_mode::ThemeMode;
use crate::theme::Theme;
use crate::{utils, Error, Sx};

/// Converts sx to css
pub fn sx_to_css<'a>(
    ref sx: Sx,
    mode: &ThemeMode,
    theme: &Theme,
    base_query: impl Into<Option<&'a str>>,
) -> Result<crate::style_manager::Css, crate::Error> {
    let mut stack = vec![];
    let option = base_query.into();
    stack.extend(option.clone().map(|c| c.into()));
    let css = _sx_to_css(sx, mode, theme, &mut stack)?;

    let mut buffer = vec![];

    match option {
        None => write!(&mut buffer, "{}", css)?,
        Some(base) => {
            let Css { declarations } = css;
            let rule = Rule {
                query: base.to_string(),
                block: declarations,
            };
            write!(&mut buffer, "{}", rule)?
        }
    }

    Ok(String::from_utf8(buffer)?)
}

fn _sx_to_css<'a>(
    sx: &Sx,
    mode: &ThemeMode,
    theme: &Theme,
    query_stack: &'a mut Vec<String>,
) -> Result<Css, crate::Error> {
    let declarations = sx_to_declarations(sx, mode, theme, query_stack)?;
    Ok(Css { declarations })
}

fn sx_to_declarations<'a>(
    sx: &'a Sx,
    mode: &'a ThemeMode,
    theme: &'a Theme,
    query_stack: &'a mut Vec<String>,
) -> Result<Vec<Declaration>, Error> {
    let mut declarations = vec![];
    let query_stack = query_stack;
    for (key, value) in sx.props.iter() {
        let declaration = property_to_declaration(&*key, value, mode, theme, query_stack)?;
        declarations.extend(declaration);
    }
    Ok(declarations)
}

fn property_to_declaration<'a, 'b: 'a>(
    key: &'b str,
    value: &'b SxValue,
    mode: &'a ThemeMode,
    theme: &'a Theme,
    query_stack: &'a mut Vec<String>,
) -> Result<Vec<Declaration>, crate::Error> {
    let mut value = Cow::<'a, _>::Borrowed(value);
    let translator = TranslationUnit::new(&theme.breakpoints);
    let key = translator.translate(key);
    let resolved: SxValue = loop {
        match value.as_ref() {
            SxValue::Callback(ref cb) => *value.to_mut() = cb.apply(theme),
            SxValue::ThemeToken {
                ref palette,
                ref selector,
            } => {
                let def_palette = theme
                    .get_palette(palette)
                    .unwrap_or_else(|| panic!("no palette named {palette:?} found"));
                let _ = def_palette
                    .select(selector, mode)
                    .unwrap_or_else(|| panic!("Could not find selector {selector:?} in palette"));

                break SxValue::CssLiteral(format!(
                    "var({})",
                    theme.palette_var(palette, selector)
                ));
            }
            SxValue::ClassVar {
                class,
                var,
                fallback,
            } => {
                break SxValue::CssLiteral(match fallback {
                    None => format!("var({})", theme.class_var(class, var)),
                    Some(fallback) => format!(
                        "var({}, {})",
                        theme.class_var(class, var),
                        fallback.clone().to_css().expect("fallback be direct css")
                    ),
                })
            }
            other => break other.clone(),
        }
    };
    match resolved {
        SxValue::Nested(ref nested) => {
            let mut emit = vec![];
            for key in key {
                query_stack.push(key.to_string());
                trace!("query stack: {:?}", query_stack);
                let key = query_stack
                    .iter()
                    .flat_map(|s| translator.translate(s))
                    .fold(String::new(), |accum, next| {
                        if next.starts_with(&['>', '~', '+', ',']) {
                            format!("{}{}", accum, next)
                        } else if next.starts_with("&") {
                            format!("{}{}", accum, next.strip_prefix("&").unwrap())
                        } else {
                            format!("{} {}", accum, next)
                        }
                    });
                let inner = sx_to_declarations(nested, mode, theme, query_stack)?;
                query_stack.pop();
                emit.push(Declaration::Rule(Rule {
                    query: key.to_string(),
                    block: inner,
                }));
            }
            Ok(emit)
        }
        simple => Ok(key
            .into_iter()
            .map(|key| Declaration::SetProperty {
                property: utils::to_property(key),
                value: simple
                    .clone()
                    .to_css()
                    .expect("should always be able to convert to css now"),
            })
            .collect()),
    }
}

#[derive(Debug)]
struct Css {
    declarations: Vec<Declaration>,
}

impl Display for Css {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for dec in &self.declarations {
            write!(f, "{}", dec)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Rule {
    query: String,
    block: Vec<Declaration>,
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let props = self
            .block
            .iter()
            .filter(|f| matches!(f, Declaration::SetProperty { .. }))
            .collect::<Vec<_>>();
        if !props.is_empty() {
            write!(f, "{query} {{", query = self.query)?;
            for dec in props {
                write!(f, "{}", dec)?;
            }
            write!(f, "}}")?;
        }
        for dec in self
            .block
            .iter()
            .filter(|f| matches!(f, Declaration::Rule(_)))
        {
            write!(f, "{}", dec)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Declaration {
    SetProperty { property: String, value: String },
    Rule(Rule),
}

impl Display for Declaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Declaration::SetProperty { property, value } => {
                write!(f, "{}: {};", property, value)
            }
            Declaration::Rule(rule) => {
                write!(f, "{}", rule)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::sx;
    use crate::utils::to_property;

    use super::*;

    #[test]
    fn sx_to_css_test() {
        let sx = sx! {
            "--test-var": "#0f0f0f"
        };
        let ref theme = Theme::default();
        let ref mode = ThemeMode::Dark;

        let css = sx_to_css(sx, mode, theme, ".happiness-8asd").expect("could not create css");
        println!("css: {:?}", css);
    }

    #[test]
    fn sx_to_nested_css_test() {
        let sx = sx! {
            "div": sx! {
                "--color": "common.black",
                "&[style=distributed]": sx! {
                    "font-family": "monospace"
                }
            }
        };
        let ref theme = Theme::default();
        let ref mode = ThemeMode::Dark;

        let css = sx_to_css(sx, mode, theme, "#root").expect("could not create css");
        println!("Css: {:#?}", css);
    }

    #[test]
    fn format_properties() {
        assert_eq!(to_property("backgroundColor"), "background-color");
        assert_eq!(to_property("background-Color"), "background-color");
        assert_eq!(to_property("--backgroundColor"), "--background-color");
        assert_eq!(to_property("--background-color"), "--background-color");
    }
}
