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
    item::ItemId,
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
    let max_upgrades = blacksmith.max_upgrades;
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
            let upgrade_cost = blacksmith.calc_upgrade_cost(item);
            let at_max = item.num_upgrades >= max_upgrades;
            let can_afford = player_gold >= upgrade_cost;

            let line = if at_max {
                Line::from(vec![
                    selection_prefix(is_selected),
                    lock_prefix(item),
                    item_display(item, None),
                    Span::styled(" - MAX", Style::default().fg(colors::DARK_GRAY)),
                ])
            } else {
                let cost_style = if can_afford {
                    Style::default()
                } else {
                    Style::default().fg(colors::RED)
                };

                Line::from(vec![
                    selection_prefix(is_selected),
                    lock_prefix(item),
                    item_display(item, None),
                    Span::raw(" - "),
                    Span::styled(format!("{} gold", upgrade_cost), cost_style),
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
                let _ = gs.town.blacksmith.upgrade_player_item(&mut gs.player, inv_item.item.item_uuid);
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
