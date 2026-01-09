//! Text builder for UI elements.
//!
//! Provides a builder pattern for spawning text with sensible defaults.
//!
//! # Example
//!
//! ```ignore
//! // Using presets
//! parent.spawn(UiText::title("Inventory").build_with_node());
//! parent.spawn(UiText::body("Choose an item").build());
//!
//! // Custom styling
//! parent.spawn(UiText::new("Health: 100").size(20.0).green().build());
//! ```

use bevy::prelude::*;

/// Default font size for body text.
pub const DEFAULT_FONT_SIZE: f32 = 18.0;

/// Common text colors.
pub mod text_colors {
    use bevy::prelude::Color;

    pub const WHITE: Color = Color::WHITE;
    pub const GRAY: Color = Color::srgb(0.7, 0.7, 0.7);
    pub const DARK_GRAY: Color = Color::srgb(0.6, 0.6, 0.6);
    pub const YELLOW: Color = Color::srgb(1.0, 0.9, 0.3);
    pub const CREAM: Color = Color::srgb(0.95, 0.9, 0.7);
    pub const GREEN: Color = Color::srgb(0.5, 0.8, 0.5);
    pub const RED: Color = Color::srgb(0.8, 0.5, 0.5);
    pub const GOLD: Color = Color::srgb(0.9, 0.8, 0.3);
}

/// Common font sizes.
pub mod font_sizes {
    pub const SMALL: f32 = 14.0;
    pub const BODY: f32 = 18.0;
    pub const MEDIUM: f32 = 20.0;
    pub const LARGE: f32 = 24.0;
    pub const HEADING: f32 = 28.0;
    pub const TITLE: f32 = 48.0;
}

/// Builder for creating styled UI text.
#[derive(Clone)]
pub struct UiText {
    text: String,
    font_size: f32,
    color: Color,
    margin_bottom: Option<f32>,
    margin_top: Option<f32>,
}

impl UiText {
    /// Create a new text builder with default settings (18px, white).
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            font_size: DEFAULT_FONT_SIZE,
            color: Color::WHITE,
            margin_bottom: None,
            margin_top: None,
        }
    }

    // ===== Size Methods =====

    /// Set custom font size.
    pub fn size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Small text (14px) - for hints, secondary info.
    pub fn small(mut self) -> Self {
        self.font_size = font_sizes::SMALL;
        self
    }

    /// Medium text (20px) - slightly emphasized.
    pub fn medium(mut self) -> Self {
        self.font_size = font_sizes::MEDIUM;
        self
    }

    /// Large text (24px) - section headers.
    pub fn large(mut self) -> Self {
        self.font_size = font_sizes::LARGE;
        self
    }

    /// Heading text (28px) - important headers.
    pub fn heading(mut self) -> Self {
        self.font_size = font_sizes::HEADING;
        self
    }

    // ===== Color Methods =====

    /// Set custom color.
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// White text (default).
    pub fn white(mut self) -> Self {
        self.color = text_colors::WHITE;
        self
    }

    /// Gray text - for labels, hints, disabled.
    pub fn gray(mut self) -> Self {
        self.color = text_colors::GRAY;
        self
    }

    /// Dark gray text - for subtle hints.
    pub fn dark_gray(mut self) -> Self {
        self.color = text_colors::DARK_GRAY;
        self
    }

    /// Yellow text - for titles, highlights.
    pub fn yellow(mut self) -> Self {
        self.color = text_colors::YELLOW;
        self
    }

    /// Cream text - for modal titles.
    pub fn cream(mut self) -> Self {
        self.color = text_colors::CREAM;
        self
    }

    /// Green text - for positive values, player.
    pub fn green(mut self) -> Self {
        self.color = text_colors::GREEN;
        self
    }

    /// Red text - for negative values, enemy, danger.
    pub fn red(mut self) -> Self {
        self.color = text_colors::RED;
        self
    }

    /// Gold text - for currency, special.
    pub fn gold(mut self) -> Self {
        self.color = text_colors::GOLD;
        self
    }

    // ===== Margin Methods =====

    /// Add margin below the text.
    pub fn margin_bottom(mut self, px: f32) -> Self {
        self.margin_bottom = Some(px);
        self
    }

    /// Add margin above the text.
    pub fn margin_top(mut self, px: f32) -> Self {
        self.margin_top = Some(px);
        self
    }

    // ===== Preset Constructors =====

    /// Title text preset (48px, cream, margin bottom 20px).
    pub fn title(text: impl Into<String>) -> Self {
        Self::new(text)
            .size(font_sizes::TITLE)
            .cream()
            .margin_bottom(20.0)
    }

    /// Section header preset (24px, yellow, margin bottom 10px).
    pub fn section(text: impl Into<String>) -> Self {
        Self::new(text)
            .large()
            .yellow()
            .margin_bottom(10.0)
    }

    /// Body text preset (18px, white).
    pub fn body(text: impl Into<String>) -> Self {
        Self::new(text)
    }

    /// Label/hint text preset (16px, gray).
    pub fn label(text: impl Into<String>) -> Self {
        Self::new(text).size(16.0).gray()
    }

    /// Stat value preset (20px, white - chain .green()/.red() for +/-).
    pub fn stat(text: impl Into<String>) -> Self {
        Self::new(text).medium()
    }

    /// Instruction text preset (16px, dark gray, margin top 15px).
    pub fn instruction(text: impl Into<String>) -> Self {
        Self::new(text)
            .size(16.0)
            .dark_gray()
            .margin_top(15.0)
    }

    // ===== Build Methods =====

    /// Build into a tuple that can be spawned (without Node).
    pub fn build(self) -> (Text, TextFont, TextColor) {
        (
            Text::new(self.text),
            TextFont {
                font_size: self.font_size,
                ..default()
            },
            TextColor(self.color),
        )
    }

    /// Build with Node component for margin support.
    pub fn build_with_node(self) -> (Text, TextFont, TextColor, Node) {
        let mut margin = UiRect::default();
        if let Some(bottom) = self.margin_bottom {
            margin.bottom = Val::Px(bottom);
        }
        if let Some(top) = self.margin_top {
            margin.top = Val::Px(top);
        }

        (
            Text::new(self.text),
            TextFont {
                font_size: self.font_size,
                ..default()
            },
            TextColor(self.color),
            Node {
                margin,
                ..default()
            },
        )
    }
}
