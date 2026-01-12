use bevy::prelude::*;

use crate::assets::{GameSprites, SpriteSheetKey};
use crate::input::GameAction;
use crate::screens::modal::{spawn_modal_overlay, ActiveModal, ModalType};

/// Plugin that manages the book popup system.
pub struct BookPopupPlugin;

impl Plugin for BookPopupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_book_popup_toggle, handle_book_popup_close));
    }
}

/// Component marker for the book popup UI.
#[derive(Component)]
pub struct BookPopupRoot;

/// System to handle opening the book popup with 'b' key.
fn handle_book_popup_toggle(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    game_sprites: Res<GameSprites>,
    existing_popup: Query<Entity, With<BookPopupRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::OpenBook {
            // Close existing popup if open
            if let Ok(entity) = existing_popup.get_single() {
                commands.entity(entity).despawn_recursive();
                active_modal.modal = None;
            } else if active_modal.modal.is_none() {
                // Only open if no other modal is active
                spawn_book_popup(&mut commands, &game_sprites);
                active_modal.modal = Some(ModalType::Book);
            }
        }
    }
}

/// System to handle closing the book popup with Escape.
fn handle_book_popup_close(
    mut commands: Commands,
    mut action_reader: EventReader<GameAction>,
    mut active_modal: ResMut<ActiveModal>,
    popup_query: Query<Entity, With<BookPopupRoot>>,
) {
    for action in action_reader.read() {
        if *action == GameAction::CloseModal && active_modal.modal == Some(ModalType::Book) {
            if let Ok(entity) = popup_query.get_single() {
                commands.entity(entity).despawn_recursive();
                active_modal.modal = None;
            }
        }
    }
}

/// Spawn the book popup UI showing the Slice_4891 sprite.
fn spawn_book_popup(commands: &mut Commands, game_sprites: &GameSprites) {
    let Some(ui_all) = game_sprites.get(SpriteSheetKey::UiAll) else {
        return;
    };
    let Some(idx) = ui_all.get("Slice_4891") else {
        return;
    };

    let overlay = spawn_modal_overlay(commands);

    commands
        .entity(overlay)
        .insert(BookPopupRoot)
        .with_children(|parent| {
            // Display the sprite at 3x scale (224x133 -> 672x399)
            parent.spawn((
                ImageNode::from_atlas_image(
                    ui_all.texture.clone(),
                    TextureAtlas {
                        layout: ui_all.layout.clone(),
                        index: idx,
                    },
                ),
                Node {
                    width: Val::Px(672.0),
                    height: Val::Px(399.0),
                    ..default()
                },
            ));
        });
}
