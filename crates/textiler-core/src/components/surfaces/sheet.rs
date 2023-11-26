use yew::{function_component, html, Children, Html, Properties};

use crate::style::{Color, Variant};
use crate::system::{ColorProp, VariantProp};
use crate::theme::sx::Sx;
use crate::{components::system::StylingBox, sx, theme::sx::SxValue};

#[derive(Default, Debug, Clone, PartialEq, Properties)]
pub struct SheetProps {
    #[prop_or_default]
    pub sx: Sx,
    #[prop_or_default]
    pub variant: VariantProp,
    #[prop_or_default]
    pub color: ColorProp,
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn Sheet(props: &SheetProps) -> Html {
    let sx = props.sx.clone().merge(sx! {
        "bgcolor": SxValue::var("sheet", "background-color", None),
    });
    let SheetProps { color, variant, .. } = props;

    html! {
        <StylingBox {sx} class={yew::classes!("sheet")} {color} {variant}>
            {for props.children.clone()}
        </StylingBox>
    }
}

#[cfg(test)]
mod tests {
    use yew::{html, ServerRenderer};

    use super::*;

    #[tokio::test]
    async fn render_sheet() {
        #[function_component]
        fn Test() -> Html {
            html! {
                <Sheet>

                </Sheet>
            }
        }

        let rendered = ServerRenderer::<Test>::new().render().await;
        println!("{rendered:?}")
    }
}
