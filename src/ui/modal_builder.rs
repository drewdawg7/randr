//! Modal builder for creating consistent modal UIs.
//!
//! # Examples
//!
//! ```ignore
//! // Standard modal with solid background
//! commands.spawn_modal(
//!     Modal::new()
//!         .title("Inventory")
//!         .size(600.0, 400.0)
//!         .hint("[Esc] Close")
//!         .with_root_marker(|e| { e.insert(InventoryModalRoot); })
//!         .content(|c| {
//!             c.spawn(Text::new("Hello"));
//!         })
//! );
//!
//! // Atlas background (e.g., book sprite)
//! commands.spawn_modal(
//!     Modal::new()
//!         .background(ModalBackground::Atlas { texture, layout, index })
//!         .size(672.0, 399.0)
//!         .with_root_marker(|e| { e.insert(MonsterCompendiumRoot); })
//!         .content(|c| { /* absolute positioning */ })
//! );
//! ```

use bevy::prelude::*;

use crate::ui::hints::spawn_modal_hint;
use crate::ui::screens::modal::{ModalContent, ModalOverlay, ModalType};

// ============================================================================
// Constants
// ============================================================================

/// Modal overlay background color (semi-transparent black).
pub const MODAL_OVERLAY_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);

/// Modal overlay z-index.
pub const MODAL_OVERLAY_Z_INDEX: i32 = 100;

/// Modal container background color (dark brown).
pub const MODAL_BG_COLOR: Color = Color::srgb(0.15, 0.12, 0.1);

/// Modal container border color (tan).
pub const MODAL_BORDER_COLOR: Color = Color::srgb(0.6, 0.5, 0.3);

/// Modal title text color (cream).
pub const MODAL_TITLE_COLOR: Color = Color::srgb(0.95, 0.9, 0.7);

/// Default modal width in pixels.
pub const MODAL_DEFAULT_WIDTH: f32 = 800.0;

/// Default modal maximum width percentage.
pub const MODAL_DEFAULT_MAX_WIDTH_PERCENT: f32 = 90.0;

/// Default modal maximum height percentage.
pub const MODAL_DEFAULT_MAX_HEIGHT_PERCENT: f32 = 80.0;

/// Default modal padding in pixels.
pub const MODAL_DEFAULT_PADDING: f32 = 30.0;

/// Default modal border width in pixels.
pub const MODAL_DEFAULT_BORDER_WIDTH: f32 = 3.0;

/// Modal title font size.
pub const MODAL_TITLE_FONT_SIZE: f32 = 48.0;

/// Modal title bottom margin.
pub const MODAL_TITLE_MARGIN_BOTTOM: f32 = 20.0;

// ============================================================================
// ModalBackground
// ============================================================================

/// Background type for the modal container.
#[derive(Clone)]
pub enum ModalBackground {
    /// Solid color with border (default modal style).
    Solid {
        background: Color,
        border: Color,
    },
    /// Texture atlas image (e.g., book sprite for Monster Compendium).
    /// Container uses `PositionType::Relative`; children should use absolute positioning.
    Atlas {
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        index: usize,
    },
}

impl Default for ModalBackground {
    fn default() -> Self {
        Self::Solid {
            background: MODAL_BG_COLOR,
            border: MODAL_BORDER_COLOR,
        }
    }
}

// ============================================================================
// Modal Builder
// ============================================================================

/// Content closure type alias.
type ContentFn = Box<dyn FnOnce(&mut ChildBuilder) + Send + Sync>;

/// Root marker closure type alias.
type RootMarkerFn = Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>;

/// Builder for creating modal UI with consistent styling.
pub struct Modal {
    /// Title text displayed at top (only for Solid backgrounds).
    title: Option<String>,
    /// Hint text displayed at bottom (only for Solid backgrounds).
    hints: Vec<String>,
    /// Width and height in pixels.
    size: Option<(f32, f32)>,
    /// Maximum width as percentage (default: 90%).
    max_width_percent: Option<f32>,
    /// Maximum height as percentage (default: 80%).
    max_height_percent: Option<f32>,
    /// Background type.
    background: ModalBackground,
    /// Padding in pixels (only for Solid background).
    padding: Option<f32>,
    /// Border width in pixels (only for Solid background).
    border_width: Option<f32>,
    /// Modal type for ActiveModal tracking.
    modal_type: Option<ModalType>,
    /// Content closure.
    content: Option<ContentFn>,
    /// Custom root marker component closure.
    root_marker: Option<RootMarkerFn>,
}

impl Default for Modal {
    fn default() -> Self {
        Self {
            title: None,
            hints: Vec::new(),
            size: None,
            max_width_percent: None,
            max_height_percent: None,
            background: ModalBackground::default(),
            padding: None,
            border_width: None,
            modal_type: None,
            content: None,
            root_marker: None,
        }
    }
}

impl Modal {
    /// Create a new modal builder with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    // ===== Content Methods =====

    /// Set the modal title (only rendered for Solid backgrounds).
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add a hint line at the bottom (only rendered for Solid backgrounds).
    /// Can be called multiple times for multiple hint lines.
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
        self
    }

    /// Set the content builder closure.
    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildBuilder) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }

    // ===== Size Methods =====

    /// Set exact width and height in pixels.
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = Some((width, height));
        self
    }

    /// Set maximum width as percentage.
    pub fn max_width_percent(mut self, percent: f32) -> Self {
        self.max_width_percent = Some(percent);
        self
    }

    /// Set maximum height as percentage.
    pub fn max_height_percent(mut self, percent: f32) -> Self {
        self.max_height_percent = Some(percent);
        self
    }

    // ===== Styling Methods =====

    /// Set the background type.
    pub fn background(mut self, bg: ModalBackground) -> Self {
        self.background = bg;
        self
    }

    /// Set padding in pixels (only applies to Solid backgrounds).
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding);
        self
    }

    /// Set border width in pixels (only applies to Solid backgrounds).
    pub fn border(mut self, width: f32) -> Self {
        self.border_width = Some(width);
        self
    }

    // ===== Tracking Methods =====

    /// Set the modal type for ActiveModal resource tracking.
    #[allow(dead_code)]
    pub fn modal_type(mut self, modal_type: ModalType) -> Self {
        self.modal_type = Some(modal_type);
        self
    }

    /// Add a custom marker component to the root overlay entity.
    /// This allows the caller to query and despawn the modal.
    pub fn with_root_marker<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut EntityCommands) + Send + Sync + 'static,
    {
        self.root_marker = Some(Box::new(f));
        self
    }
}

