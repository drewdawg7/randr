use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::ui::theme as colors;

/// Returns a small stone wall patch (~6 rows, ~15 chars wide)
pub fn small_stone_patch() -> Vec<Line<'static>> {
    let dark = Style::default().fg(colors::DARK_STONE);
    let mid = Style::default().fg(colors::GRANITE);
    let light = Style::default().fg(colors::LIGHT_STONE);

    vec![
        Line::from(vec![
            Span::styled("⠒⠂⠤⠀⠀", dark),
            Span::styled("⠂⠀⠀⠤⠀", mid),
            Span::styled("⠐⠚⠂⠒⢲", light),
        ]),
        Line::from(vec![
            Span::styled("⠒⠒⠒⠒⠒", mid),
            Span::styled("⡖⠒⠒⠓⠀", dark),
            Span::styled("⠀⣤⠄⠀⠘", light),
        ]),
        Line::from(vec![
            Span::styled("⠤⠤⡤⠤⠤", light),
            Span::styled("⠷⠤⢤⡤⠤", mid),
            Span::styled("⠤⠼⠤⠤⢬", dark),
        ]),
        Line::from(vec![
            Span::styled("⠤⠤⡧⡤⠤", dark),
            Span::styled("⡤⠠⠤⠧⠤", light),
            Span::styled("⠤⢤⠤⠤⠼", mid),
        ]),
        Line::from(vec![
            Span::styled("⣀⣠⣄⣱⣀", mid),
            Span::styled("⣇⣀⣀⣀⣀", dark),
            Span::styled("⣀⣼⣀⣀⣀", light),
        ]),
        Line::from(vec![
            Span::styled("⣀⣀⣅⣀⣀", light),
            Span::styled("⣀⣐⣀⣋⣀", mid),
            Span::styled("⣀⣀⣀⣀⣀", dark),
        ]),
    ]
}

/// Returns a medium stone wall patch (~8 rows, ~20 chars wide)
pub fn medium_stone_patch() -> Vec<Line<'static>> {
    let dark = Style::default().fg(colors::DARK_STONE);
    let mid = Style::default().fg(colors::GRANITE);
    let light = Style::default().fg(colors::LIGHT_STONE);

    vec![
        Line::from(vec![
            Span::styled("⠒⠂⠤⠀⠀⠂⠀", dark),
            Span::styled("⠀⠤⠀⠐⠚⠂⠒", mid),
            Span::styled("⢲⠒⠒⠒⠓⠒", light),
        ]),
        Line::from(vec![
            Span::styled("⠒⠒⠒⠒⠒⡖⠒", mid),
            Span::styled("⠒⠓⠀⠀⣤⠄⠀", light),
            Span::styled("⠘⠒⠒⢶⡖⠖", dark),
        ]),
        Line::from(vec![
            Span::styled("⠤⠤⡤⠤⠤⠷⠤", light),
            Span::styled("⢤⡤⠤⠤⠼⠤⠤", dark),
            Span::styled("⢬⡤⠮⠴⠃⠁", mid),
        ]),
        Line::from(vec![
            Span::styled("⠤⠤⡧⡤⠤⡤⠠", dark),
            Span::styled("⠤⠧⠤⠤⢤⠤⠤", mid),
            Span::styled("⠼⠧⠤⠤⡦⠤", light),
        ]),
        Line::from(vec![
            Span::styled("⣀⣠⣄⣱⣀⣇⣀", mid),
            Span::styled("⣀⣀⣀⣀⣼⣀⣀", light),
            Span::styled("⣀⣠⠀⢠⡇⠀", dark),
        ]),
        Line::from(vec![
            Span::styled("⣀⣀⣅⣀⣀⣀⣐", light),
            Span::styled("⣀⣋⣀⣀⣀⣀⣀", dark),
            Span::styled("⣀⣀⣇⣀⣀⣰", mid),
        ]),
        Line::from(vec![
            Span::styled("⠀⠀⢀⢀⢀⣏⠀", dark),
            Span::styled("⠀⠀⠀⢀⢸⢀⡀", mid),
            Span::styled("⢀⣄⠀⣀⣏⣰", light),
        ]),
        Line::from(vec![
            Span::styled("⠀⠘⡏⠁⠀⠀⠀", mid),
            Span::styled("⠀⢹⠉⠀⠀⠙⠈", light),
            Span::styled("⢙⡧⠀⢀⡀⠀", dark),
        ]),
    ]
}

