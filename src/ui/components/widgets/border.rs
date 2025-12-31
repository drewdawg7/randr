use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::ui::theme as colors;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderTheme {
    #[default]
    None,
    Forest,
    Ember,
    Wood,
    Stone,
}

impl BorderTheme {
    fn top_pattern(&self) -> &'static str {
        match self {
            BorderTheme::None => "",
            BorderTheme::Forest => "*^*v",
            BorderTheme::Ember => "~'^\"~",
            BorderTheme::Wood => "=#=[=]=",
            BorderTheme::Stone => "#^.~#",
        }
    }

    fn bottom_pattern(&self) -> &'static str {
        match self {
            BorderTheme::None => "",
            BorderTheme::Forest => "vV",
            BorderTheme::Ember => ".o@O#",
            BorderTheme::Wood => "]_|_[_|_",
            BorderTheme::Stone => "_.-~.",
        }
    }

    fn left_pattern(&self) -> &'static [char] {
        match self {
            BorderTheme::None => &[],
            BorderTheme::Forest => &['}', '{', '|', ':'],
            BorderTheme::Ember => &['!', '|', '*', ':', '\''],
            BorderTheme::Wood => &['[', '|', '#', '|'],
            BorderTheme::Stone => &['[', '{', '|', '#', ':'],
        }
    }

    fn right_pattern(&self) -> &'static [char] {
        match self {
            BorderTheme::None => &[],
            BorderTheme::Forest => &['{', '}', '|', ';'],
            BorderTheme::Ember => &['\'', ':', '|', '*', '!'],
            BorderTheme::Wood => &[']', '|', '#', '|'],
            BorderTheme::Stone => &[']', '}', '|', '#', ':'],
        }
    }

    fn top_colors(&self) -> &'static [Color] {
        match self {
            BorderTheme::None => &[],
            BorderTheme::Forest => &[
                colors::DARK_FOREST,
                colors::FOREST_GREEN,
                colors::GREEN,
                colors::LIME_GREEN,
                colors::PALE_GREEN,
            ],
            BorderTheme::Ember => &[
                colors::COAL_BLACK,
                colors::EMBER_RED,
                colors::DEEP_ORANGE,
                colors::FLAME_ORANGE,
                colors::BRIGHT_YELLOW,
                colors::HOT_WHITE,
                colors::BRIGHT_YELLOW,
                colors::FLAME_ORANGE,
                colors::DEEP_ORANGE,
                colors::EMBER_RED,
            ],
            BorderTheme::Wood => &[
                colors::DARK_WALNUT,
                colors::WOOD_BROWN,
                colors::OAK_BROWN,
                colors::TAN_WOOD,
                colors::LIGHT_BEIGE,
                colors::CREAM_WOOD,
                colors::LIGHT_BEIGE,
                colors::TAN_WOOD,
                colors::OAK_BROWN,
            ],
            BorderTheme::Stone => &[
                colors::DEEP_SLATE,
                colors::DARK_STONE,
                colors::GRANITE,
                colors::LIGHT_STONE,
                colors::PALE_ROCK,
                colors::LIGHT_STONE,
                colors::GRANITE,
                colors::DARK_STONE,
            ],
        }
    }

    fn bottom_colors(&self) -> &'static [Color] {
        match self {
            BorderTheme::None => &[],
            BorderTheme::Forest => self.top_colors(), // Forest uses same colors for top/bottom
            BorderTheme::Ember => &[
                colors::ASH_GRAY,
                colors::COAL_BLACK,
                colors::EMBER_RED,
                colors::DEEP_ORANGE,
                colors::FLAME_ORANGE,
                colors::EMBER_RED,
                colors::ASH_GRAY,
            ],
            BorderTheme::Wood => &[
                colors::WOOD_BROWN,
                colors::DARK_WALNUT,
                colors::OAK_BROWN,
                colors::WOOD_BROWN,
                colors::TAN_WOOD,
                colors::OAK_BROWN,
                colors::DARK_WALNUT,
            ],
            BorderTheme::Stone => &[
                colors::DARK_STONE,
                colors::DEEP_SLATE,
                colors::GRANITE,
                colors::DARK_STONE,
                colors::LIGHT_STONE,
                colors::GRANITE,
                colors::DEEP_SLATE,
            ],
        }
    }

    fn side_colors(&self) -> &'static [Color] {
        match self {
            BorderTheme::None => &[],
            BorderTheme::Forest => self.top_colors(),
            BorderTheme::Ember => &[
                colors::COAL_BLACK,
                colors::EMBER_RED,
                colors::DEEP_ORANGE,
                colors::FLAME_ORANGE,
                colors::BRIGHT_YELLOW,
                colors::HOT_WHITE,
                colors::BRIGHT_YELLOW,
                colors::FLAME_ORANGE,
            ],
            BorderTheme::Wood => &[
                colors::DARK_WALNUT,
                colors::WOOD_BROWN,
                colors::OAK_BROWN,
                colors::TAN_WOOD,
                colors::LIGHT_BEIGE,
                colors::OAK_BROWN,
            ],
            BorderTheme::Stone => self.top_colors(),
        }
    }

    pub fn generate_top_border(&self, width: u16) -> Line<'static> {
        if *self == BorderTheme::None {
            return Line::default();
        }
        generate_line(self.top_pattern(), self.top_colors(), width)
    }

    pub fn generate_bottom_border(&self, width: u16) -> Line<'static> {
        if *self == BorderTheme::None {
            return Line::default();
        }
        generate_line(self.bottom_pattern(), self.bottom_colors(), width)
    }

    pub fn generate_left_border_char(&self, row: u16) -> Span<'static> {
        if *self == BorderTheme::None {
            return Span::default();
        }
        generate_char(self.left_pattern(), self.side_colors(), row)
    }

    pub fn generate_right_border_char(&self, row: u16) -> Span<'static> {
        if *self == BorderTheme::None {
            return Span::default();
        }
        generate_char(self.right_pattern(), self.side_colors(), row)
    }
}

fn generate_line(pattern: &str, colors: &[Color], width: u16) -> Line<'static> {
    let spans: Vec<Span> = pattern
        .chars()
        .cycle()
        .take(width as usize)
        .enumerate()
        .map(|(i, ch)| {
            let color = colors[i % colors.len()];
            Span::styled(ch.to_string(), Style::default().fg(color))
        })
        .collect();

    Line::from(spans)
}

fn generate_char(pattern: &[char], colors: &[Color], row: u16) -> Span<'static> {
    let ch = pattern[row as usize % pattern.len()];
    let color = colors[row as usize % colors.len()];
    Span::styled(ch.to_string(), Style::default().fg(color))
}
