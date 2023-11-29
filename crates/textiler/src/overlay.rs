use web_sys::MouseEvent;
use yew::{Callback, Children, function_component, ContextProvider, hook, Html, html, Properties, use_context};

#[derive(Debug, Clone, PartialEq)]
pub struct OverlayContext {
    set_onclick: Callback<Callback<MouseEvent>>,
}

#[hook]
pub fn use_overlay() -> Option<Callback<Callback<MouseEvent>>> {
    use_context::<OverlayContext>().map(|over| over.set_onclick)
}

#[derive(Debug, Clone, Properties, PartialEq)]
pub struct OverlayProps {
    pub set_onclick: Callback<Callback<MouseEvent>>,
    pub children: Children
}

#[function_component]
pub fn OverlayProvider(props: &OverlayProps) -> Html {
    let context = OverlayContext {
        set_onclick: props.set_onclick.clone()
    };

    html! {
        <ContextProvider<OverlayContext> context={context.clone()}>
            { for props.children.iter() }
        </ContextProvider<OverlayContext>>
    }
}