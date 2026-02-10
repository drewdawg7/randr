mod cell;
mod components;
mod systems;

use bevy::prelude::*;

use crate::ui::focus::FocusState;
use systems::{on_add_item_grid, update_grid_items, update_grid_selector};

pub use components::{ItemGrid, ItemGridEntry, ItemGridFocusPanel, ItemGridSelection};

pub(super) const CELL_SIZE: f32 = 48.0;
pub(super) const GAP: f32 = 4.0;
pub(super) const NINE_SLICE_INSET: f32 = 58.0;

pub struct ItemGridPlugin;

impl Plugin for ItemGridPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_item_grid).add_systems(
            PostUpdate,
            (
                update_grid_items,
                update_grid_selector.run_if(
                    resource_exists::<FocusState>
                        .and(resource_changed::<FocusState>)
                        .or(any_match_filter::<Changed<ItemGridSelection>>),
                ),
            )
                .chain(),
        );
    }
}
