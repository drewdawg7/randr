use ratatui::{layout::Rect, widgets::ListState, Frame};
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

use crate::inventory::HasInventory;
use crate::system::game_state;
use crate::ui::components::utilities::collect_player_equipment;

use super::{forge, menu, quality, smelt, upgrade};

pub enum StateChange {
    ToMenu,
    ToUpgrade,
    ToQuality,
    ToSmelt,
    ToForge,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum BlacksmithState {
    Menu,
    Upgrade,
    Quality,
    Smelt,
    Forge,
}

pub struct BlacksmithTab {
    props: Props,
    state: BlacksmithState,
    list_state: ListState,
}

impl BlacksmithTab {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            props: Props::default(),
            state: BlacksmithState::Menu,
            list_state,
        }
    }

    fn reset_selection(&mut self) {
        self.list_state.select(Some(0));
    }

    fn apply_state_change(&mut self, change: StateChange) {
        match change {
            StateChange::ToMenu => self.state = BlacksmithState::Menu,
            StateChange::ToUpgrade => self.state = BlacksmithState::Upgrade,
            StateChange::ToQuality => self.state = BlacksmithState::Quality,
            StateChange::ToSmelt => self.state = BlacksmithState::Smelt,
            StateChange::ToForge => self.state = BlacksmithState::Forge,
        }
        self.reset_selection();
    }
}

impl MockComponent for BlacksmithTab {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        match self.state {
            BlacksmithState::Menu => menu::render(frame, area, &mut self.list_state),
            BlacksmithState::Upgrade => upgrade::render(frame, area, &mut self.list_state),
            BlacksmithState::Quality => quality::render(frame, area, &mut self.list_state),
            BlacksmithState::Smelt => smelt::render(frame, area, &mut self.list_state),
            BlacksmithState::Forge => forge::render(frame, area, &mut self.list_state),
        }
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.list_state.selected().unwrap_or(0)))
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        let (result, state_change) = match self.state {
            BlacksmithState::Menu => menu::handle(cmd, &mut self.list_state),
            BlacksmithState::Upgrade => upgrade::handle(cmd, &mut self.list_state),
            BlacksmithState::Quality => quality::handle(cmd, &mut self.list_state),
            BlacksmithState::Smelt => smelt::handle(cmd, &mut self.list_state),
            BlacksmithState::Forge => forge::handle(cmd, &mut self.list_state),
        };

        if let Some(change) = state_change {
            self.apply_state_change(change);
        }

        result
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for BlacksmithTab {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(tuirealm::command::Direction::Up));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                self.perform(Cmd::Move(tuirealm::command::Direction::Down));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                self.perform(Cmd::Submit);
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                self.perform(Cmd::Cancel);
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Char('E'), .. }) => {
                if self.state == BlacksmithState::Upgrade || self.state == BlacksmithState::Quality {
                    let items = collect_player_equipment();
                    let selected = self.list_state.selected().unwrap_or(0);
                    if let Some(inv_item) = items.get(selected) {
                        let item = &inv_item.item;
                        if let Some(slot) = item.item_type.equipment_slot() {
                            if item.is_equipped {
                                let _ = game_state().player.unequip_item(slot);
                            } else {
                                game_state().player.equip_from_inventory(item.item_uuid, slot);
                            }
                        }
                    }
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Char('L'), modifiers: KeyModifiers::SHIFT }) => {
                if self.state == BlacksmithState::Upgrade || self.state == BlacksmithState::Quality {
                    let items = collect_player_equipment();
                    let selected = self.list_state.selected().unwrap_or(0);
                    if let Some(inv_item) = items.get(selected) {
                        if let Some(player_inv_item) = game_state().player.find_item_by_uuid_mut(inv_item.item.item_uuid) {
                            player_inv_item.item.toggle_lock();
                        }
                    }
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Char('d'), .. }) => {
                if self.state == BlacksmithState::Upgrade || self.state == BlacksmithState::Quality {
                    game_state().show_item_details = !game_state().show_item_details;
                }
                None
            }
            _ => None,
        }
    }
}
