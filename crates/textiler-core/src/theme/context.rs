//! Theme context

use mode_context::ThemeModeContext;
use std::ops::{Deref, DerefMut};
use style_manager_context::StyleManagerContext;
use stylist::ast::ToStyleStr;
use stylist::manager::StyleManager;
use stylist::yew::styled_component;
use theme_context::ThemeContext;
use wasm_bindgen::JsCast;
use yew::{function_component, html, use_effect_with, use_state_eq, Children, Html, Properties};

use crate::theme::baseline::baseline;
use crate::theme::theme_mode::ThemeMode;
use crate::theme::{hooks, Theme};
use crate::{Error, Sx};

pub mod mode_context;
pub mod style_manager_context;
pub mod theme_context;

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
