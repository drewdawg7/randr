//! List navigation utilities.
//!
//! Pure functions for navigating list selections with wrapping behavior.

use ratatui::widgets::ListState;

/// Move selection up in a list with wrapping.
pub fn list_move_up(list_state: &mut ListState, item_count: usize) {
    if item_count == 0 {
        return;
    }
    let current = list_state.selected().unwrap_or(0);
    let new_idx = if current == 0 { item_count - 1 } else { current - 1 };
    list_state.select(Some(new_idx));
}

/// Move selection down in a list with wrapping.
pub fn list_move_down(list_state: &mut ListState, item_count: usize) {
    if item_count == 0 {
        return;
    }
    let current = list_state.selected().unwrap_or(0);
    let new_idx = if current >= item_count - 1 { 0 } else { current + 1 };
    list_state.select(Some(new_idx));
}
