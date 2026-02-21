use bevy::prelude::*;

use crate::assets::{GameFonts, GameSprites, GridSlotSlice, SpriteSheetKey};
use crate::crafting_station::ForgeCraftingState;
use crate::inventory::Inventory;
use crate::item::{ItemId, ItemRegistry};
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::modal_content_row;
use crate::ui::widgets::{
    spawn_outlined_quantity_text, ItemDetailPane, ItemGrid, ItemGridEntry, ItemGridFocusPanel,
    ItemGridSelection, OutlinedQuantityConfig,
};
use crate::ui::InfoPanelSource;
use crate::ui::{Modal, ModalBackground, SpawnModalExt};

use super::components::{
    ForgeSlotCell, ForgeSlotItemSprite, ForgeSlotQuantityText, LABEL_FONT_SIZE, SLOT_GAP,
    SLOT_SIZE,
};
use super::state::{
    ActiveForgeEntity, ForgeModalRoot, ForgeModalState, ForgePlayerGrid, ForgeSlotIndex,
    ForgeSlotsGrid,
};

pub fn spawn_forge_modal_impl(
    mut commands: Commands,
    game_sprites: &GameSprites,
    game_fonts: &GameFonts,
    inventory: &Inventory,
    forge_state_query: &Query<&ForgeCraftingState>,
    active_forge: &ActiveForgeEntity,
    modal_state: &ForgeModalState,
    registry: &ItemRegistry,
) {
    commands.insert_resource(FocusState {
        focused: Some(FocusPanel::ForgeInventory),
    });

    let player_entries = ItemGridEntry::from_inventory(inventory);
    let forge_state = forge_state_query.get(active_forge.0).ok().cloned();
    let game_sprites = game_sprites.clone();
    let game_fonts = game_fonts.clone();
    let modal_state = modal_state.clone();
    let registry = registry.clone();

    commands.spawn_modal(
        Modal::builder()
            .background(ModalBackground::None)
            .root_marker(Box::new(|e| {
                e.insert(ForgeModalRoot);
            }))
            .content(Box::new(move |c| {
                c.spawn(modal_content_row()).with_children(|row| {
                    spawn_crafting_slots(
                        row,
                        &game_sprites,
                        &game_fonts,
                        forge_state.as_ref(),
                        &modal_state,
                        &registry,
                    );
                    row.spawn((
                        ForgePlayerGrid,
                        ItemGridFocusPanel(FocusPanel::ForgeInventory),
                        ItemGrid {
                            items: player_entries,
                            grid_size: 5,
                        },
                        ItemGridSelection::default(),
                    ));
                    row.spawn(ItemDetailPane {
                        source: InfoPanelSource::None,
                    });
                });
            }))
            .build(),
    );
}

fn spawn_crafting_slots(
    parent: &mut ChildSpawnerCommands,
    game_sprites: &GameSprites,
    game_fonts: &GameFonts,
    forge_state: Option<&ForgeCraftingState>,
    _modal_state: &ForgeModalState,
    registry: &ItemRegistry,
) {
    let slots_width = 3.0 * SLOT_SIZE + 2.0 * SLOT_GAP + 32.0;
    let slots_height = SLOT_SIZE + 40.0;

    parent
        .spawn((
            ForgeSlotsGrid,
            Node {
                width: Val::Px(slots_width),
                height: Val::Px(slots_height),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(8.0)),
                border_radius: BorderRadius::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.08, 0.06, 0.9)),
            BorderColor::all(Color::srgb(0.5, 0.4, 0.3)),
        ))
        .with_children(|container| {
            container
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(SLOT_GAP),
                    ..default()
                })
                .with_children(|slot_row| {
                    spawn_slot(
                        slot_row,
                        game_sprites,
                        game_fonts,
                        ForgeSlotIndex::Coal,
                        "Coal",
                        forge_state.and_then(|s| s.coal_slot),
                        registry,
                    );

                    spawn_slot(
                        slot_row,
                        game_sprites,
                        game_fonts,
                        ForgeSlotIndex::Ore,
                        "Ore",
                        forge_state.and_then(|s| s.ore_slot),
                        registry,
                    );

                    spawn_slot(
                        slot_row,
                        game_sprites,
                        game_fonts,
                        ForgeSlotIndex::Product,
                        "Ingot",
                        forge_state.and_then(|s| s.product_slot),
                        registry,
                    );
                });
        });
}

fn spawn_slot(
    parent: &mut ChildSpawnerCommands,
    game_sprites: &GameSprites,
    game_fonts: &GameFonts,
    slot_type: ForgeSlotIndex,
    label: &str,
    contents: Option<(ItemId, u32)>,
    registry: &ItemRegistry,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|slot_column| {
            let mut slot_entity = slot_column.spawn((
                ForgeSlotCell { slot_type },
                Node {
                    width: Val::Px(SLOT_SIZE),
                    height: Val::Px(SLOT_SIZE),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Relative,
                    ..default()
                },
            ));

            if let Some(cell_img) = game_sprites
                .get(SpriteSheetKey::GridSlot)
                .and_then(|s| s.image_node(GridSlotSlice::Slot.as_str()))
            {
                slot_entity.insert(cell_img);
            }

            slot_entity.with_children(|cell| {
                if let Some((item_id, quantity)) = contents {
                    spawn_slot_item(cell, game_sprites, game_fonts, item_id, quantity, registry);
                }
            });

            slot_column.spawn((
                Text::new(label),
                game_fonts.pixel_font(LABEL_FONT_SIZE),
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

pub fn spawn_slot_item(
    cell: &mut ChildSpawnerCommands,
    game_sprites: &GameSprites,
    game_fonts: &GameFonts,
    item_id: ItemId,
    quantity: u32,
    registry: &ItemRegistry,
) {
    let item = registry.spawn(item_id);
    if let Some(icon_img) = game_sprites
        .get(item.sprite.sheet_key)
        .and_then(|s| s.image_node(&item.sprite.name))
    {
        cell.spawn((
            ForgeSlotItemSprite,
            Node {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..default()
            },
            icon_img,
        ));

        if quantity > 1 {
            spawn_outlined_quantity_text(
                cell,
                game_fonts,
                quantity,
                OutlinedQuantityConfig::default(),
                ForgeSlotQuantityText,
            );
        }
    }
}
