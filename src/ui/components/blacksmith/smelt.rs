use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::ListState,
    Frame,
};
use tuirealm::command::{Cmd, CmdResult};

use crate::{
    combat::HasGold,
    inventory::HasInventory,
    item::{ItemId, recipe::RecipeId},
    system::game_state,
};
use crate::ui::components::utilities::{
    list_move_down, list_move_up, render_location_header, selection_prefix,
    COIN, FIRE, RETURN_ARROW,
};
use crate::ui::theme as colors;

use super::forge_art::render_forge_art;
use super::StateChange;

pub fn render(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let gs = game_state();
    let player_gold = gs.player.gold();
    let blacksmith = gs.blacksmith();
    let fuel = blacksmith.fuel_amount;
    let coal_count = gs.player.find_item_by_id(ItemId::Coal)
        .map(|inv| inv.quantity).unwrap_or(0);

    let header_lines = vec![
        Line::from(vec![
            Span::styled(blacksmith.name.to_string(), Style::default().fg(colors::ORANGE)),
        ]),
        Line::from(vec![
            Span::styled(format!("{} ", COIN), Style::default().fg(colors::YELLOW)),
            Span::styled(format!("{}", player_gold), Style::default().fg(colors::WHITE)),
            Span::styled("  |  ", Style::default().fg(colors::WHITE)),
            Span::styled(format!("{} ", FIRE), Style::default().fg(colors::FLAME_ORANGE)),
            Span::styled(format!("Fuel: {}/100", fuel), Style::default().fg(colors::WHITE)),
            Span::styled("  |  ", Style::default().fg(colors::WHITE)),
            Span::styled("Coal: ", Style::default().fg(colors::DARK_GRAY)),
            Span::styled(format!("{}", coal_count), Style::default().fg(colors::WHITE)),
        ]),
    ];
    let content_area = render_location_header(frame, area, header_lines, colors::BLACKSMITH_BG, colors::DEEP_ORANGE);

    // Content width based on widest element (menu items ~42 chars)
    const CONTENT_WIDTH: u16 = 42;

    // Calculate horizontal padding to center content block + 3 extra for better visual centering
    let h_padding = (content_area.width.saturating_sub(CONTENT_WIDTH) / 2) + 3;
    let padding_str = " ".repeat(h_padding as usize);

    // Vertical layout: fuel bar (1) + forge art (18) + menu (5) = 24 lines
    // Center vertically with 2:3 ratio (shifted up)
    const FUEL_BAR_HEIGHT: u16 = 1;
    const FORGE_HEIGHT: u16 = 18;
    const MENU_HEIGHT: u16 = 5;
    const TOTAL_HEIGHT: u16 = FUEL_BAR_HEIGHT + FORGE_HEIGHT + MENU_HEIGHT;

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(2),
            Constraint::Length(TOTAL_HEIGHT),
            Constraint::Fill(3),
        ])
        .split(content_area);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(FUEL_BAR_HEIGHT),
            Constraint::Length(FORGE_HEIGHT),
            Constraint::Length(MENU_HEIGHT),
        ])
        .split(vertical_chunks[1]);

    let text_style = Style::default().fg(colors::WHITE);

    // Fuel bar - render directly to buffer to preserve background
    let fuel_bar_width = 20;
    let filled = ((fuel as f32 / 100.0) * fuel_bar_width as f32) as usize;
    let empty = fuel_bar_width - filled;
    let fuel_bar = Line::from(vec![
        Span::raw(padding_str.clone()),
        Span::styled("Fuel: [", text_style),
        Span::styled("█".repeat(filled), Style::default().fg(colors::FLAME_ORANGE)),
        Span::styled("░".repeat(empty), Style::default().fg(colors::DARK_GRAY)),
        Span::styled(format!("] {}/100", fuel), text_style),
    ]);

    let fuel_area = inner_chunks[0];
    let buf = frame.buffer_mut();
    let y = fuel_area.y;
    let mut x = fuel_area.x;
    for span in fuel_bar.spans.iter() {
        let has_style = span.style.fg.is_some() || span.style.bg.is_some();
        for ch in span.content.chars() {
            if x < fuel_area.x + fuel_area.width {
                if ch == ' ' && !has_style {
                    x += 1;
                    continue;
                }
                let cell = buf.cell_mut((x, y)).unwrap();
                cell.set_char(ch);
                if let Some(fg) = span.style.fg {
                    cell.set_fg(fg);
                }
                if let Some(bg) = span.style.bg {
                    cell.set_bg(bg);
                }
                x += 1;
            }
        }
    }

    // Forge art - render directly to buffer to preserve background
    // Skip space characters in unstyled spans to let background show through
    let forge_lines = render_forge_art(h_padding as usize);
    let forge_area = inner_chunks[1];
    for (i, line) in forge_lines.iter().enumerate() {
        if i < forge_area.height as usize {
            let y = forge_area.y + i as u16;
            let mut x = forge_area.x;
            for span in line.spans.iter() {
                let has_style = span.style.fg.is_some() || span.style.bg.is_some();
                for ch in span.content.chars() {
                    if x < forge_area.x + forge_area.width {
                        // Skip spaces in unstyled spans to preserve background
                        if ch == ' ' && !has_style {
                            x += 1;
                            continue;
                        }
                        let cell = buf.cell_mut((x, y)).unwrap();
                        cell.set_char(ch);
                        if let Some(fg) = span.style.fg {
                            cell.set_fg(fg);
                        }
                        if let Some(bg) = span.style.bg {
                            cell.set_bg(bg);
                        }
                        x += 1;
                    }
                }
            }
        }
    }

    // Menu - below the forge art, rendered directly to buffer to preserve background
    let selected = list_state.selected().unwrap_or(0);
    let tin_ore = gs.player.find_item_by_id(ItemId::TinOre).map(|i| i.quantity).unwrap_or(0);
    let copper_ore = gs.player.find_item_by_id(ItemId::CopperOre).map(|i| i.quantity).unwrap_or(0);

    let menu_lines: Vec<Line> = vec![
        Line::from(vec![
            Span::raw(padding_str.clone()),
            selection_prefix(selected == 0),
            Span::styled("+", Style::default().fg(colors::GREEN)),
            Span::styled(format!(" Add Fuel (Coal: {})", coal_count), text_style),
        ]),
        Line::from(vec![
            Span::raw(padding_str.clone()),
            selection_prefix(selected == 1),
            Span::styled(format!("{}", FIRE), Style::default().fg(colors::BRIGHT_YELLOW)),
            Span::styled(format!(" Smelt Tin (Tin Ore: {})", tin_ore), text_style),
        ]),
        Line::from(vec![
            Span::raw(padding_str.clone()),
            selection_prefix(selected == 2),
            Span::styled(format!("{}", FIRE), Style::default().fg(colors::DEEP_ORANGE)),
            Span::styled(format!(" Smelt Copper (Copper Ore: {})", copper_ore), text_style),
        ]),
        Line::from(vec![
            Span::raw(padding_str.clone()),
            selection_prefix(selected == 3),
            Span::styled(format!("{}", FIRE), Style::default().fg(colors::EMBER_RED)),
            Span::styled(format!(" Smelt Bronze (Copper: {}, Tin: {})", copper_ore, tin_ore), text_style),
        ]),
        Line::from(vec![
            Span::raw(padding_str),
            selection_prefix(selected == 4),
            Span::styled(format!("{} Back", RETURN_ARROW), text_style),
        ]),
    ];

    // Render menu lines directly to buffer to preserve background
    let menu_area = inner_chunks[2];
    for (i, line) in menu_lines.iter().enumerate() {
        if i < menu_area.height as usize {
            let y = menu_area.y + i as u16;
            let mut x = menu_area.x;
            for span in line.spans.iter() {
                let has_style = span.style.fg.is_some() || span.style.bg.is_some();
                for ch in span.content.chars() {
                    if x < menu_area.x + menu_area.width {
                        // Skip spaces in unstyled spans to preserve background
                        if ch == ' ' && !has_style {
                            x += 1;
                            continue;
                        }
                        let cell = buf.cell_mut((x, y)).unwrap();
                        cell.set_char(ch);
                        if let Some(fg) = span.style.fg {
                            cell.set_fg(fg);
                        }
                        if let Some(bg) = span.style.bg {
                            cell.set_bg(bg);
                        }
                        x += 1;
                    }
                }
            }
        }
    }
}

