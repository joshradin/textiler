//! The card


use strum::AsRefStr;
use yew::{Callback, ContextProvider, Children, classes, function_component, Html, html, MouseEvent, Properties, ToHtml, use_callback, use_state};

use textiler_core::prelude::*;
use textiler_core::system::{ColorProp, VariantProp};
use textiler_core::theme::sx::SxValue;
use crate::overlay::*;

#[derive(Debug, Clone, PartialEq)]
struct CardContext {
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, AsRefStr)]
pub enum Direction {
    #[default]
    Vertical,
    Horizontal
}

#[derive(Debug, Default, Clone, Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub direction: Option<Direction>,
    #[prop_or_default]
    pub sx: Sx,
    #[prop_or_default]
    pub variant: VariantProp,
    #[prop_or_default]
    pub color: ColorProp,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>
}

#[function_component]
pub fn Card(CardProps { direction, sx, variant, color, children, onclick }: &CardProps) -> Html {
    let class_sx = use_sx(|theme, mode| {
        sx! {
            borderRadius: "8px",
            p: "3px",
            display: "flex",
            width: "fit-content"
        }
    });

    let direction = direction.unwrap_or_default();
    let sx = {
        let sx = sx.clone();
        yew::use_memo(sx, |sx| {
            let mut sx = sx.clone();
            sx.insert("flexDirection", SxValue::CssLiteral(match direction {
                Direction::Horizontal => "row".to_string(),
                Direction::Vertical => "column".to_string()
            }));
            sx
        })
    };


    let onclick = {
        let onclick = onclick.clone();
        use_state(move || {
            onclick
        })
    };

    let set_onclick = {
        let onclick = onclick.clone();
        use_callback(onclick, |onclick: Callback<MouseEvent>, state| {
            state.set(Some(onclick))
        })
    };

    let context = CardContext {
    };


    html! {
        <OverlayProvider set_onclick={set_onclick.clone()}>
        <ContextProvider<CardContext> context={context}>
            <StylingBox onclick={(*onclick).clone()} sx={(*sx).clone()} {variant} {color} class={classes!("card", class_sx)}>
                { for children.iter() }
            </StylingBox>
        </ContextProvider<CardContext>>
        </OverlayProvider>
    }
}


#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardCoverProps {
    #[prop_or_default]
    pub children: Children
}
#[function_component]
pub fn CardCover(props: &CardCoverProps) -> Html {
    html! {

    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardContentProps {
    #[prop_or_default]
    pub children: Children
}
#[function_component]
pub fn CardContent(props: &CardContentProps) -> Html {
    html! {

    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardActionsProps {
    #[prop_or_default]
    pub overlay: bool,
    #[prop_or_default]
    pub children: Children
}

#[function_component]
pub fn CardActions(props: &CardActionsProps) -> Html {

    html! {
        <>
            { for props.children.iter() }
        </>
    }
}

#[cfg(test)]
mod tests {
    use yew::{function_component, Html, html, ServerRenderer};

    use super::*;

    #[tokio::test]
    async fn render_card() {
        #[function_component]
        fn Test() -> Html {
            html! {
                <Card variant="outlined">
                    {"Hello, world"}
                    <CardContent>
                        <Typography>{"hello, world!"}</Typography>
                    </CardContent>
                </Card>
            }
        }

        let rendered = ServerRenderer::<Test>::new().render().await;
        println!("{rendered}")
    }
}