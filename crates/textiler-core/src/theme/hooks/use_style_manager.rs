use crate::theme::context::style_manager_context::StyleManagerContext;
use stylist::manager::StyleManager;
use yew::hook;

/// Use a theme
#[hook]
pub(crate) fn use_style_manager() -> StyleManagerContext {
    let mgr = yew::use_context::<StyleManagerContext>();
    mgr.unwrap_or_default()
}