pub fn handle(cmd: Cmd, list_state: &mut ListState) -> (CmdResult, Option<StateChange>) {
    const MENU_SIZE: usize = 5;

    match cmd {
        Cmd::Move(tuirealm::command::Direction::Up) => {
            list_move_up(list_state, MENU_SIZE);
            (CmdResult::Changed(tuirealm::State::None), None)
        }
        Cmd::Move(tuirealm::command::Direction::Down) => {
            list_move_down(list_state, MENU_SIZE);
            (CmdResult::Changed(tuirealm::State::None), None)
        }
        Cmd::Submit => {
            use crate::location::BlacksmithError;

            let selected = list_state.selected().unwrap_or(0);
            let gs = game_state();
            let result = match selected {
                0 => gs.town.blacksmith.add_fuel(&mut gs.player).map(|_| "Added fuel"),
                1 => gs.town.blacksmith.smelt_and_give(&mut gs.player, &RecipeId::TinIngot).map(|_| "Smelted Tin Ingot"),
                2 => gs.town.blacksmith.smelt_and_give(&mut gs.player, &RecipeId::CopperIngot).map(|_| "Smelted Copper Ingot"),
                3 => gs.town.blacksmith.smelt_and_give(&mut gs.player, &RecipeId::BronzeIngot).map(|_| "Smelted Bronze Ingot"),
                4 => return (CmdResult::Submit(tuirealm::State::None), Some(StateChange::ToMenu)),
                _ => return (CmdResult::None, None),
            };

            match result {
                Ok(msg) => gs.toasts.success(msg),
                Err(e) => {
                    let msg = match e {
                        BlacksmithError::NotEnoughFuel => "Not enough fuel",
                        BlacksmithError::NoFuel => "No fuel to add",
                        BlacksmithError::RecipeError(_) => "Missing ingredients",
                        BlacksmithError::InventoryFull => "Inventory is full",
                        _ => "Smelting failed",
                    };
                    gs.toasts.error(msg);
                }
            }
            (CmdResult::Submit(tuirealm::State::None), None)
        }
        Cmd::Cancel => {
            (CmdResult::Changed(tuirealm::State::None), Some(StateChange::ToMenu))
        }
        _ => (CmdResult::None, None),
    }
}
