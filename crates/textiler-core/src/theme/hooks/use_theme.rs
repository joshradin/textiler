use crate::theme::context::theme_context::ThemeContext;
use crate::theme::Theme;
use yew::hook;

/// Use a theme
#[hook]
pub fn use_theme() -> Theme {
    let theme: Option<ThemeContext> = yew::use_context::<ThemeContext>();
    theme
        .map(|theme: ThemeContext| (*theme).clone())
        .unwrap_or_default()
}
