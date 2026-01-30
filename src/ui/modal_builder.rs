use bevy::prelude::*;

use crate::ui::hints::spawn_modal_hint;
use crate::ui::screens::modal::{ModalContent, ModalOverlayBundle, ModalType};

pub const MODAL_BG_COLOR: Color = Color::srgb(0.15, 0.12, 0.1);
pub const MODAL_BORDER_COLOR: Color = Color::srgb(0.6, 0.5, 0.3);
pub const MODAL_TITLE_COLOR: Color = Color::srgb(0.95, 0.9, 0.7);
pub const MODAL_DEFAULT_WIDTH: f32 = 800.0;
pub const MODAL_DEFAULT_MAX_WIDTH_PERCENT: f32 = 90.0;
pub const MODAL_DEFAULT_MAX_HEIGHT_PERCENT: f32 = 80.0;
pub const MODAL_DEFAULT_PADDING: f32 = 30.0;
pub const MODAL_DEFAULT_BORDER_WIDTH: f32 = 3.0;
pub const MODAL_TITLE_FONT_SIZE: f32 = 48.0;
pub const MODAL_TITLE_MARGIN_BOTTOM: f32 = 20.0;

#[derive(Clone)]
pub enum ModalBackground {
    Solid {
        background: Color,
        border: Color,
    },
    Atlas {
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        index: usize,
    },
    None,
}

impl Default for ModalBackground {
    fn default() -> Self {
        Self::Solid {
            background: MODAL_BG_COLOR,
            border: MODAL_BORDER_COLOR,
        }
    }
}

type ContentFn = Box<dyn FnOnce(&mut ChildBuilder) + Send + Sync>;
type RootMarkerFn = Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>;

pub struct Modal {
    title: Option<String>,
    hints: Vec<String>,
    size: Option<(f32, f32)>,
    max_width_percent: Option<f32>,
    max_height_percent: Option<f32>,
    background: ModalBackground,
    padding: Option<f32>,
    border_width: Option<f32>,
    modal_type: Option<ModalType>,
    content: Option<ContentFn>,
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
        self
    }

    pub fn content<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut ChildBuilder) + Send + Sync + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = Some((width, height));
        self
    }

    pub fn max_width_percent(mut self, percent: f32) -> Self {
        self.max_width_percent = Some(percent);
        self
    }

    pub fn max_height_percent(mut self, percent: f32) -> Self {
        self.max_height_percent = Some(percent);
        self
    }

    pub fn background(mut self, bg: ModalBackground) -> Self {
        self.background = bg;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = Some(padding);
        self
    }

    pub fn border(mut self, width: f32) -> Self {
        self.border_width = Some(width);
        self
    }

    #[allow(dead_code)]
    pub fn modal_type(mut self, modal_type: ModalType) -> Self {
        self.modal_type = Some(modal_type);
        self
    }

    pub fn with_root_marker<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&mut EntityCommands) + Send + Sync + 'static,
    {
        self.root_marker = Some(Box::new(f));
        self
    }
}

pub trait SpawnModalExt {
    fn spawn_modal(&mut self, modal: Modal) -> Entity;
}

impl SpawnModalExt for Commands<'_, '_> {
    fn spawn_modal(&mut self, modal: Modal) -> Entity {
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

        let overlay = self.spawn(ModalOverlayBundle::new()).id();

        if let Some(marker_fn) = root_marker {
            marker_fn(&mut self.entity(overlay));
        }

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
            ModalBackground::None => {
                if let Some(content_fn) = content {
                    self.entity(overlay).with_children(|parent| {
                        content_fn(parent);
                    });
                }
            }
        }

        overlay
    }
}

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

            if let Some(content_fn) = content {
                content_fn(container);
            }

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

fn spawn_atlas_modal(
    parent: &mut ChildBuilder,
    size: Option<(f32, f32)>,
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    index: usize,
    content: Option<ContentFn>,
) {
    let (width, height) = size.unwrap_or((672.0, 399.0));

    parent
        .spawn((
            ModalContent,
            ImageNode::from_atlas_image(texture, TextureAtlas { layout, index }),
            Node {
                width: Val::Px(width),
                height: Val::Px(height),
                position_type: PositionType::Relative,
                ..default()
            },
        ))
        .with_children(|container| {
            if let Some(content_fn) = content {
                content_fn(container);
            }
        });
}
