use bevy::prelude::*;

/// Standard selection colors (blue theme - used in menus, store, blacksmith, etc.)
pub mod colors {
    use bevy::prelude::Color;

    pub const SELECTED_BG: Color = Color::srgb(0.3, 0.3, 0.6);
    pub const SELECTED_TEXT: Color = Color::WHITE;
    pub const NORMAL_BG: Color = Color::NONE;
    pub const NORMAL_TEXT: Color = Color::srgb(0.8, 0.8, 0.8);

    // Inventory modal variant (brown theme)
    pub const INVENTORY_SELECTED_BG: Color = Color::srgb(0.35, 0.28, 0.22);
    pub const INVENTORY_NORMAL_BG: Color = Color::srgb(0.2, 0.17, 0.15);

    // Navigation variant (text-only, different gray)
    pub const NAV_SELECTED_TEXT: Color = Color::srgb(1.0, 1.0, 1.0);
    pub const NAV_NORMAL_TEXT: Color = Color::srgb(0.7, 0.7, 0.7);

    // Hint text colors
    pub const HINT_TOWN: Color = Color::srgb(0.5, 0.5, 0.5);
    pub const HINT_MODAL: Color = Color::srgb(0.7, 0.7, 0.7);
}

pub const SELECTED_PREFIX: &str = "> ";
pub const NORMAL_PREFIX: &str = "  ";

/// Returns (bg_color, text_color) based on selection state
pub fn selection_colors(is_selected: bool) -> (Color, Color) {
    if is_selected {
        (colors::SELECTED_BG, colors::SELECTED_TEXT)
    } else {
        (colors::NORMAL_BG, colors::NORMAL_TEXT)
    }
}

/// Returns prefix string based on selection state
pub fn selection_prefix(is_selected: bool) -> &'static str {
    if is_selected {
        SELECTED_PREFIX
    } else {
        NORMAL_PREFIX
    }
}

/// Returns inventory modal bg_color based on selection state
pub fn inventory_selection_bg(is_selected: bool) -> Color {
    if is_selected {
        colors::INVENTORY_SELECTED_BG
    } else {
        colors::INVENTORY_NORMAL_BG
    }
}

/// Returns navigation text color based on selection state
pub fn nav_selection_text(is_selected: bool) -> Color {
    if is_selected {
        colors::NAV_SELECTED_TEXT
    } else {
        colors::NAV_NORMAL_TEXT
    }
}
