//! Theme context

use std::ops::{Deref, DerefMut};
use stylist::ast::ToStyleStr;
use stylist::manager::StyleManager;
use wasm_bindgen::JsCast;
use yew::{function_component, html, use_effect_with, use_state_eq, Children, Html, Properties};

use crate::theme::baseline::baseline;
use crate::theme::theme_mode::ThemeMode;
use crate::theme::{Theme};
use crate::{Error, hooks, Sx};

pub use self::{mode_context::ThemeModeContext, style_manager_context::StyleManagerContext, theme_context::ThemeContext};

mod mode_context;
mod style_manager_context;
mod theme_context;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub theme: Theme,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme_state = ThemeContext::new(yew::use_state_eq(|| props.theme.clone()));
    let manager = StyleManagerContext::new(yew::use_memo(theme_state.clone(), |_| {
        StyleManager::builder()
            .prefix(theme_state.prefix.clone().into())
            .build()
            .expect("could not create style manager")
    }));
    let mode = ThemeModeContext::new(use_state_eq(|| ThemeMode::System.detect()));

    html! {
            <yew::ContextProvider<ThemeContext> context={theme_state}>
                <yew::ContextProvider<ThemeModeContext> context={mode}>
                    <yew::ContextProvider<StyleManagerContext> context={manager}>
                        {for props.children.iter()}
                    </yew::ContextProvider<StyleManagerContext>>
                </yew::ContextProvider<ThemeModeContext>>
            </yew::ContextProvider<ThemeContext>>
    }
}

#[function_component]
pub fn CssBaseline() -> Html {
    let theme = hooks::use_theme();
    let style_manager: StyleManagerContext = hooks::use_style_manager();
    let (mode, ..) = hooks::use_mode();

    use_effect_with((theme, mode), move |(theme, mode)| {
        style_manager
            .mount(theme, &mode, baseline(theme, &mode))
            .expect("could not mount sx");
    });

    html! {}
}
