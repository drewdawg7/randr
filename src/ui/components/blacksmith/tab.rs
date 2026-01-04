use ratatui::{layout::Rect, widgets::ListState, Frame};
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

use crate::commands::{apply_result, execute, GameCommand};
use crate::system::game_state;
use crate::ui::components::backgrounds::render_stone_wall;
use crate::ui::components::widgets::item_list::{
    ForgeFilter, InventoryFilter, ItemList, QualityItem, RecipeItem, UpgradeableItem,
};

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
    menu_list_state: ListState,
    smelt_list_state: ListState,
    forge_list: ItemList<RecipeItem, ForgeFilter>,
    upgrade_list: ItemList<UpgradeableItem, InventoryFilter>,
    quality_list: ItemList<QualityItem, InventoryFilter>,
}

impl BlacksmithTab {
    pub fn new() -> Self {
        let mut menu_list_state = ListState::default();
        menu_list_state.select(Some(0));
        let mut smelt_list_state = ListState::default();
        smelt_list_state.select(Some(0));

        Self {
            props: Props::default(),
            state: BlacksmithState::Menu,
            menu_list_state,
            smelt_list_state,
            forge_list: forge::create_item_list(),
            upgrade_list: upgrade::create_item_list(),
            quality_list: quality::create_item_list(),
        }
    }

    fn reset_selection(&mut self) {
        self.menu_list_state.select(Some(0));
        self.smelt_list_state.select(Some(0));
        self.forge_list.reset_selection();
        self.upgrade_list.reset_selection();
        self.quality_list.reset_selection();
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
            BlacksmithState::Menu => {
                // Render stone wall background first, then menu on top
                render_stone_wall(frame, area);
                menu::render(frame, area, &mut self.menu_list_state);
            }
            BlacksmithState::Upgrade => {
                render_stone_wall(frame, area);
                upgrade::render(frame, area, &mut self.upgrade_list);
            }
            BlacksmithState::Quality => {
                render_stone_wall(frame, area);
                quality::render(frame, area, &mut self.quality_list);
            }
            BlacksmithState::Smelt => {
                render_stone_wall(frame, area);
                smelt::render(frame, area, &mut self.smelt_list_state);
            }
            BlacksmithState::Forge => {
                render_stone_wall(frame, area);
                forge::render(frame, area, &mut self.forge_list);
            }
        }
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        let index = match self.state {
            BlacksmithState::Menu => self.menu_list_state.selected().unwrap_or(0),
            BlacksmithState::Upgrade => self.upgrade_list.selected_index(),
            BlacksmithState::Quality => self.quality_list.selected_index(),
            BlacksmithState::Smelt => self.smelt_list_state.selected().unwrap_or(0),
            BlacksmithState::Forge => self.forge_list.selected_index(),
        };
        State::One(StateValue::Usize(index))
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        let (result, state_change) = match self.state {
            BlacksmithState::Menu => menu::handle(cmd, &mut self.menu_list_state),
            BlacksmithState::Upgrade => upgrade::handle(cmd, &mut self.upgrade_list),
            BlacksmithState::Quality => quality::handle(cmd, &mut self.quality_list),
            BlacksmithState::Smelt => smelt::handle(cmd, &mut self.smelt_list_state),
            BlacksmithState::Forge => forge::handle(cmd, &mut self.forge_list),
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
            Event::Keyboard(KeyEvent {
                code: Key::Char('E'),
                ..
            }) => {
                if self.state == BlacksmithState::Upgrade {
                    if let Some(item) = self.upgrade_list.selected_item() {
                        let inv_item = &item.inv_item;
                        if let Some(slot) = inv_item.item.item_type.equipment_slot() {
                            let result = if inv_item.item.is_equipped {
                                execute(GameCommand::UnequipItem { slot })
                            } else {
                                execute(GameCommand::EquipItem {
                                    item_uuid: inv_item.item.item_uuid,
                                    slot,
                                })
                            };
                            apply_result(&result);
                        }
                    }
                } else if self.state == BlacksmithState::Quality {
                    if let Some(item) = self.quality_list.selected_item() {
                        let inv_item = &item.inv_item;
                        if let Some(slot) = inv_item.item.item_type.equipment_slot() {
                            let result = if inv_item.item.is_equipped {
                                execute(GameCommand::UnequipItem { slot })
                            } else {
                                execute(GameCommand::EquipItem {
                                    item_uuid: inv_item.item.item_uuid,
                                    slot,
                                })
                            };
                            apply_result(&result);
                        }
                    }
                }
                None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('L'),
                modifiers: KeyModifiers::SHIFT,
            }) => {
                if self.state == BlacksmithState::Upgrade {
                    if let Some(item) = self.upgrade_list.selected_item() {
                        let result = execute(GameCommand::ToggleLock {
                            item_uuid: item.inv_item.item.item_uuid,
                        });
                        apply_result(&result);
                    }
                } else if self.state == BlacksmithState::Quality {
                    if let Some(item) = self.quality_list.selected_item() {
                        let result = execute(GameCommand::ToggleLock {
                            item_uuid: item.inv_item.item.item_uuid,
                        });
                        apply_result(&result);
                    }
                }
                None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('d'),
                ..
            }) => {
                if self.state == BlacksmithState::Upgrade || self.state == BlacksmithState::Quality
                {
                    game_state().show_item_details = !game_state().show_item_details;
                }
                None
            }
            Event::Keyboard(KeyEvent {
                code: Key::Char('f') | Key::Char('F'),
                ..
            }) => {
                // Cycle filter in upgrade/quality/forge mode
                if self.state == BlacksmithState::Upgrade {
                    self.upgrade_list.cycle_filter();
                } else if self.state == BlacksmithState::Quality {
                    self.quality_list.cycle_filter();
                } else if self.state == BlacksmithState::Forge {
                    self.forge_list.cycle_filter();
                }
                None
            }
            // Pass unhandled events back to parent (for tab switching)
            _ => Some(ev),
        }
    }
}
