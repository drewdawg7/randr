//#![warn(clippy::single_match)]
use ratatui::{
    layout::Rect,
    style::Color,
    Frame,
};

use crate::ui::components::widgets::scroll_border::{render_scroll_with_styled_content, StyledContent};
use crate::ui::components::utilities::{CROSSED_SWORDS, SHIELD, COIN, PICKAXE};
use crate::ui::utilities::HAMMER;
use crate::ui::theme::{quality_color, SOFT_GREEN, SOFT_RED};

use crate::{
    inventory::HasInventory,
    item::Item,
    stats::HasStats,
    system::game_state,
};

const ARROW_UP: char = '↑';
const ARROW_DOWN: char = '↓';

/// Gets the currently equipped item to compare against based on item type.
/// Returns None if the item itself is already equipped or if not equipment.
fn get_comparison_item(item: &Item) -> Option<Item> {
    if item.is_equipped {
        return None;
    }

    let slot = item.item_type.equipment_slot()?;
    game_state().player.get_equipped_item(slot).map(|inv| inv.item.clone())
}

/// Formats a stat comparison as colored text segments
/// Returns (base_text, comparison_segments) where comparison_segments may be empty
fn format_stat_with_comparison(
    icon: char,
    stat_name: &str,
    value: i32,
    compare_value: Option<i32>,
) -> StyledContent {
    let base_text = format!("{} {}: {}", icon, stat_name, value);

    match compare_value {
        Some(compare) => {
            let diff = value - compare;
            if diff == 0 {
                // No change, just show the base stat
                StyledContent::plain(base_text)
            } else {
                let (arrow, color): (char, Color) = if diff > 0 {
                    (ARROW_UP, SOFT_GREEN)
                } else {
                    (ARROW_DOWN, SOFT_RED)
                };
                let comparison_text = format!(" ({}{}", arrow, diff.abs());
                StyledContent::multi(vec![
                    (base_text, None),
                    (comparison_text, Some(color)),
                    (")".to_string(), Some(color)),
                ])
            }
        }
        None => StyledContent::plain(base_text),
    }
}

fn render_item_details_inner(
    frame: &mut Frame,
    area: Rect,
    item: Option<&Item>,
    compare_to: Option<&Item>,
    price: Option<(i32, &str)>,
) {
    match item {
        Some(item) => {
            let mut content_lines: Vec<StyledContent> = vec![];
            let color = quality_color(item.quality);
            let is_equipment = item.item_type.is_equipment();

            // Item name as header/title with quality color
            let equipped_prefix = if item.is_equipped { "(E) " } else { "" };
            let name_display = if is_equipment {
                format!("{}{} (+{})", equipped_prefix, item.name, item.num_upgrades)
            } else {
                format!("{}{}", equipped_prefix, item.name)
            };
            content_lines.push(StyledContent::colored(name_display, color));

            // Quality line below item name
            content_lines.push(StyledContent::colored(
                item.quality.display_name().to_string(),
                color,
            ));

            // Stats and upgrades only for equipment
            if is_equipment {
                let attack = item.attack();
                let defense = item.def();
                let gold_find = item.goldfind();
                let mining = item.mining();
                let compare_attack = compare_to.map(|c| c.attack());
                let compare_defense = compare_to.map(|c| c.def());
                let compare_gold_find = compare_to.map(|c| c.goldfind());
                let compare_mining = compare_to.map(|c| c.mining());

                // Only show stats that are non-zero (or have a non-zero comparison)
                if attack > 0 || compare_attack.map_or(false, |c| c > 0) {
                    content_lines.push(format_stat_with_comparison(CROSSED_SWORDS, "Attack", attack, compare_attack));
                }
                if defense > 0 || compare_defense.map_or(false, |c| c > 0) {
                    content_lines.push(format_stat_with_comparison(SHIELD, "Defense", defense, compare_defense));
                }
                if gold_find > 0 || compare_gold_find.map_or(false, |c| c > 0) {
                    content_lines.push(format_stat_with_comparison(COIN, "Gold Find", gold_find, compare_gold_find));
                }
                if mining > 0 || compare_mining.map_or(false, |c| c > 0) {
                    content_lines.push(format_stat_with_comparison(PICKAXE, "Mining", mining, compare_mining));
                }

                content_lines.push(StyledContent::plain(format!("{} Upgrades: {}/{}", HAMMER, item.num_upgrades, item.max_upgrades)));
            }

            // Price (if provided)
            if let Some((amount, label)) = price {
                content_lines.push(StyledContent::plain(format!("{} {}: {}g", COIN, label, amount)));
            }

            render_scroll_with_styled_content(frame, area, content_lines);
        }
        None => {}
    }
}

/// Renders an item details panel showing stats for the given item.
/// Automatically compares to the currently equipped item of the same type.
pub fn render_item_details(frame: &mut Frame, area: Rect, item: Option<&Item>) {
    let compare_to = item.and_then(get_comparison_item);
    render_item_details_inner(frame, area, item, compare_to.as_ref(), None);
}

/// Renders item details panel to the right of list_area.
/// Only renders if game_state().show_item_details is true and no modal is blocking.
pub fn render_item_details_beside(frame: &mut Frame, list_area: Rect, item: Option<&Item>) {
    use crate::system::ModalType;

    let gs = game_state();
    if !gs.show_item_details {
        return;
    }

    // Don't render if inventory modal is active (it will render its own)
    if gs.active_modal == ModalType::Inventory {
        return;
    }

    render_item_details_panel(frame, list_area, item);
}

/// Renders item details for the inventory modal (always renders if show_item_details is true).
pub fn render_item_details_for_modal(frame: &mut Frame, list_area: Rect, item: Option<&Item>) {
    if !game_state().show_item_details {
        return;
    }

    render_item_details_panel(frame, list_area, item);
}

/// Shared rendering logic for item details panel.
fn render_item_details_panel(frame: &mut Frame, list_area: Rect, item: Option<&Item>) {
    if let Some(item) = item {
        let gap = 2u16;

        // Position based on where item text typically ends (~35 chars),
        // not the full list_area width which may be much larger
        let item_content_width = 35u16;
        let x = list_area.x + item_content_width + gap;
        let y = list_area.y;
        let details_height = list_area.height;

        let details_area = Rect::new(x, y, 40, details_height);
        render_item_details(frame, details_area, Some(item));
    }
}
