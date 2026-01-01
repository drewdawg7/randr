use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::ui::theme as colors;

/// Returns a small stone wall patch (~4 rows, ~12 chars wide)
/// Uses sparser braille for texture variation
pub fn small_patch_a() -> Vec<Line<'static>> {
    let dark = Style::default().fg(colors::DARK_STONE);
    let mid = Style::default().fg(colors::GRANITE);
    let light = Style::default().fg(colors::LIGHT_STONE);

    vec![
        Line::from(vec![
            Span::styled("⠂", dark),
            Span::styled("⠀⠤", mid),
            Span::styled("⠀⠐", light),
            Span::styled("⠂⠒", mid),
            Span::styled("⠒⠀", dark),
            Span::styled("⠠⠀", light),
        ]),
        Line::from(vec![
            Span::styled("⠒⠒", mid),
            Span::styled("⡖⠒", light),
            Span::styled("⠓⠀", dark),
            Span::styled("⠀⣤", mid),
            Span::styled("⠄⠀", light),
            Span::styled("⠘⠒", dark),
        ]),
        Line::from(vec![
            Span::styled("⠤⡤", light),
            Span::styled("⠤⠷", dark),
            Span::styled("⠤⢤", mid),
            Span::styled("⡤⠤", light),
            Span::styled("⠼⠤", dark),
            Span::styled("⠤⢬", mid),
        ]),
        Line::from(vec![
            Span::styled("⡧⡤", dark),
            Span::styled("⠤⡤", mid),
            Span::styled("⠠⠤", light),
            Span::styled("⠧⠤", dark),
            Span::styled("⢤⠤", mid),
            Span::styled("⠤⠼", light),
        ]),
    ]
}

/// Returns a small stone wall patch variant B
pub fn small_patch_b() -> Vec<Line<'static>> {
    let dark = Style::default().fg(colors::DARK_STONE);
    let mid = Style::default().fg(colors::GRANITE);
    let light = Style::default().fg(colors::LIGHT_STONE);

    vec![
        Line::from(vec![
            Span::styled("⣀⣠", mid),
            Span::styled("⣄⣱", light),
            Span::styled("⣀⣇", dark),
            Span::styled("⣀⣀", mid),
            Span::styled("⣀⣀", light),
        ]),
        Line::from(vec![
            Span::styled("⣀⣅", light),
            Span::styled("⣀⣀", dark),
            Span::styled("⣐⣀", mid),
            Span::styled("⣋⣀", light),
            Span::styled("⣀⣀", dark),
        ]),
        Line::from(vec![
            Span::styled("⠀⢀", dark),
            Span::styled("⢀⣏", mid),
            Span::styled("⠀⠀", light),
            Span::styled("⢀⢸", dark),
            Span::styled("⢀⡀", mid),
        ]),
        Line::from(vec![
            Span::styled("⠘⡏", mid),
            Span::styled("⠁⠀", light),
            Span::styled("⠀⢹", dark),
            Span::styled("⠉⠀", mid),
            Span::styled("⠙⠈", light),
        ]),
    ]
}

/// Returns a small stone wall patch variant C - very sparse
pub fn small_patch_c() -> Vec<Line<'static>> {
    let dark = Style::default().fg(colors::DARK_STONE);
    let mid = Style::default().fg(colors::GRANITE);
    let light = Style::default().fg(colors::LIGHT_STONE);

    vec![
        Line::from(vec![
            Span::styled("⠈⠉", light),
            Span::styled("⠉⡟", mid),
            Span::styled("⠁⠋", dark),
            Span::styled("⠙⠁", light),
            Span::styled("⠉⢻", mid),
        ]),
        Line::from(vec![
            Span::styled("⠙⠉", dark),
            Span::styled("⠛⠛", light),
            Span::styled("⠉⠩", mid),
            Span::styled("⡏⠉", dark),
            Span::styled("⠉⠭", light),
        ]),
        Line::from(vec![
            Span::styled("⠐⡖", mid),
            Span::styled("⠒⠛", dark),
            Span::styled("⠒⢲", light),
            Span::styled("⠒⠚", mid),
            Span::styled("⠒⠒", dark),
        ]),
    ]
}

