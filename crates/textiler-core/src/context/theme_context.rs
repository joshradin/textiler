use crate::theme::Theme;
use std::ops::Deref;
use yew::UseStateHandle;

/// The theme context
#[derive(Debug, Clone)]
pub struct ThemeContext {
    inner: UseStateHandle<Theme>,
}

impl ThemeContext {
    pub(crate) fn new(inner: UseStateHandle<Theme>) -> Self {
        Self { inner }
    }

    /// Modifies the theme
    pub fn modify<F: FnOnce(&mut Theme)>(&self, cb: F) {
        let mut theme: Theme = (*self.inner).clone();
        cb(&mut theme);
        self.inner.set(theme);
    }
}

impl Deref for ThemeContext {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}
