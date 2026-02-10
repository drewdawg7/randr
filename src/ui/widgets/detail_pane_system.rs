use bevy::prelude::*;

use super::item_detail_pane::ItemDetailPane;
use super::item_grid::ItemGridSelection;
use crate::ui::focus::{FocusPanel, FocusState};
use crate::ui::InfoPanelSource;

pub trait DetailPaneContext: 'static + Send + Sync {
    type LeftGridMarker: Component;
    type RightGridMarker: Component;

    const LEFT_FOCUS: FocusPanel;
    const RIGHT_FOCUS: FocusPanel;

    fn source_from_left_grid(selection: &ItemGridSelection) -> InfoPanelSource;
    fn source_from_right_grid(selection: &ItemGridSelection) -> InfoPanelSource;
}

pub fn update_detail_pane_source<C: DetailPaneContext>(
    focus_state: Option<Res<FocusState>>,
    left_grids: Query<Ref<ItemGridSelection>, With<C::LeftGridMarker>>,
    right_grids: Query<Ref<ItemGridSelection>, With<C::RightGridMarker>>,
    mut panes: Query<&mut ItemDetailPane>,
) {
    let Some(focus_state) = focus_state else {
        return;
    };

    let source = if focus_state.is_focused(C::LEFT_FOCUS) {
        left_grids
            .single()
            .ok()
            .map(|s| C::source_from_left_grid(&s))
    } else if focus_state.is_focused(C::RIGHT_FOCUS) {
        right_grids
            .single()
            .ok()
            .map(|s| C::source_from_right_grid(&s))
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
