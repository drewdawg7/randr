use bevy::prelude::*;

use super::item_detail_pane::ItemDetailPane;
use super::item_grid::ItemGrid;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::InfoPanelSource;

pub trait DetailPaneContext: 'static + Send + Sync {
    type LeftGridMarker: Component;
    type RightGridMarker: Component;

    const LEFT_FOCUS: FocusPanel;
    const RIGHT_FOCUS: FocusPanel;

    fn source_from_left_grid(grid: &ItemGrid) -> InfoPanelSource;
    fn source_from_right_grid(grid: &ItemGrid) -> InfoPanelSource;
}

pub fn update_detail_pane_source<C: DetailPaneContext>(
    focus_state: Option<Res<FocusState>>,
    left_grids: Query<Ref<ItemGrid>, With<C::LeftGridMarker>>,
    right_grids: Query<Ref<ItemGrid>, With<C::RightGridMarker>>,
    mut panes: Query<&mut ItemDetailPane>,
) {
    let Some(focus_state) = focus_state else {
        return;
    };

    let focus_changed = focus_state.is_changed();
    let left_grid_changed = left_grids
        .get_single()
        .map(|g| g.is_changed())
        .unwrap_or(false);
    let right_grid_changed = right_grids
        .get_single()
        .map(|g| g.is_changed())
        .unwrap_or(false);

    if !focus_changed && !left_grid_changed && !right_grid_changed {
        return;
    }

    let source = if focus_state.is_focused(C::LEFT_FOCUS) {
        left_grids.get_single().ok().map(|g| C::source_from_left_grid(&g))
    } else if focus_state.is_focused(C::RIGHT_FOCUS) {
        right_grids.get_single().ok().map(|g| C::source_from_right_grid(&g))
    } else {
        None
    };

    let Some(source) = source else {
        return;
    };

    for mut pane in &mut panes {
        if pane.source != source {
            pane.source = source;
        }
    }
}
