#![allow(dead_code)]
use ratatui::style::{Color, Style};

use crate::item::enums::ItemQuality;

// RGB color constants - adjust values to match your colorscheme
pub const YELLOW: Color = Color::Rgb(255, 204, 0);
pub const CYAN: Color = Color::Rgb(0, 188, 212);
pub const BLUE: Color = Color::Rgb(33, 150, 243);
pub const RED: Color = Color::Rgb(244, 67, 54);
pub const GREEN: Color = Color::Rgb(76, 175, 80);
// Forest border greens (for Field tab ASCII art border)
pub const DARK_FOREST: Color = Color::Rgb(34, 85, 51);
pub const FOREST_GREEN: Color = Color::Rgb(34, 139, 34);
pub const LIME_GREEN: Color = Color::Rgb(124, 179, 66);
pub const PALE_GREEN: Color = Color::Rgb(144, 238, 144);
// Ember/flame colors (for Blacksmith tab ASCII art border)
pub const COAL_BLACK: Color = Color::Rgb(20, 20, 20);
pub const ASH_GRAY: Color = Color::Rgb(60, 55, 50);
pub const EMBER_RED: Color = Color::Rgb(139, 35, 35);
pub const DEEP_ORANGE: Color = Color::Rgb(180, 70, 20);
pub const FLAME_ORANGE: Color = Color::Rgb(255, 120, 30);
pub const BRIGHT_YELLOW: Color = Color::Rgb(255, 200, 50);
pub const HOT_WHITE: Color = Color::Rgb(255, 240, 200);
// Wood/plank colors (for Store tab ASCII art border)
pub const DARK_WALNUT: Color = Color::Rgb(50, 35, 20);
pub const WOOD_BROWN: Color = Color::Rgb(101, 67, 33);
pub const OAK_BROWN: Color = Color::Rgb(139, 90, 43);
pub const TAN_WOOD: Color = Color::Rgb(180, 130, 70);
pub const LIGHT_BEIGE: Color = Color::Rgb(210, 180, 140);
pub const CREAM_WOOD: Color = Color::Rgb(235, 210, 180);
pub const WHITE: Color = Color::Rgb(240, 240, 240);
pub const MAGENTA: Color = Color::Rgb(156, 39, 176);
pub const DARK_GRAY: Color = Color::Rgb(66, 66, 66);
pub const GREY: Color = Color::Rgb(128, 128, 128);
pub const BLACK: Color = Color::Rgb(18, 18, 18);
pub const BACKGROUND: Color = Color::Rgb(36, 40, 59); // Tokyo Night Storm #24283b
pub const HEADER_BG: Color = Color::Rgb(45, 50, 70); // Slightly lighter for location headers
// Themed tab backgrounds (very subtle tints, close to base BACKGROUND)
pub const STORE_BG: Color = Color::Rgb(38, 38, 54); // Very subtle warm tint
pub const BLACKSMITH_BG: Color = Color::Rgb(40, 38, 54); // Very subtle warm tint
pub const FIELD_BG: Color = Color::Rgb(34, 42, 56); // Very subtle green tint
pub const FIGHT_BG: Color = Color::Rgb(34, 42, 56); // Same as Field
pub const ORANGE: Color = Color::Rgb(255, 152, 0);
pub const PURPLE: Color = Color::Rgb(171, 71, 188);
// Soft colors for stat comparisons
pub const SOFT_GREEN: Color = Color::Rgb(100, 200, 100);
pub const SOFT_RED: Color = Color::Rgb(200, 100, 100);

/// Returns a color based on upgrade count (0=white, 1=green, 2=blue, 3=purple, 4+=orange)
pub fn upgrade_color(num_upgrades: i32) -> Color {
    match num_upgrades {
        0 => WHITE,
        1 => GREEN,
        2 => BLUE,
        3 => PURPLE,
        _ => ORANGE,
    }
}

/// Returns a color based on item quality
pub fn quality_color(quality: ItemQuality) -> Color {
    match quality {
        ItemQuality::Poor => GREY,
        ItemQuality::Normal => WHITE,
        ItemQuality::Improved => GREEN,
        ItemQuality::WellForged => BLUE,
        ItemQuality::Masterworked => PURPLE,
        ItemQuality::Mythic => RED,
    }
}

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
