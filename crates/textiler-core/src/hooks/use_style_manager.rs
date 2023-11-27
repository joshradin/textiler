use yew::hook;
use crate::context::StyleManagerContext;

/// Use a theme
#[hook]
pub(crate) fn use_style_manager() -> StyleManagerContext {
    let mgr = yew::use_context::<StyleManagerContext>();
    mgr.unwrap_or_default()
}
