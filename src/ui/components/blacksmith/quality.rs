use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
    Frame,
};
use tuirealm::command::{Cmd, CmdResult};

use crate::{
    combat::HasGold,
    inventory::HasInventory,
    item::{ItemId, enums::ItemQuality},
    system::game_state,
};
use crate::ui::components::player::item_details::render_item_details_beside;
use crate::ui::components::utilities::{
    blacksmith_header, collect_player_equipment, item_display, list_move_down,
    list_move_up, lock_prefix, render_location_header, selection_prefix, RETURN_ARROW,
};
use crate::ui::theme as colors;

use super::StateChange;

pub fn render(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let items = collect_player_equipment();
    let player_gold = game_state().player.gold();
    let blacksmith = game_state().blacksmith();
    let stones = game_state().player.find_item_by_id(ItemId::QualityUpgradeStone)
        .map(|inv| inv.quantity).unwrap_or(0);

    let header_lines = blacksmith_header(blacksmith, player_gold, stones);
    let content_area = render_location_header(frame, area, header_lines, colors::BLACKSMITH_BG, colors::DEEP_ORANGE);

    let selected = list_state.selected().unwrap_or(0);

    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, inv_item)| {
            let item = &inv_item.item;
            let is_selected = selected == i;
            let at_max = item.quality == ItemQuality::Mythic;
            let current_quality_color = colors::quality_color(item.quality);

            let line = if at_max {
                Line::from(vec![
                    selection_prefix(is_selected),
                    lock_prefix(item),
                    item_display(item, None),
                    Span::raw(" - "),
                    Span::styled(item.quality.display_name(), Style::default().fg(current_quality_color)),
                    Span::styled(" (MAX)", Style::default().fg(colors::DARK_GRAY)),
                ])
            } else {
                let next_quality = item.quality.next_quality().unwrap();
                let next_quality_color = colors::quality_color(next_quality);
                Line::from(vec![
                    selection_prefix(is_selected),
                    lock_prefix(item),
                    item_display(item, None),
                    Span::raw(" - "),
                    Span::styled(item.quality.display_name(), Style::default().fg(current_quality_color)),
                    Span::raw(" -> "),
                    Span::styled(next_quality.display_name(), Style::default().fg(next_quality_color)),
                ])
            };

            ListItem::new(line)
        })
        .collect();

    let back_selected = selected == items.len();
    let mut all_items = list_items;
    all_items.push(ListItem::new(Line::from(vec![
        selection_prefix(back_selected),
        Span::raw(format!("{} Back", RETURN_ARROW)),
    ])));

    let list = List::new(all_items);
    frame.render_stateful_widget(list, content_area, list_state);

    let selected_item = items.get(selected).map(|inv| &inv.item);
    render_item_details_beside(frame, content_area, selected_item);
}

pub fn handle(cmd: Cmd, list_state: &mut ListState) -> (CmdResult, Option<StateChange>) {
    let items = collect_player_equipment();
    let total_items = items.len() + 1;

    match cmd {
        Cmd::Move(tuirealm::command::Direction::Up) => {
            list_move_up(list_state, total_items);
            (CmdResult::Changed(tuirealm::State::None), None)
        }
        Cmd::Move(tuirealm::command::Direction::Down) => {
            list_move_down(list_state, total_items);
            (CmdResult::Changed(tuirealm::State::None), None)
        }
        Cmd::Submit => {
            let selected = list_state.selected().unwrap_or(0);
            if selected == items.len() {
                (CmdResult::Submit(tuirealm::State::None), Some(StateChange::ToMenu))
            } else if let Some(inv_item) = items.get(selected) {
                let gs = game_state();
                let _ = gs.town.blacksmith.upgrade_player_item_quality(&mut gs.player, inv_item.item.item_uuid);
                (CmdResult::Submit(tuirealm::State::None), None)
            } else {
                (CmdResult::None, None)
            }
        }
        Cmd::Cancel => {
            (CmdResult::Changed(tuirealm::State::None), Some(StateChange::ToMenu))
        }
        _ => (CmdResult::None, None),
    }
}
