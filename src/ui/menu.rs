use bevy::prelude::*;

use super::theme::nav_selection_text;

/// Component that stores a menu item's index for selection tracking.
/// Add this alongside a marker component (like `ActionMenuItem`) for filtering.
#[derive(Component)]
pub struct MenuIndex(pub usize);

/// Updates the TextColor of menu items based on selection state.
/// Uses nav theme colors (white for selected, gray for unselected).
///
/// # Type Parameters
/// * `M` - Marker component to filter which menu items to update
///
/// # Example
/// ```ignore
/// update_menu_colors::<ActionMenuItem>(selected_index, &mut items);
/// ```
pub fn update_menu_colors<M: Component>(
    selected_index: usize,
    items: &mut Query<(&MenuIndex, &mut TextColor), With<M>>,
) {
    for (menu_index, mut color) in items.iter_mut() {
        *color = TextColor(nav_selection_text(menu_index.0 == selected_index));
    }
}
