use web_sys::MouseEvent;
use yew::{Callback, Children, function_component, Html, html, Properties, use_node_ref};
use crate::overlay::use_overlay;
use crate::typography::Typography;

/// Link properties
#[derive(Debug, Clone, Properties, PartialEq)]
pub struct LinkProps {
    /// Sets overlay
    #[prop_or_default]
    pub overlay: bool,
    #[prop_or_default]
    pub on_click: Option<Callback<MouseEvent>>,
    /// href
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Link(props: &LinkProps) -> Html {
    let overlay_ctx = use_overlay();

    if props.href.is_some() && props.on_click.is_some() {
        panic!("can not set on_click and href")
    }

    if props.overlay {
        let Some(overlay_ctx) = overlay_ctx else {
            panic!("can not use overlay prop in non-overlayable environment");
        };
        if let Some(on_click) = &props.on_click {
            overlay_ctx.emit(on_click.clone());
        }

    }

    html! {
        <Typography component="a">
            { for props.children.iter() }
        </Typography>
    }
}