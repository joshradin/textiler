//! Contains system components. Not meant for general use

use std::ops::Deref;

use strum::IntoEnumIterator;
use web_sys::{HtmlElement, MouseEvent};
use yew::{AttrValue, Callback, classes, Classes, function_component, html, Html, NodeRef, Properties, ServerRenderer, use_effect, use_effect_with};
use yew::html::{Children, ImplicitClone, IntoPropValue};

use crate::hooks::{use_sx, use_theme};
use crate::style::{Color, Variant};
use crate::theme::sx::Sx;

#[derive(Default, Debug, Clone, PartialEq, Properties)]
pub struct StylingBoxProps {
    /// Element level css
    #[prop_or_default]
    pub sx: Sx,
    /// onclick
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    /// Variant
    #[prop_or_default]
    pub variant: VariantProp,
    #[prop_or_default]
    pub color: ColorProp,
    #[prop_or_else(|| "div".to_string().into())]
    pub component: AttrValue,
    #[prop_or_else(|| classes!("box"))]
    pub class: Classes,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn StylingBox(props: &StylingBoxProps) -> Html {
    let sx = use_sx(|_, _| props.sx.clone());
    let theme = use_theme();
    let mut classes = classes!(sx);
    classes.extend(props.class.clone());
    classes.extend(classes!(format!("{}-system", theme.prefix)));


    let html_ref = yew::use_node_ref();
    {
        let html_ref = html_ref.clone();
        use_effect_with(
            (props.variant, props.color, props.disabled, html_ref),
            |(variant, color, disabled, node)| {
                info!("setting attributes for color and variant for node: {node:?}");
                let element: HtmlElement = node
                    .cast::<HtmlElement>()
                    .expect("should be an html element");
                if let Some(variant) = **variant {
                    element
                        .set_attribute("variant", &variant.to_string())
                        .expect("could not set variant attribute");
                }
                if let Some(color) = **color {
                    element
                        .set_attribute("color", &color.to_string())
                        .expect("could not set color attribute");
                }
                if *disabled {
                    element
                        .set_attribute("disabled", "")
                        .expect("could not set color attribute");
                } else {
                    let _ = element.remove_attribute("disabled");
                }
            },
        );
    }

    html! {
        <@{props.component.to_string()} onclick={props.onclick.clone()} class={classes} ref={html_ref}>
            { for props.children.clone() }
        </@>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct VariantProp(Option<Variant>);

impl IntoPropValue<VariantProp> for &str {
    fn into_prop_value(self) -> VariantProp {
        for var in Variant::iter() {
            if var.as_ref().to_lowercase() == self {
                return VariantProp(Some(var));
            }
        }
        panic!("no variant named {}", self)
    }
}

impl IntoPropValue<VariantProp> for Variant {
    fn into_prop_value(self) -> VariantProp {
        VariantProp(Some(self))
    }
}

impl Deref for VariantProp {
    type Target = Option<Variant>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ImplicitClone for VariantProp {}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ColorProp(Option<Color>);

impl IntoPropValue<ColorProp> for &str {
    fn into_prop_value(self) -> ColorProp {
        for var in Color::iter() {
            if var.as_ref().to_lowercase() == self {
                return ColorProp(Some(var));
            }
        }
        panic!("no Color named {}", self)
    }
}

impl IntoPropValue<ColorProp> for Color {
    fn into_prop_value(self) -> ColorProp {
        ColorProp(Some(self))
    }
}

impl Deref for ColorProp {
    type Target = Option<Color>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ImplicitClone for ColorProp {}

#[cfg(test)]
mod tests {
    use yew::ServerRenderer;

    use super::*;

    #[tokio::test]
    async fn styled_box() {
        let renderer = ServerRenderer::<StylingBox>::new();
        let s = renderer.render().await;
        println!("{s}");
    }
}
