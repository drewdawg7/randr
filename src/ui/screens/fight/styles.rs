use bevy::prelude::*;

/// Fight screen action menu colors (brown theme with selection indicator).
pub const SELECTED_TEXT_COLOR: Color = Color::srgb(0.15, 0.1, 0.05);
pub const UNSELECTED_TEXT_COLOR: Color = Color::srgb(0.4, 0.35, 0.3);

/// Suffix appended to selected action items.
pub const SELECTED_SUFFIX: &str = " <";

/// Returns the appropriate action menu text color based on selection state.
pub fn action_text_color(selected: bool) -> Color {
    if selected {
        SELECTED_TEXT_COLOR
    } else {
        UNSELECTED_TEXT_COLOR
    }
}

/// Formats an action label with the selection suffix if selected.
pub fn action_label(label: &str, selected: bool) -> String {
    if selected {
        format!("{}{}", label, SELECTED_SUFFIX)
    } else {
        label.to_string()
    }
}
