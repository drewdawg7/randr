//! Pure styling functions for UI elements.
//!
//! These functions create styled Spans for list items, items, and other UI elements.
//! They are pure functions with no side effects or game state access.

use ratatui::{
    style::Style,
    text::Span,
};

use crate::item::Item;
use crate::ui::theme::{self as colors, icons, quality_color, ColorExt};

/// Returns a styled prefix Span for list items. Selected items get a yellow ">", unselected get "  ".
pub fn selection_prefix(is_selected: bool) -> Span<'static> {
    if is_selected {
        Span::styled("> ", Style::default().color(colors::YELLOW))
    } else {
        Span::raw("  ")
    }
}

/// Returns a lock icon Span if the item is locked, otherwise an empty span.
pub fn lock_prefix(item: &Item) -> Span<'static> {
    if item.is_locked {
        Span::styled(format!("{} ", icons::LOCK), Style::default().color(colors::BRONZE))
    } else {
        Span::raw("")
    }
}

/// Returns an equip icon Span if the item is equipped, otherwise an empty span.
pub fn equip_prefix(item: &Item) -> Span<'static> {
    if item.is_equipped {
        Span::styled(format!("{} ", icons::SHIRT), Style::default().color(colors::DARK_GRAY))
    } else {
        Span::raw("")
    }
}

/// Returns a styled Span for an item, colored by quality.
/// - Equipment: "{name} (+{num_upgrades})"
/// - Materials: "{name} (x{quantity})"
pub fn item_display(item: &Item, quantity: Option<u32>) -> Span<'static> {
    let color = quality_color(item.quality);
    let text = if item.item_type.is_equipment() {
        format!("{} (+{})", item.name, item.num_upgrades)
    } else {
        match quantity {
            Some(q) => format!("{} (x{})", item.name, q),
            None => item.name.to_string(),
        }
    };
    Span::styled(text, Style::default().color(color))
}