/// Returns a large stone wall patch (~10 rows, ~30 chars wide)
pub fn large_stone_patch() -> Vec<Line<'static>> {
    let dark = Style::default().fg(colors::DARK_STONE);
    let mid = Style::default().fg(colors::GRANITE);
    let light = Style::default().fg(colors::LIGHT_STONE);

    vec![
        Line::from(vec![
            Span::styled("⠒⠂⠤⠀⠀⠂⠀⠀⠤⠀", dark),
            Span::styled("⠐⠚⠂⠒⢲⠒⠒⠒⠓⠒", mid),
            Span::styled("⠒⡖⠐⠒⠒⠀⠀⠠⠀⠀", light),
        ]),
        Line::from(vec![
            Span::styled("⠒⠒⠒⠒⠒⡖⠒⠒⠓⠀", mid),
            Span::styled("⠀⣤⠄⠀⠘⠒⠒⢶⡖⠖", light),
            Span::styled("⠖⠓⠒⠒⢶⠒⠒⠒⠒⠒", dark),
        ]),
        Line::from(vec![
            Span::styled("⠤⠤⡤⠤⠤⠷⠤⢤⡤⠤", light),
            Span::styled("⠤⠼⠤⠤⢬⡤⠮⠴⠃⠁", dark),
            Span::styled("⠰⡤⠄⠐⠚⠂⠐⠦⠄⠀", mid),
        ]),
        Line::from(vec![
            Span::styled("⠤⠤⡧⡤⠤⡤⠠⠤⠧⠤", dark),
            Span::styled("⠤⢤⠤⠤⠼⠧⠤⠤⡦⠤", mid),
            Span::styled("⠤⠳⠰⠰⠤⠤⠤⡐⠂⠀", light),
        ]),
        Line::from(vec![
            Span::styled("⣀⣠⣄⣱⣀⣇⣀⣀⣀⣀", mid),
            Span::styled("⣀⣼⣀⣀⣀⣠⠀⢠⡇⠀", light),
            Span::styled("⠠⣤⠢⠤⠬⠤⠤⢧⡄⠀", dark),
        ]),
        Line::from(vec![
            Span::styled("⣀⣀⣅⣀⣀⣀⣐⣀⣋⣀", light),
            Span::styled("⣀⣀⣀⣀⣀⣀⣇⣀⣀⣰", dark),
            Span::styled("⣀⣀⣷⣄⣀⣀⣀⠤⢼⠧", mid),
        ]),
        Line::from(vec![
            Span::styled("⠀⠀⢀⢀⢀⣏⠀⠀⠀⠀", dark),
            Span::styled("⢀⢸⢀⡀⢀⣄⠀⣀⣏⣰", mid),
            Span::styled("⣈⣀⡀⠀⣸⣀⠀⠀⡀⠀", light),
        ]),
        Line::from(vec![
            Span::styled("⠀⠘⡏⠁⠀⠀⠀⠀⢹⠉", mid),
            Span::styled("⠀⠀⠙⠈⢙⡧⠀⢀⡀⠀", light),
            Span::styled("⠀⢸⢀⣀⣀⣀⣀⣀⣁⠀", dark),
        ]),
        Line::from(vec![
            Span::styled("⠀⠈⠉⠉⠉⡟⠁⠋⠙⠁", light),
            Span::styled("⠉⢻⠙⠉⠛⠛⠉⠩⡏⠉", dark),
            Span::styled("⠉⠉⠭⠉⠉⡇⠀⠀⠀⠀", mid),
        ]),
        Line::from(vec![
            Span::styled("⠀⠐⡖⠒⠒⠛⠒⠒⢲⠒", dark),
            Span::styled("⠒⠚⠒⠒⠒⠶⠒⠒⠛⠂", mid),
            Span::styled("⠂⢰⠂⠚⠙⠓⠂⠐⡖⠐", light),
        ]),
    ]
}

/// Renders stone wall patches in the dead space of the menu area.
/// Should be called BEFORE rendering the menu list so menu appears on top.
pub fn render_stone_patches(frame: &mut Frame, area: Rect, menu_item_count: u16) {
    let dead_space_height = area.height.saturating_sub(menu_item_count);

    // Only render if we have enough space for at least a small patch
    if dead_space_height < 6 || area.width < 20 {
        return;
    }

    // Large patch in bottom-left
    if area.width >= 35 && dead_space_height >= 10 {
        let large_rect = Rect {
            x: area.x + 2,
            y: area.y + area.height.saturating_sub(10),
            width: 30,
            height: 10,
        };
        frame.render_widget(Paragraph::new(large_stone_patch()), large_rect);
    }

    // Medium patch in bottom-right
    if area.width >= 50 && dead_space_height >= 8 {
        let medium_rect = Rect {
            x: area.x + area.width.saturating_sub(22),
            y: area.y + area.height.saturating_sub(8),
            width: 20,
            height: 8,
        };
        frame.render_widget(Paragraph::new(medium_stone_patch()), medium_rect);
    }

    // Small patch in top-right area (above medium if it exists)
    if area.width >= 40 && dead_space_height >= 12 {
        let small_rect = Rect {
            x: area.x + area.width.saturating_sub(18),
            y: area.y + menu_item_count + 2,
            width: 15,
            height: 6,
        };
        frame.render_widget(Paragraph::new(small_stone_patch()), small_rect);
    }

    // Another small patch in center-bottom
    if area.width >= 55 && dead_space_height >= 10 {
        let center_x = area.x + (area.width / 2).saturating_sub(7);
        let small_rect = Rect {
            x: center_x,
            y: area.y + area.height.saturating_sub(7),
            width: 15,
            height: 6,
        };
        frame.render_widget(Paragraph::new(small_stone_patch()), small_rect);
    }
}
