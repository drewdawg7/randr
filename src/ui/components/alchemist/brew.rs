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
    commands::{apply_result, execute, GameCommand},
    inventory::HasInventory,
    item::recipe::{Recipe, RecipeId},
    system::game_state,
};
use crate::ui::components::utilities::{
    list_move_down, list_move_up, render_location_header, selection_prefix,
    COIN,
};
use crate::ui::theme as colors;

use super::flask_art::render_flask_art;
use super::StateChange;

pub fn render(frame: &mut Frame, area: Rect, list_state: &mut ListState) {
    let gs = game_state();
    let player_gold = gs.player.gold();
    let alchemist = gs.alchemist();

    let header_lines = vec![
        Line::from(vec![
            Span::styled(alchemist.name.to_string(), Style::default().fg(colors::BRIGHT_VIOLET)),
        ]),
        Line::from(vec![
            Span::styled(format!("{} ", COIN), Style::default().fg(colors::YELLOW)),
            Span::raw(format!("{}", player_gold)),
        ]),
    ];
    let content_area = render_location_header(
        frame, area, header_lines, colors::ALCHEMIST_BG, colors::MYSTIC_PURPLE
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(14),  // Flask art
            Constraint::Min(6),      // Recipe list
        ])
        .split(content_area);

    let flask_width = 28u16;
    let h_padding = content_area.width.saturating_sub(flask_width) / 2;

    // Flask art
    let flask_lines = render_flask_art(h_padding as usize);
    frame.render_widget(Paragraph::new(flask_lines), chunks[0]);

    // Recipe list
    let selected = list_state.selected().unwrap_or(0);
    let menu_padding = " ".repeat(h_padding as usize);
    let recipes = RecipeId::all_alchemy_recipes();

    let menu_items: Vec<ListItem> = recipes.iter().enumerate().map(|(idx, &recipe_id)| {
        let recipe = Recipe::new(recipe_id).expect("Recipe should exist");
        let ingredients_str = recipe.ingredients()
            .iter()
            .map(|(&item_id, &qty)| {
                let have = gs.player.find_item_by_id(item_id)
                    .map(|inv| inv.quantity).unwrap_or(0);
                let name = gs.get_item_name(item_id);
                format!("{}: {}/{}", name, have, qty)
            })
            .collect::<Vec<_>>()
            .join(", ");

        ListItem::new(Line::from(vec![
            Span::raw(menu_padding.clone()),
            selection_prefix(selected == idx),
            Span::raw(format!("{} ({})", recipe.name(), ingredients_str)),
        ]))
    }).collect();

    frame.render_stateful_widget(List::new(menu_items), chunks[1], list_state);
}

pub fn handle(cmd: Cmd, list_state: &mut ListState) -> (CmdResult, Option<StateChange>) {
    let recipes = RecipeId::all_alchemy_recipes();
    let menu_size = recipes.len();

    match cmd {
        Cmd::Move(tuirealm::command::Direction::Up) => {
            list_move_up(list_state, menu_size);
            (CmdResult::Changed(tuirealm::State::None), None)
        }
        Cmd::Move(tuirealm::command::Direction::Down) => {
            list_move_down(list_state, menu_size);
            (CmdResult::Changed(tuirealm::State::None), None)
        }
        Cmd::Submit => {
            let selected = list_state.selected().unwrap_or(0);

            if selected < recipes.len() {
                let recipe_id = recipes[selected];
                let result = execute(GameCommand::BrewRecipe { recipe_id });
                apply_result(&result);
            }
            (CmdResult::Submit(tuirealm::State::None), None)
        }
        Cmd::Cancel => {
            (CmdResult::Changed(tuirealm::State::None), Some(StateChange::ToMenu))
        }
        _ => (CmdResult::None, None),
    }
}
