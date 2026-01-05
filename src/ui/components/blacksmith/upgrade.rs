use ratatui::{layout::Rect, Frame};
use tuirealm::command::{Cmd, CmdResult};

use crate::{
    combat::HasGold,
    commands::{apply_result, execute, GameCommand},
    inventory::FindsItems,
    item::ItemId,
    system::game_state,
    ui::components::player::item_details::render_item_details_beside,
    ui::components::utilities::{blacksmith_header, collect_player_equipment, render_location_header},
    ui::components::widgets::item_list::{InventoryFilter, ItemList, ItemListConfig, UpgradeableItem},
    ui::theme as colors,
};

use super::StateChange;

pub fn create_item_list() -> ItemList<UpgradeableItem, InventoryFilter> {
    let config = ItemListConfig {
        show_filter_button: true,
        show_scroll_indicators: true,
        visible_count: 10,
        show_back_button: false,
        back_label: "",
        background: None,
    };
    ItemList::new(config)
}

fn rebuild_items(item_list: &mut ItemList<UpgradeableItem, InventoryFilter>) {
    let items = collect_player_equipment();
    let player_gold = game_state().player.gold();
    let blacksmith = game_state().blacksmith();
    let max_upgrades = blacksmith.max_upgrades;

    let upgrade_items: Vec<UpgradeableItem> = items
        .into_iter()
        .map(|inv_item| {
            let upgrade_cost = blacksmith.calc_upgrade_cost(&inv_item.item);
            let at_max = inv_item.item.num_upgrades >= max_upgrades;
            let can_afford = player_gold >= upgrade_cost;
            UpgradeableItem {
                inv_item,
                upgrade_cost,
                at_max,
                can_afford,
            }
        })
        .collect();

    item_list.set_items(upgrade_items);
}

pub fn render(frame: &mut Frame, area: Rect, item_list: &mut ItemList<UpgradeableItem, InventoryFilter>) {
    rebuild_items(item_list);

    let player_gold = game_state().player.gold();
    let blacksmith = game_state().blacksmith();
    let stones = game_state()
        .player
        .find_item_by_id(ItemId::QualityUpgradeStone)
        .map(|inv| inv.quantity)
        .unwrap_or(0);

    let header_lines = blacksmith_header(blacksmith, player_gold, stones);
    let content_area =
        render_location_header(frame, area, header_lines, colors::BLACKSMITH_BG, colors::DEEP_ORANGE);

    // Render the item list with back button
    item_list.render(frame, content_area);

    // Render item details beside list if toggled on
    let selected_item = item_list.selected_item().map(|ui| &ui.inv_item.item);
    render_item_details_beside(frame, content_area, selected_item);
}

pub fn handle(
    cmd: Cmd,
    item_list: &mut ItemList<UpgradeableItem, InventoryFilter>,
) -> (CmdResult, Option<StateChange>) {
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
            if let Some(upgrade_item) = item_list.selected_item() {
                let result = execute(GameCommand::UpgradeItem {
                    item_uuid: upgrade_item.inv_item.item.item_uuid,
                });
                apply_result(&result);
                (CmdResult::Submit(tuirealm::State::None), None)
            } else {
                (CmdResult::None, None)
            }
        }
        Cmd::Cancel => (
            CmdResult::Changed(tuirealm::State::None),
            Some(StateChange::ToMenu),
        ),
        _ => (CmdResult::None, None),
    }
}
