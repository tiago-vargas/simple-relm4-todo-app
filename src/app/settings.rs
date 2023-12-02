pub(crate) enum Settings {
    WindowWidth,
    WindowHeight,
    WindowIsMaximized,
}

impl Settings {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::WindowWidth => "window-width",
            Self::WindowHeight => "window-height",
            Self::WindowIsMaximized => "window-is-maximized",
        }
    }
}
