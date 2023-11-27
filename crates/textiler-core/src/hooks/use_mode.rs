use crate::context::ThemeModeContext;
use crate::theme::theme_mode::ThemeMode;
use yew::{hook, use_context, Callback};

#[hook]
pub fn use_mode() -> (ThemeMode, Callback<ThemeMode>) {
    let ctx = use_context::<ThemeModeContext>();
    match ctx {
        Some(ctx) => {
            let callback = {
                let ctx = ctx.clone();
                Callback::from(move |mode| ctx.set(mode))
            };
            ((**ctx).clone(), callback)
        }
        None => (ThemeMode::System, Callback::from(|_| {})),
    }
}
