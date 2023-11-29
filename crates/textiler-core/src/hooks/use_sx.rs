
use yew::{hook, use_memo};
use crate::context::StyleManagerContext;

use crate::hooks::{use_mode, use_style_manager, use_theme};
use crate::theme::ThemeMode;
use crate::style_manager::{SxRef, StyleManager};
use crate::theme::sx::Sx;
use crate::theme::Theme;


/// Use sx attaches sx to the css body
#[hook]
pub fn use_sx<F>(source: F) -> SxRef
where
    F : Fn(&Theme, &ThemeMode) -> Sx
{
    let ctx = use_theme();
    let (mode, ..) = use_mode();
    let manager: StyleManagerContext = use_style_manager();

    let sx = source(&ctx, &mode);
    let css = use_memo((sx, ctx, mode), |(sx, ctx, mode)| {
        let theme: &Theme = &*ctx;
        debug!(
            "creating css fron sx:{sx:#?} using theme {} with mode {mode:?}",
            theme.prefix
        );
        sx.clone().to_css(mode, theme)
    });

    manager.mount(&*css).expect("could not mount css")
}
