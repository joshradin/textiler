use dark_light::Mode;

/// The theme kind
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ThemeMode {
    /// Dark mode
    Dark,
    /// Light mode
    Light,
    /// Follow the system
    #[default]
    System,
}

impl ThemeMode {
    /// Detects system mode if possible, but only has effect if
    /// the mode is System
    pub fn detect(self) -> ThemeMode {
        match self {
            ThemeMode::System => match dark_light::detect() {
                Mode::Dark => ThemeMode::Dark,
                Mode::Light => ThemeMode::Light,
                Mode::Default => ThemeMode::Light,
            },
            other => other,
        }
    }
}
