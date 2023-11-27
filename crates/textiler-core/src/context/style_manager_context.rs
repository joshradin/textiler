use std::ops::Deref;
use std::rc::Rc;

use cfg_if::cfg_if;
use gloo::utils::document;
use stylist::ast::ToStyleStr;
use stylist::manager::StyleManager;
use wasm_bindgen::JsCast;
use web_sys::{HtmlStyleElement, Node};

use crate::{Error, Sx};
use crate::theme::Theme;
use crate::theme::theme_mode::ThemeMode;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct StyleManagerContext {
    manager: Rc<StyleManager>,
}

impl StyleManagerContext {
    pub fn new(manager: Rc<StyleManager>) -> Self {
        Self { manager }
    }

    #[cfg(target_arch = "wasm32")]
    pub(crate) fn mount_wasm(
        &self,
        theme: &Theme,
        mode: &ThemeMode,
        to_mount: Sx,
    ) -> Result<(), Error> {
        let document = document();
        let container = self.container().ok_or(Error::Web(None))?;

        (|| {
            let css = to_mount.to_css(&mode, theme);
            let style_element = document.create_element("style")?;
            let theme_name = format!("theme-{}-main", theme.prefix);
            style_element.set_attribute("data-style", &theme_name)?;
            let base_css = css.to_style_str(None);

            if option_env!("MINIFY_CSS").is_some() {
                match minifier::css::minify(&base_css) {
                    Ok(minified) => {
                        style_element.set_text_content(Some(&minified.to_string()));
                    }
                    Err(non_minified) => {
                        style_element.set_text_content(Some(non_minified));
                    }
                }
            } else {
                style_element.set_text_content(Some(&base_css));
            }

            let list = container.child_nodes();
            let len = list.length();
            let mut existing: Option<Node> = None;
            for i in 0..len {
                if let Some(child) = list.get(i) {
                    if let Some(style_element) = child.dyn_ref::<HtmlStyleElement>() {
                        if style_element.get_attribute("data-style").as_ref() == Some(&theme_name) {
                            existing = Some(child);
                            break;
                        }
                    }
                }
            }

            if let Some(ref existing) = existing {
                container.replace_child(&style_element, existing)?;
            } else {
                container.append_child(&style_element)?;
            }

            Ok(())
        })()
        .map_err(|e| Error::Web(Some(e)))
    }
    pub fn mount(&self, theme: &Theme, mode: &ThemeMode, to_mount: Sx) -> Result<(), crate::Error> {
        cfg_if! {
            if #[cfg(target_arch="wasm32")] {
                self.mount_wasm(theme, mode, to_mount)
            } else {
                Err(Error::MountingUnsupported)
            }
        }
    }
}

impl Deref for StyleManagerContext {
    type Target = StyleManager;

    fn deref(&self) -> &Self::Target {
        &*self.manager
    }
}
