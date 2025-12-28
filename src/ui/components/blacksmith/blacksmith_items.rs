use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
    Frame,
};
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

use crate::{
    combat::HasGold,
    inventory::{EquipmentSlot, HasInventory},
    item::Item,
    system::game_state,
    ui::Id,
};
use crate::ui::components::player::item_details::render_item_details;
use crate::ui::components::utilities::blacksmith_header;
use crate::ui::components::wrappers::with_action::WithAction;

pub struct UpgradeableItem {
    pub item: Item,
}

impl MockComponent for UpgradeableItem {
    fn view(&mut self, _frame: &mut Frame, _area: Rect) {}
    fn query(&self, _attr: Attribute) -> Option<AttrValue> { None }
    fn attr(&mut self, _attr: Attribute, _value: AttrValue) {}
    fn state(&self) -> State { State::None }
    fn perform(&mut self, _cmd: Cmd) -> CmdResult { CmdResult::None }
}

impl UpgradeableItem {
    pub fn new(item: Item) -> WithAction<Self> {
        let item_uuid = item.item_uuid;
        let inner = Self { item };

        WithAction::new(inner, move || {
            let gs = game_state();
            let player = &mut gs.player;

            // Check equipped items first
            for slot in [EquipmentSlot::Weapon, EquipmentSlot::OffHand] {
                if let Some(equipped) = player.get_equipped_item(slot) && equipped.item_uuid == item_uuid {
                        // Need to unequip, upgrade, and re-equip
                        let _ = player.unequip_item(slot);
                        // Find the item we just unequipped in inventory
                        if let Some(idx) = player.find_item_index_by_uuid(item_uuid) {
                            let items = &mut player.inventory_mut().items;
                            let _ = game_state().blacksmith.upgrade_item(&mut items[idx].item);
                            let inv_item = items.remove(idx);
                            let mut upgraded = inv_item.item;
                            player.equip_item(&mut upgraded, slot);
                        }
                        return;
                }
            }

            // Check inventory items
            if let Some(idx) = player.find_item_index_by_uuid(item_uuid) {
                let items = &mut player.inventory_mut().items;
                let _ = game_state().blacksmith.upgrade_item(&mut items[idx].item);
            }
        })
    }
}

pub struct BlacksmithItems {
    props: Props,
    items: Vec<WithAction<UpgradeableItem>>,
    list_state: ListState,
}

impl Default for BlacksmithItems {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            props: Props::default(),
            items: Vec::new(),
            list_state,
        }
    }
}

impl BlacksmithItems {
    fn rebuild_items(&mut self) {
        self.items.clear();

        // Add equipped items
        if let Some(item) = game_state().player.get_equipped_item(EquipmentSlot::Weapon) {
            self.items.push(UpgradeableItem::new(item.clone()));
        }
        if let Some(item) = game_state().player.get_equipped_item(EquipmentSlot::OffHand) {
            self.items.push(UpgradeableItem::new(item.clone()));
        }

        // Add inventory items
        for inv_item in game_state().player.get_inventory_items().iter() {
            self.items.push(UpgradeableItem::new(inv_item.item.clone()));
        }
    }
}

impl MockComponent for BlacksmithItems {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        self.rebuild_items();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(0)])
            .split(area);

        // Render header with blacksmith name, gold, and max upgrades
        let player_gold = game_state().player.gold();
        let blacksmith = game_state().blacksmith();
        let max_upgrades = blacksmith.max_upgrades;
        let header_line = blacksmith_header(&blacksmith, player_gold);
        frame.render_widget(Paragraph::new(header_line), chunks[0]);

        // Split content area into list (left) and details (right)
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let selected = self.list_state.selected().unwrap_or(0);

        let list_items: Vec<ListItem> = self.items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let inner = item.inner();
                let is_selected = selected == i;
                let prefix = if is_selected { "> " } else { "  " };
                let upgrade_cost = blacksmith.calc_upgrade_cost(&inner.item);
                let at_max = inner.item.num_upgrades >= max_upgrades;
                let can_afford = player_gold >= upgrade_cost;

                let base_style = if is_selected {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                };

                let line = if at_max {
                    // Item is at max upgrades - show in gray/dim
                    let dim_style = Style::default().fg(Color::DarkGray);
                    Line::from(vec![
                        Span::styled(prefix, base_style),
                        Span::styled(inner.item.name.to_string(), dim_style),
                        Span::styled(
                            format!(" ({}/{}) - ", inner.item.num_upgrades, max_upgrades),
                            dim_style,
                        ),
                        Span::styled("MAX", dim_style),
                    ])
                } else {
                    let cost_style = if can_afford {
                        base_style
                    } else {
                        Style::default().fg(Color::Red)
                    };

                    Line::from(vec![
                        Span::styled(prefix, base_style),
                        Span::styled(inner.item.name.to_string(), base_style),
                        Span::styled(
                            format!(" ({}/{}) - ", inner.item.num_upgrades, max_upgrades),
                            base_style,
                        ),
                        Span::styled(format!("{} gold", upgrade_cost), cost_style),
                    ])
                };

                ListItem::new(line)
            })
            .collect();

        // Add back button
        let back_selected = selected == self.items.len();
        let back_style = if back_selected {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let back_prefix = if back_selected { "> " } else { "  " };

        let mut all_items = list_items;
        all_items.push(ListItem::new(Line::from(vec![
            Span::styled(back_prefix, back_style),
            Span::styled(format!("{} Back", crate::ui::utilities::RETURN_ARROW), back_style),
        ])));

        let list = List::new(all_items);
        frame.render_stateful_widget(list, content_chunks[0], &mut self.list_state);

        // Render item details panel on the right
        let selected_item = if selected < self.items.len() {
            Some(&self.items[selected].inner().item)
        } else {
            None
        };
        render_item_details(frame, content_chunks[1], selected_item);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.list_state.selected().unwrap_or(0)))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(tuirealm::command::Direction::Up) => {
                let total = self.items.len() + 1;
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = if current == 0 { total - 1 } else { current - 1 };
                self.list_state.select(Some(new_idx));
                CmdResult::Changed(self.state())
            }
            Cmd::Move(tuirealm::command::Direction::Down) => {
                let total = self.items.len() + 1;
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = (current + 1) % total;
                self.list_state.select(Some(new_idx));
                CmdResult::Changed(self.state())
            }
            Cmd::Submit => {
                let selected = self.list_state.selected().unwrap_or(0);
                if selected < self.items.len() {
                    self.items[selected].perform(Cmd::Submit);
                }
                CmdResult::Submit(self.state())
            }
            _ => CmdResult::None,
        }
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for BlacksmithItems {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        let total_items = self.items.len() + 1; // items + back button
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = if current == 0 { total_items - 1 } else { current - 1 };
                self.list_state.select(Some(new_idx));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                let current = self.list_state.selected().unwrap_or(0);
                let new_idx = (current + 1) % total_items;
                self.list_state.select(Some(new_idx));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                let selected = self.list_state.selected().unwrap_or(0);
                if selected == self.items.len() {
                    // Back button
                    game_state().current_screen = Id::Town;
                } else if selected < self.items.len() {
                    self.items[selected].perform(Cmd::Submit);
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                game_state().current_screen = Id::Town;
                None
            }
            _ => None,
        }
    }
}
