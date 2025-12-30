use ratatui::style::{Color, Style};

// RGB color constants - adjust values to match your colorscheme
pub const YELLOW: Color = Color::Rgb(255, 204, 0);
pub const CYAN: Color = Color::Rgb(0, 188, 212);
pub const BLUE: Color = Color::Rgb(33, 150, 243);
pub const RED: Color = Color::Rgb(244, 67, 54);
pub const GREEN: Color = Color::Rgb(76, 175, 80);
pub const WHITE: Color = Color::Rgb(240, 240, 240);
pub const MAGENTA: Color = Color::Rgb(156, 39, 176);
pub const DARK_GRAY: Color = Color::Rgb(66, 66, 66);
pub const BLACK: Color = Color::Rgb(18, 18, 18);

pub trait ColorExt {
    fn color(self, color: Color) -> Self;
    fn on_color(self, color: Color) -> Self;
}

impl ColorExt for Style {
    fn color(self, color: Color) -> Self {
        self.fg(color)
    }
    fn on_color(self, color: Color) -> Self {
        self.bg(color)
    }
}