// ============================================================================
// SpawnModalExt
// ============================================================================

/// Extension trait for spawning modals via Commands.
pub trait SpawnModalExt {
    /// Spawn a modal and return the overlay entity ID.
    fn spawn_modal(&mut self, modal: Modal) -> Entity;
}

impl SpawnModalExt for Commands<'_, '_> {
    fn spawn_modal(&mut self, modal: Modal) -> Entity {
        // Destructure the modal to take ownership of all fields
        let Modal {
            title,
            hints,
            size,
            max_width_percent,
            max_height_percent,
            background,
            padding,
            border_width,
            modal_type: _modal_type,
            content,
            root_marker,
        } = modal;

        // 1. Spawn the overlay (full-screen semi-transparent background)
        let overlay = self
            .spawn((
                ModalOverlay,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(MODAL_OVERLAY_COLOR),
                ZIndex(MODAL_OVERLAY_Z_INDEX),
            ))
            .id();

        // 2. Apply custom root marker if provided
        if let Some(marker_fn) = root_marker {
            marker_fn(&mut self.entity(overlay));
        }

        // 3. Build the modal container based on background type
        match background {
            ModalBackground::Solid {
                background: bg_color,
                border: border_color,
            } => {
                self.entity(overlay).with_children(|parent| {
                    spawn_solid_modal(
                        parent,
                        title,
                        hints,
                        size,
                        max_width_percent,
                        max_height_percent,
                        padding,
                        border_width,
                        bg_color,
                        border_color,
                        content,
                    );
                });
            }
            ModalBackground::Atlas {
                texture,
                layout,
                index,
            } => {
                self.entity(overlay).with_children(|parent| {
                    spawn_atlas_modal(parent, size, texture, layout, index, content);
                });
            }
        }

        overlay
    }
}

// ============================================================================
// Private Helpers
// ============================================================================

/// Spawn a modal with solid color background.
fn spawn_solid_modal(
    parent: &mut ChildBuilder,
    title: Option<String>,
    hints: Vec<String>,
    size: Option<(f32, f32)>,
    max_width_percent: Option<f32>,
    max_height_percent: Option<f32>,
    padding: Option<f32>,
    border_width: Option<f32>,
    background: Color,
    border: Color,
    content: Option<ContentFn>,
) {
    let (width, height) = size.unwrap_or((MODAL_DEFAULT_WIDTH, 0.0));
    let max_width = max_width_percent.unwrap_or(MODAL_DEFAULT_MAX_WIDTH_PERCENT);
    let max_height = max_height_percent.unwrap_or(MODAL_DEFAULT_MAX_HEIGHT_PERCENT);
    let padding_val = padding.unwrap_or(MODAL_DEFAULT_PADDING);
    let border_val = border_width.unwrap_or(MODAL_DEFAULT_BORDER_WIDTH);

    parent
        .spawn((
            ModalContent,
            Node {
                width: Val::Px(width),
                max_width: Val::Percent(max_width),
                height: if height > 0.0 {
                    Val::Px(height)
                } else {
                    Val::Auto
                },
                max_height: Val::Percent(max_height),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(padding_val)),
                border: UiRect::all(Val::Px(border_val)),
                ..default()
            },
            BackgroundColor(background),
            BorderColor(border),
        ))
        .with_children(|container| {
            // Title (if provided)
            if let Some(title_text) = title {
                container.spawn((
                    Text::new(title_text),
                    TextFont {
                        font_size: MODAL_TITLE_FONT_SIZE,
                        ..default()
                    },
                    TextColor(MODAL_TITLE_COLOR),
                    Node {
                        margin: UiRect::bottom(Val::Px(MODAL_TITLE_MARGIN_BOTTOM)),
                        ..default()
                    },
                ));
            }

            // Content (if provided)
            if let Some(content_fn) = content {
                content_fn(container);
            }

            // Hints (if provided)
            if !hints.is_empty() {
                container
                    .spawn(Node {
                        flex_direction: FlexDirection::Column,
                        row_gap: Val::Px(5.0),
                        margin: UiRect::top(Val::Px(15.0)),
                        ..default()
                    })
                    .with_children(|hints_container| {
                        for hint in &hints {
                            spawn_modal_hint(hints_container, hint);
                        }
                    });
            }
        });
}

/// Spawn a modal with texture atlas background.
fn spawn_atlas_modal(
    parent: &mut ChildBuilder,
    size: Option<(f32, f32)>,
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    index: usize,
    content: Option<ContentFn>,
) {
    let (width, height) = size.unwrap_or((672.0, 399.0)); // Book default size

    parent
        .spawn((
            ModalContent,
            ImageNode::from_atlas_image(texture, TextureAtlas { layout, index }),
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                position_type: PositionType::Relative, // For absolute child positioning
                ..default()
            },
        ))
        .with_children(|container| {
            // Content only - no title/hints for atlas backgrounds
            // Caller uses absolute positioning for children
            if let Some(content_fn) = content {
                content_fn(container);
            }
        });
}
