use bevy::prelude::Color;

// Book container dimensions
pub const BOOK_WIDTH: f32 = 672.0;
pub const BOOK_HEIGHT: f32 = 399.0;

// Left page layout
pub const LEFT_PAGE_LEFT: f32 = 45.0;
pub const LEFT_PAGE_TOP: f32 = 40.0;
pub const LEFT_PAGE_WIDTH: f32 = 260.0;
pub const LEFT_PAGE_HEIGHT: f32 = 320.0;
pub const LEFT_PAGE_ROW_GAP: f32 = 4.0;

// Right page layout
pub const RIGHT_PAGE_LEFT: f32 = 360.0;
pub const RIGHT_PAGE_TOP: f32 = 40.0;
pub const RIGHT_PAGE_WIDTH: f32 = 280.0;
pub const RIGHT_PAGE_HEIGHT: f32 = 320.0;

// Sprite dimensions
pub const MOB_SPRITE_SIZE: f32 = 96.0;

// Typography
pub const MONSTER_NAME_FONT_SIZE: f32 = 14.0;

// Colors
pub const SELECTED_COLOR: Color = Color::srgb(0.5, 0.3, 0.1);
pub const NORMAL_COLOR: Color = Color::srgb(0.2, 0.15, 0.1);

// Stats section layout
pub const STAT_ICON_SIZE: f32 = 16.0;
pub const STAT_ROW_HEIGHT: f32 = 18.0;
pub const STAT_FONT_SIZE: f32 = 12.0;

// Drops section layout
pub const DROP_ROW_HEIGHT: f32 = 18.0;
pub const DROP_ICON_SIZE: f32 = 16.0;
pub const DROP_FONT_SIZE: f32 = 12.0;