/// Returns a medium stone wall patch (~6 rows, ~18 chars wide)
pub fn medium_patch_a() -> Vec<Line<'static>> {
    let dark = Style::default().fg(colors::DARK_STONE);
    let mid = Style::default().fg(colors::GRANITE);
    let light = Style::default().fg(colors::LIGHT_STONE);

    vec![
        Line::from(vec![
            Span::styled("⠒⠂", dark),
            Span::styled("⠤⠀", mid),
            Span::styled("⠀⠂", light),
            Span::styled("⠀⠀", dark),
            Span::styled("⠤⠀", mid),
            Span::styled("⠐⠚", light),
            Span::styled("⠂⠒", dark),
            Span::styled("⢲⠒", mid),
            Span::styled("⠒⠓", light),
        ]),
        Line::from(vec![
            Span::styled("⠒⡖", mid),
            Span::styled("⠒⠓", light),
            Span::styled("⠀⠀", dark),
            Span::styled("⣤⠄", mid),
            Span::styled("⠀⠘", light),
            Span::styled("⠒⠒", dark),
            Span::styled("⢶⡖", mid),
            Span::styled("⠖⠖", light),
            Span::styled("⠓⠒", dark),
        ]),
        Line::from(vec![
            Span::styled("⠤⡤", light),
            Span::styled("⠤⠷", dark),
            Span::styled("⠤⢤", mid),
            Span::styled("⡤⠤", light),
            Span::styled("⠤⠼", dark),
            Span::styled("⠤⠤", mid),
            Span::styled("⢬⡤", light),
            Span::styled("⠮⠴", dark),
            Span::styled("⠃⠁", mid),
        ]),
        Line::from(vec![
            Span::styled("⠤⡧", dark),
            Span::styled("⡤⠤", mid),
            Span::styled("⡤⠠", light),
            Span::styled("⠤⠧", dark),
            Span::styled("⠤⠤", mid),
            Span::styled("⢤⠤", light),
            Span::styled("⠤⠼", dark),
            Span::styled("⠧⠤", mid),
            Span::styled("⠤⡦", light),
        ]),
        Line::from(vec![
            Span::styled("⣀⣠", mid),
            Span::styled("⣄⣱", light),
            Span::styled("⣀⣇", dark),
            Span::styled("⣀⣀", mid),
            Span::styled("⣀⣀", light),
            Span::styled("⣀⣼", dark),
            Span::styled("⣀⣀", mid),
            Span::styled("⣀⣠", light),
            Span::styled("⠀⢠", dark),
        ]),
        Line::from(vec![
            Span::styled("⣀⣅", light),
            Span::styled("⣀⣀", dark),
            Span::styled("⣀⣐", mid),
            Span::styled("⣀⣋", light),
            Span::styled("⣀⣀", dark),
            Span::styled("⣀⣀", mid),
            Span::styled("⣀⣀", light),
            Span::styled("⣇⣀", dark),
            Span::styled("⣀⣰", mid),
        ]),
    ]
}

/// Returns a medium stone wall patch variant B
pub fn medium_patch_b() -> Vec<Line<'static>> {
    let dark = Style::default().fg(colors::DARK_STONE);
    let mid = Style::default().fg(colors::GRANITE);
    let light = Style::default().fg(colors::LIGHT_STONE);

    vec![
        Line::from(vec![
            Span::styled("⣀⣀", mid),
            Span::styled("⣷⣄", light),
            Span::styled("⣀⣀", dark),
            Span::styled("⣀⠤", mid),
            Span::styled("⢼⠧", light),
            Span::styled("⠤⠤", dark),
        ]),
        Line::from(vec![
            Span::styled("⠀⢀", dark),
            Span::styled("⢀⢀", mid),
            Span::styled("⣏⠀", light),
            Span::styled("⠀⠀", dark),
            Span::styled("⢀⢸", mid),
            Span::styled("⢀⡀", light),
        ]),
        Line::from(vec![
            Span::styled("⢀⣄", mid),
            Span::styled("⠀⣀", light),
            Span::styled("⣏⣰", dark),
            Span::styled("⣈⣀", mid),
            Span::styled("⡀⠀", light),
            Span::styled("⣸⣀", dark),
        ]),
        Line::from(vec![
            Span::styled("⠘⡏", light),
            Span::styled("⠁⠀", dark),
            Span::styled("⠀⠀", mid),
            Span::styled("⢹⠉", light),
            Span::styled("⠀⠀", dark),
            Span::styled("⠙⠈", mid),
        ]),
        Line::from(vec![
            Span::styled("⢙⡧", dark),
            Span::styled("⠀⢀", mid),
            Span::styled("⡀⠀", light),
            Span::styled("⠀⢸", dark),
            Span::styled("⢀⣀", mid),
            Span::styled("⣀⣀", light),
        ]),
    ]
}

