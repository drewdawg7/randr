use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    Frame,
};
use tuirealm::command::{Cmd, CmdResult};

use crate::{
    combat::HasGold,
    item::recipe::{Recipe, RecipeId},
    system::game_state,
};
use crate::ui::components::utilities::render_location_header;
use crate::ui::components::widgets::item_list::{
    ForgeFilter, ItemList, ItemListConfig, RecipeItem,
};
use crate::ui::theme as colors;

use super::anvil_art::render_anvil_art;
use super::StateChange;

const COIN: &str = "●";

/// Create a new ItemList for the forge screen.
pub fn create_item_list() -> ItemList<RecipeItem, ForgeFilter> {
    let config = ItemListConfig {
        show_filter_button: true,
        show_scroll_indicators: true,
        visible_count: 6,
        show_back_button: true,
        back_label: "Back",
        background: None,
    };
    ItemList::new(config)
}

/// Rebuild the list of forge recipes.
pub fn rebuild_items(item_list: &mut ItemList<RecipeItem, ForgeFilter>) {
    let recipes = RecipeId::all_forging_recipes();
    let items: Vec<RecipeItem> = recipes
        .into_iter()
        .filter_map(|recipe_id| {
            Recipe::new(recipe_id).ok().map(|recipe| {
                RecipeItem::new(recipe_id, recipe.name())
            })
        })
        .collect();
    item_list.set_items(items);
}

pub fn render(frame: &mut Frame, area: Rect, item_list: &mut ItemList<RecipeItem, ForgeFilter>) {
    let gs = game_state();
    let player_gold = gs.player.gold();
    let blacksmith = gs.blacksmith();

    let header_lines = vec![
        Line::from(vec![
            Span::styled(blacksmith.name.to_string(), Style::default().fg(colors::ORANGE)),
        ]),
        Line::from(vec![
            Span::styled(format!("{} ", COIN), Style::default().fg(colors::YELLOW)),
            Span::styled(format!("{}", player_gold), Style::default().fg(colors::WHITE)),
        ]),
    ];
    let content_area = render_location_header(frame, area, header_lines, colors::BLACKSMITH_BG, colors::DEEP_ORANGE);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12),
            Constraint::Min(6),
        ])
        .split(content_area);

    let anvil_width = 28u16;
    let h_padding = content_area.width.saturating_sub(anvil_width) / 2;

    // Anvil art - render directly to buffer to preserve background
    let anvil_lines = render_anvil_art(h_padding as usize);
    let buf = frame.buffer_mut();
    for (i, line) in anvil_lines.iter().enumerate() {
        if i < chunks[0].height as usize {
            let y = chunks[0].y + i as u16;
            let mut x = chunks[0].x;
            for span in line.spans.iter() {
                let has_style = span.style.fg.is_some() || span.style.bg.is_some();
                for ch in span.content.chars() {
                    if x < chunks[0].x + chunks[0].width {
                        // Skip spaces in unstyled spans to preserve background
                        if ch == ' ' && !has_style {
                            x += 1;
                            continue;
                        }
                        if let Some(cell) = buf.cell_mut((x, y)) {
                            cell.set_char(ch);
                            if let Some(fg) = span.style.fg {
                                cell.set_fg(fg);
                            }
                            if let Some(bg) = span.style.bg {
                                cell.set_bg(bg);
                            }
                        }
                        x += 1;
                    }
                }
            }
        }
    }

    // Rebuild items before rendering (to update ingredient counts)
    rebuild_items(item_list);

    // Use buffer rendering for menu area
    let menu_padding = " ".repeat(h_padding as usize);
    item_list.render_to_buffer(frame, chunks[1], &menu_padding);
}

pub fn handle(cmd: Cmd, item_list: &mut ItemList<RecipeItem, ForgeFilter>) -> (CmdResult, Option<StateChange>) {
    match cmd {
        Cmd::Move(tuirealm::command::Direction::Up) => {
            item_list.move_up();
            (CmdResult::Changed(tuirealm::State::None), None)
        }
        Cmd::Move(tuirealm::command::Direction::Down) => {
            item_list.move_down();
            (CmdResult::Changed(tuirealm::State::None), None)
        }
        Cmd::Submit => {
            if item_list.is_back_selected() {
                return (CmdResult::Submit(tuirealm::State::None), Some(StateChange::ToMenu));
            }

            if let Some(recipe_item) = item_list.selected_item() {
                let gs = game_state();
                if let Ok(recipe) = Recipe::new(recipe_item.recipe_id) {
                    if let Ok(item) = recipe.craft(&mut gs.player, |id| game_state().spawn_item(id)) {
                        use crate::inventory::HasInventory;
                        let _ = gs.player.add_to_inv(item);
                    }
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
