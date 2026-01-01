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
    ui::Id,
};
use crate::ui::components::utilities::{
    blacksmith_header, list_move_down, list_move_up, render_location_header,
    selection_prefix, CROSSED_SWORDS, DOUBLE_ARROW_UP, FIRE, RETURN_ARROW,
};
use crate::ui::utilities::HAMMER;
use crate::ui::theme as colors;

use super::StateChange;

pub fn render(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let player_gold = game_state().player.gold();
    let blacksmith = game_state().blacksmith();
    let stones = game_state().player.find_item_by_id(ItemId::QualityUpgradeStone)
        .map(|inv| inv.quantity).unwrap_or(0);

    let header_lines = blacksmith_header(blacksmith, player_gold, stones);
    let content_area = render_location_header(frame, area, header_lines, colors::BLACKSMITH_BG, colors::DEEP_ORANGE);

    // Render decorative stone wall patches in dead space (background layer)
    super::stone_wall_art::render_stone_patches(frame, content_area, 5);

    let selected = list_state.selected().unwrap_or(0);
    let menu_items: Vec<ListItem> = vec![
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 0),
            Span::raw(format!("{} Upgrade Items", HAMMER)),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 1),
            Span::raw(format!("{} Upgrade Item Quality", DOUBLE_ARROW_UP)),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 2),
            Span::styled(format!("{}", FIRE), Style::default().fg(colors::FLAME_ORANGE)),
            Span::raw(" Smelt Ores"),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 3),
            Span::styled(format!("{}", CROSSED_SWORDS), Style::default().fg(colors::LIGHT_STONE)),
            Span::raw(" Forge Items"),
        ])),
        ListItem::new(Line::from(vec![
            selection_prefix(selected == 4),
            Span::raw(format!("{} Back", RETURN_ARROW)),
        ])),
    ];

    let menu = List::new(menu_items);
    frame.render_stateful_widget(menu, content_area, list_state);
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
            let state_change = match selected {
                0 => Some(StateChange::ToUpgrade),
                1 => Some(StateChange::ToQuality),
                2 => Some(StateChange::ToSmelt),
                3 => Some(StateChange::ToForge),
                4 => {
                    game_state().current_screen = Id::Menu;
                    None
                }
                _ => None,
            };
            (CmdResult::Submit(tuirealm::State::None), state_change)
        }
        _ => (CmdResult::None, None),
    }
}