/// Renders stone wall patches in the dead space of the menu area.
/// Should be called BEFORE rendering the menu list so menu appears on top.
pub fn render_stone_patches(frame: &mut Frame, area: Rect, menu_item_count: u16) {
    let dead_space_height = area.height.saturating_sub(menu_item_count);

    // Only render if we have enough space for patches
    if dead_space_height < 4 || area.width < 15 {
        return;
    }

    // Bottom-left corner - medium patch
    if area.width >= 20 && dead_space_height >= 6 {
        let rect = Rect {
            x: area.x + 1,
            y: area.y + area.height.saturating_sub(6),
            width: 18,
            height: 6,
        };
        frame.render_widget(Paragraph::new(medium_patch_a()), rect);
    }

    // Bottom-right corner - medium patch variant
    if area.width >= 40 && dead_space_height >= 5 {
        let rect = Rect {
            x: area.x + area.width.saturating_sub(14),
            y: area.y + area.height.saturating_sub(5),
            width: 12,
            height: 5,
        };
        frame.render_widget(Paragraph::new(medium_patch_b()), rect);
    }

    // Top-right of dead space - small patch
    if area.width >= 30 && dead_space_height >= 8 {
        let rect = Rect {
            x: area.x + area.width.saturating_sub(14),
            y: area.y + menu_item_count + 1,
            width: 12,
            height: 4,
        };
        frame.render_widget(Paragraph::new(small_patch_a()), rect);
    }

    // Center-bottom area - small patch B
    if area.width >= 45 && dead_space_height >= 6 {
        let center_x = area.x + (area.width / 2).saturating_sub(5);
        let rect = Rect {
            x: center_x,
            y: area.y + area.height.saturating_sub(5),
            width: 10,
            height: 4,
        };
        frame.render_widget(Paragraph::new(small_patch_b()), rect);
    }

    // Left side mid-height - small patch C (sparse)
    if area.width >= 25 && dead_space_height >= 8 {
        let rect = Rect {
            x: area.x + 2,
            y: area.y + menu_item_count + 2,
            width: 10,
            height: 3,
        };
        frame.render_widget(Paragraph::new(small_patch_c()), rect);
    }

    // Upper-center - small patch A variant
    if area.width >= 50 && dead_space_height >= 10 {
        let rect = Rect {
            x: area.x + (area.width / 2).saturating_sub(8),
            y: area.y + menu_item_count + 1,
            width: 12,
            height: 4,
        };
        frame.render_widget(Paragraph::new(small_patch_a()), rect);
    }

    // Right side mid-area - small patch B
    if area.width >= 55 && dead_space_height >= 10 {
        let rect = Rect {
            x: area.x + area.width.saturating_sub(12),
            y: area.y + menu_item_count + 5,
            width: 10,
            height: 4,
        };
        frame.render_widget(Paragraph::new(small_patch_b()), rect);
    }

    // Left side lower - small patch C
    if area.width >= 35 && dead_space_height >= 10 {
        let rect = Rect {
            x: area.x + 3,
            y: area.y + menu_item_count + 5,
            width: 10,
            height: 3,
        };
        frame.render_widget(Paragraph::new(small_patch_c()), rect);
    }

    // Center cluster - additional small patch
    if area.width >= 60 && dead_space_height >= 12 {
        let rect = Rect {
            x: area.x + (area.width / 2) + 3,
            y: area.y + menu_item_count + 3,
            width: 10,
            height: 4,
        };
        frame.render_widget(Paragraph::new(small_patch_b()), rect);
    }
}
