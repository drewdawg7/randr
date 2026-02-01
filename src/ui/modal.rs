use bevy::prelude::*;
use bon::Builder;

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

type ContentFn = Box<dyn FnOnce(&mut ChildSpawnerCommands) + Send + Sync>;
type RootMarkerFn = Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>;

#[derive(Builder)]
#[builder(state_mod(vis = "pub"))]
pub struct Modal {
    #[builder(field)]
    hints: Vec<String>,

    #[builder(into)]
    title: Option<String>,

    size: Option<(f32, f32)>,

    max_width_percent: Option<f32>,

    max_height_percent: Option<f32>,

    #[builder(default)]
    background: ModalBackground,

    padding: Option<f32>,

    border_width: Option<f32>,

    modal_type: Option<ModalType>,

    content: Option<ContentFn>,

    root_marker: Option<RootMarkerFn>,
}

impl<S: modal_builder::State> ModalBuilder<S> {
    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
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
    parent: &mut ChildSpawnerCommands,
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
            BorderColor::all(border),
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
    parent: &mut ChildSpawnerCommands,
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
