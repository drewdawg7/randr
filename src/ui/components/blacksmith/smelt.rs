use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
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
            Span::raw(format!("{}", player_gold)),
            Span::raw("  |  "),
            Span::styled(format!("{} ", FIRE), Style::default().fg(colors::FLAME_ORANGE)),
            Span::raw(format!("Fuel: {}/100", fuel)),
            Span::raw("  |  "),
            Span::styled("Coal: ", Style::default().fg(colors::DARK_GRAY)),
            Span::raw(format!("{}", coal_count)),
        ]),
    ];
    let content_area = render_location_header(frame, area, header_lines, colors::BLACKSMITH_BG, colors::DEEP_ORANGE);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(20),
            Constraint::Min(6),
        ])
        .split(content_area);

    let forge_width = 42u16;
    let h_padding = content_area.width.saturating_sub(forge_width) / 2;

    // Fuel bar
    let fuel_bar_width = 20;
    let filled = ((fuel as f32 / 100.0) * fuel_bar_width as f32) as usize;
    let empty = fuel_bar_width - filled;
    let fuel_bar = Line::from(vec![
        Span::raw(" ".repeat(h_padding as usize)),
        Span::raw("Fuel: ["),
        Span::styled("█".repeat(filled), Style::default().fg(colors::FLAME_ORANGE)),
        Span::styled("░".repeat(empty), Style::default().fg(colors::DARK_GRAY)),
        Span::raw(format!("] {}/100", fuel)),
    ]);
    frame.render_widget(Paragraph::new(fuel_bar), chunks[0]);

    // Forge art
    let forge_lines = render_forge_art(h_padding as usize);
    frame.render_widget(Paragraph::new(forge_lines), chunks[1]);

    // Menu
    let selected = list_state.selected().unwrap_or(0);
    let tin_ore = gs.player.find_item_by_id(ItemId::TinOre).map(|i| i.quantity).unwrap_or(0);
    let copper_ore = gs.player.find_item_by_id(ItemId::CopperOre).map(|i| i.quantity).unwrap_or(0);
    let menu_padding = " ".repeat(h_padding as usize);

    let menu_items: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            Span::raw(menu_padding.clone()),
            selection_prefix(selected == 0),
            Span::styled("+", Style::default().fg(colors::GREEN)),
            Span::raw(format!(" Add Fuel (Coal: {})", coal_count)),
        ])),
        ListItem::new(Line::from(vec![
            Span::raw(menu_padding.clone()),
            selection_prefix(selected == 1),
            Span::styled(format!("{}", FIRE), Style::default().fg(colors::BRIGHT_YELLOW)),
            Span::raw(format!(" Smelt Tin (Tin Ore: {})", tin_ore)),
        ])),
        ListItem::new(Line::from(vec![
            Span::raw(menu_padding.clone()),
            selection_prefix(selected == 2),
            Span::styled(format!("{}", FIRE), Style::default().fg(colors::DEEP_ORANGE)),
            Span::raw(format!(" Smelt Copper (Copper Ore: {})", copper_ore)),
        ])),
        ListItem::new(Line::from(vec![
            Span::raw(menu_padding.clone()),
            selection_prefix(selected == 3),
            Span::styled(format!("{}", FIRE), Style::default().fg(colors::EMBER_RED)),
            Span::raw(format!(" Smelt Bronze (Copper: {}, Tin: {})", copper_ore, tin_ore)),
        ])),
        ListItem::new(Line::from(vec![
            Span::raw(menu_padding),
            selection_prefix(selected == 4),
            Span::raw(format!("{} Back", RETURN_ARROW)),
        ])),
    ];

    frame.render_stateful_widget(List::new(menu_items), chunks[2], list_state);
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
            let selected = list_state.selected().unwrap_or(0);
            let gs = game_state();
            match selected {
                0 => { let _ = gs.town.blacksmith.add_fuel(&mut gs.player); }
                1 => { let _ = gs.town.blacksmith.smelt_and_give(&mut gs.player, &RecipeId::TinIngot); }
                2 => { let _ = gs.town.blacksmith.smelt_and_give(&mut gs.player, &RecipeId::CopperIngot); }
                3 => { let _ = gs.town.blacksmith.smelt_and_give(&mut gs.player, &RecipeId::BronzeIngot); }
                4 => return (CmdResult::Submit(tuirealm::State::None), Some(StateChange::ToMenu)),
                _ => {}
            }
            (CmdResult::Submit(tuirealm::State::None), None)
        }
        Cmd::Cancel => {
            (CmdResult::Changed(tuirealm::State::None), Some(StateChange::ToMenu))
        }
        _ => (CmdResult::None, None),
    }
}
