use stylist::Style;
use yew::{hook, use_memo};

use crate::hooks::{use_mode, use_style_manager, use_theme};
use crate::theme::sx::{Sx, SxRef};
use crate::theme::Theme;

#[hook]
pub fn use_sx<Source>(source: Source) -> SxRef
where
    Source: Into<Sx>,
{
    let ctx = use_theme();
    let (mode, ..) = use_mode();
    let manager = use_style_manager();

    let sx = source.into();
    let css = use_memo((sx, ctx, mode), |(sx, ctx, mode)| {
        let theme: &Theme = &*ctx;
        debug!(
            "creating css fron sx:{sx:#?} using theme {} with mode {mode:?}",
            theme.prefix
        );
        sx.clone().to_css(mode, theme)
    });

    let style = Style::new_with_manager((*css).clone(), &*manager).expect("could not create style");
    SxRef::new(style)
}
