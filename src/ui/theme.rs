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
// Stone/rock colors (for Mine screen ASCII art border)
pub const DEEP_SLATE: Color = Color::Rgb(30, 30, 35);
pub const DARK_STONE: Color = Color::Rgb(50, 50, 55);
pub const GRANITE: Color = Color::Rgb(80, 80, 85);
pub const LIGHT_STONE: Color = Color::Rgb(120, 120, 125);
pub const PALE_ROCK: Color = Color::Rgb(160, 160, 165);
// Mystic/Alchemy colors (for Alchemist tab ASCII art border)
pub const DEEP_VIOLET: Color = Color::Rgb(48, 25, 70);
pub const DARK_PURPLE: Color = Color::Rgb(75, 35, 100);
pub const MYSTIC_PURPLE: Color = Color::Rgb(120, 60, 150);
pub const BRIGHT_VIOLET: Color = Color::Rgb(160, 90, 200);
pub const PALE_LAVENDER: Color = Color::Rgb(200, 150, 230);
pub const MYSTIC_GLOW: Color = Color::Rgb(180, 100, 255);
pub const VAPOR_CYAN: Color = Color::Rgb(100, 200, 220);
pub const BUBBLE_GREEN: Color = Color::Rgb(100, 255, 150);
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
pub const MINE_BG: Color = Color::Rgb(32, 32, 38); // Very subtle grey tint
// Cave interior colors (greyscale for top-down mine view)
pub const CAVE_WALL_DARK: Color = Color::Rgb(60, 60, 65);    // Dense wall rock (#)
pub const CAVE_WALL_MID: Color = Color::Rgb(90, 90, 95);     // Wall texture (@)
pub const CAVE_WALL_LIGHT: Color = Color::Rgb(120, 120, 125); // Wall edge (%)
pub const CAVE_FLOOR_EDGE: Color = Color::Rgb(150, 150, 155); // Floor edge transition (;)
pub const CAVE_FLOOR_BG: Color = Color::Rgb(25, 25, 28);     // Open floor background (darker)
pub const ALCHEMIST_BG: Color = Color::Rgb(38, 32, 48); // Very subtle purple tint
pub const ORANGE: Color = Color::Rgb(255, 152, 0);
pub const PURPLE: Color = Color::Rgb(171, 71, 188);
pub const BRONZE: Color = Color::Rgb(205, 127, 50);

// Icon constants (Unicode symbols from nerd fonts)
pub mod icons {
    pub const HEART: char           = '\u{F004}';
    pub const COIN: char            = '\u{EDE8}';
    pub const CROSSED_SWORDS: char  = '\u{f0787}';
    pub const CHECKED: char         = '\u{F14A}';
    pub const UNCHECKED: char       = '\u{F0C8}';
    pub const STORE: char           = '\u{ee17}';
    pub const PERSON: char          = '\u{F415}';
    pub const SHIRT: char           = '\u{EE1C}';
    pub const OPEN_DOOR: char       = '\u{F081C}';
    pub const SHIELD: char          = '\u{F132}';
    pub const ANVIL: char           = '\u{F089B}';
    pub const DOUBLE_ARROW_UP: char = '\u{F102}';
    pub const HOUSE: char           = '\u{F015}';
    pub const RETURN_ARROW: char    = '\u{F17B1}';
    pub const HAMMER: char          = '\u{EEFF}';
    pub const LOCK: char            = '\u{F023}';
    pub const PICKAXE: char         = '\u{F08B7}';
    pub const HOURGLASS: char       = '\u{F252}';
    pub const FIRE: char            = '\u{F0238}';
    pub const FLASK: char           = '\u{F0093}';
}

// Soft colors for stat comparisons
pub const SOFT_GREEN: Color = Color::Rgb(100, 200, 100);
pub const SOFT_RED: Color = Color::Rgb(200, 100, 100);
// Rock type colors
pub const COPPER_ORE: Color = Color::Rgb(184, 115, 51);  // Copper orange
pub const TIN_ORE: Color = Color::Rgb(180, 180, 190);    // Silver-ish
pub const COAL_ORE: Color = Color::Rgb(40, 40, 45);      // Dark coal

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
