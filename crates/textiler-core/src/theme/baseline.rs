use crate::theme::sx::SxValue;
use crate::theme::theme_mode::ThemeMode;
use crate::theme::typography::TypographyLevel;
use crate::theme::Theme;
use crate::{sx, Sx};

/// Creates the base style sheet for happiness
pub fn baseline(theme: &Theme, mode: &ThemeMode) -> Sx {
    let mut emit = sx!();

    let theme_system_class = theme.system_class();
    for (typography_level, _) in theme.typography() {
        if typography_level != &TypographyLevel::Star {
            let sx = theme.typography().at(typography_level).unwrap();
            emit.insert(format!("{theme_system_class}.{typography_level}"), sx);
        }
    }

    for (palette_name, palette) in theme.palettes() {
        let mut to_merge = sx!();
        for selector_name in palette.selectors() {
            let mut selector = palette.select(selector_name, mode).unwrap().clone();
            if let Ok(adjusted) = selector.to_rgba_color() {
                selector = adjusted;
            }
            to_merge.insert(
                theme.palette_var(palette_name, selector_name),
                SxValue::Color(selector),
            )
        }
        emit = emit.merge(sx! {
            "html": to_merge
        })
    }

    emit.merge(sx! {
        ":root, html": {
            "color": "text.primary",
            "bgcolor": "background.body",
        },
        "p, span, code, h1, h2, h3, h4": {
            "margin-block-start": "0.1em",
            "margin-block-end": "0.1em",
            "margin-inline-start": "0px",
            "margin-inline-end": "0px",
        },
        "body": {
            "margin": "0",
        },
        (theme.system_class()): {
            "&[color=success]": {
                "color": "success.050",
            },
            "&[variant=outlined]": {
                "borderWidth": "3px",
                "borderStyle": "solid",
                "padding": "3px",
                "borderColor": "inherit",
                "&[color=success]": {
                    "borderColor": "success.outlinedBorder",
                    "color": "success.outlinedColor",
                    "&[disabled]": {
                        "borderColor": "success.outlinedDisabledBorder",
                        "color": "success.outlinedDisabledColor",
                    }
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::theme::baseline::baseline;
    use crate::theme::theme_mode::ThemeMode;

    #[test]
    fn create_light_baseline() {
        let theme = Theme::default();
        let baseline = baseline(&theme, &ThemeMode::Light);
        println!("baseline: {baseline:#?}");
    }
}
