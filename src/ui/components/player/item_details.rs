//#![warn(clippy::single_match)]
use ratatui::{
    layout::Rect,
    Frame,
};

use crate::ui::components::widgets::scroll_border::{render_scroll_with_styled_content, StyledContent};
use crate::ui::components::utilities::{CROSSED_SWORDS, SHIELD, COIN};
use crate::ui::utilities::HAMMER;
use crate::ui::theme::quality_color;

use crate::{item::Item, stats::HasStats};

pub fn render_item_details_with_price(
    frame: &mut Frame,
    area: Rect,
    item: Option<&Item>,
    price: Option<(i32, &str)>,
) {
    match item {

        Some(item) => {
            let mut content_lines: Vec<StyledContent> = vec![];
            let color = quality_color(item.quality);

            // Item name as header/title with quality color
            let equipped_prefix = if item.is_equipped { "(E) " } else { "" };
            content_lines.push(StyledContent::colored(
                format!("{}{} (+{})", equipped_prefix, item.name, item.num_upgrades),
                color,
            ));

            // Quality line below item name
            content_lines.push(StyledContent::colored(
                item.quality.display_name().to_string(),
                color,
            ));

            // Stats displayed vertically
            let attack = item.attack();
            let defense = item.def();

            content_lines.push(StyledContent::plain(format!("{} Attack: {}", CROSSED_SWORDS, attack)));
            content_lines.push(StyledContent::plain(format!("{} Defense: {}", SHIELD, defense)));
            content_lines.push(StyledContent::plain(format!("{} Upgrades: {}/{}", HAMMER, item.num_upgrades, item.max_upgrades)));

            // Price (if provided)
            if let Some((amount, label)) = price {
                content_lines.push(StyledContent::plain(format!("{} {}: {}g", COIN, label, amount)));
            }

            render_scroll_with_styled_content(frame, area, content_lines);
        }
        None => {}
    }
}

/// Renders an item details panel showing stats for the given item (without price).
pub fn render_item_details(frame: &mut Frame, area: Rect, item: Option<&Item>) {
    render_item_details_with_price(frame, area, item, None);
}
