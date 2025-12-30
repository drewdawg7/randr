use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::ui::{theme::{self as colors, upgrade_color, ColorExt}, utilities::HAMMER};

use crate::{item::Item, stats::HasStats};
use crate::ui::components::utilities::{CROSSED_SWORDS, SHIELD, CHECKED, UNCHECKED, COIN};

const PANEL_BG: Color = Color::Rgb(50, 55, 75);

pub fn render_item_details_with_price(
    frame: &mut Frame,
    area: Rect,
    item: Option<&Item>,
    price: Option<(i32, &str)>,
) {
    match item {
        Some(item) => {
            let mut lines = vec![];

            // Item name with upgrade color
            let name_color = upgrade_color(item.num_upgrades);
            lines.push(Line::from(Span::styled(
                format!("{} (+{})", item.name, item.num_upgrades),
                Style::default().color(name_color).bold(),
            )));

            // Attack and Defense with icons only
            let attack = item.attack();
            let defense = item.def();
            lines.push(Line::from(vec![
                Span::styled(format!("{} ", CROSSED_SWORDS), Style::default().color(colors::RED)),
                Span::styled(format!("{:<4}", attack), Style::default().color(colors::WHITE)),
                Span::styled(format!("{} ", SHIELD), Style::default().color(colors::BLUE)),
                Span::styled(format!("{}", defense), Style::default().color(colors::WHITE)),
            ]));

            // Upgrades and equipped status on same line
            let (equipped_icon, equipped_style) = if item.is_equipped {
                (CHECKED, Style::default().color(colors::GREEN))
            } else {
                (UNCHECKED, Style::default().color(colors::DARK_GRAY))
            };
            lines.push(Line::from(vec![
                Span::styled(format!("{} ", HAMMER), Style::default().color(colors::BLACK)),
                Span::styled(format!("{}/{}  ", item.num_upgrades, item.max_upgrades), Style::default().color(colors::WHITE)),
                Span::styled(format!("{}", equipped_icon), equipped_style),
            ]));

            // Price (if provided)
            if let Some((amount, label)) = price {
                lines.push(Line::from(vec![
                    Span::styled(format!("{} ", COIN), Style::default().color(colors::YELLOW)),
                    Span::styled(format!("{} ", label), Style::default().color(colors::WHITE)),
                    Span::styled(format!("{}g", amount), Style::default().color(colors::YELLOW)),
                ]));
            }

            // Create compact box that fits content
            let content_height = lines.len() as u16;
            let content_width = 18u16;
            let box_area = Rect::new(
                area.x,
                area.y,
                content_width.min(area.width),
                content_height.min(area.height),
            );

            let block = Block::default().style(Style::default().bg(PANEL_BG));
            let paragraph = Paragraph::new(lines).block(block);
            frame.render_widget(paragraph, box_area);
        }
        None => {}
    }
}

/// Renders an item details panel showing stats for the given item (without price).
pub fn render_item_details(frame: &mut Frame, area: Rect, item: Option<&Item>) {
    render_item_details_with_price(frame, area, item, None);
}
