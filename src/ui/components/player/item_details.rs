use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{item::{Item, ItemType}, stats::HasStats};
use crate::ui::components::utilities::{CROSSED_SWORDS, SHIELD, CHECKED, UNCHECKED, DOUBLE_ARROW_UP};

/// Renders an item details panel showing stats for the given item.
/// If no item is provided, renders an empty bordered box.
pub fn render_item_details(frame: &mut Frame, area: Rect, item: Option<&Item>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Item Details ");

    match item {
        Some(item) => {
            let mut lines = vec![
                Line::from(Span::styled(item.name, Style::default().bold())),
                Line::from(""),
            ];

            // Item type
            let type_str = match item.item_type {
                ItemType::Weapon => "Weapon",
                ItemType::Shield => "Shield",
            };
            lines.push(Line::from(vec![
                Span::raw("Type: "),
                Span::styled(type_str, Style::default().cyan()),
            ]));

            // Attack stat (show for all items, but highlight for weapons)
            let attack = item.attack();
            let attack_style = if item.item_type == ItemType::Weapon {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            lines.push(Line::from(vec![
                Span::styled(format!("{} ", CROSSED_SWORDS), Style::default().red()),
                Span::raw("Attack: "),
                Span::styled(format!("{}", attack), attack_style),
            ]));

            // Defense stat (show for all items, but highlight for shields)
            let defense = item.def();
            let defense_style = if item.item_type == ItemType::Shield {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            lines.push(Line::from(vec![
                Span::styled(format!("{} ", SHIELD), Style::default().blue()),
                Span::raw("Defense: "),
                Span::styled(format!("{}", defense), defense_style),
            ]));

            // Upgrades
            lines.push(Line::from(vec![
                Span::styled(format!("{} ", DOUBLE_ARROW_UP), Style::default().green()),
                Span::raw("Upgrades: "),
                Span::styled(
                    format!("{}/{}", item.num_upgrades, item.max_upgrades),
                    Style::default().fg(Color::White),
                ),
            ]));

            // Equipped status
            let (equipped_icon, equipped_text, equipped_style) = if item.is_equipped {
                (CHECKED, "Equipped", Style::default().fg(Color::Green))
            } else {
                (UNCHECKED, "Not Equipped", Style::default().fg(Color::DarkGray))
            };
            lines.push(Line::from(vec![
                Span::styled(format!("{} ", equipped_icon), equipped_style),
                Span::styled(equipped_text, equipped_style),
            ]));

            let paragraph = Paragraph::new(lines).block(block);
            frame.render_widget(paragraph, area);
        }
        None => {
            let paragraph = Paragraph::new("").block(block);
            frame.render_widget(paragraph, area);
        }
    }
}
