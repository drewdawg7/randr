use ratatui::{
    layout::Rect,
    style::Style,
    widgets::Block,
    Frame,
};

use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State,
};

use crate::loot::collect_loot_drops;
use crate::system::game_state;
use crate::ui::Id;
use crate::ui::theme as colors;

use super::cave_art;

pub struct MineScreen {
    props: Props,
}

impl MineScreen {
    pub fn new() -> Self {
        Self {
            props: Props::default(),
        }
    }
}

impl Default for MineScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl MockComponent for MineScreen {
    fn view(&mut self, frame: &mut Frame, _area: Rect) {
        let gs = game_state();

        // Ensure the mine has a cave
        gs.town.mine.ensure_cave_exists();

        let frame_size = frame.area();

        // Fill entire screen with cave floor background
        let bg_fill = Block::default().style(Style::default().bg(colors::CAVE_FLOOR_BG));
        frame.render_widget(bg_fill, frame_size);

        // Render the cave art centered
        if let Some(cave) = gs.town.mine.cave() {
            cave_art::render_cave(frame, frame_size, cave);
        }
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for MineScreen {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Backspace, .. }) => {
                // Backspace goes back to Town
                game_state().current_screen = Id::Town;
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                if let Some(cave) = game_state().town.mine.cave_mut() {
                    cave.move_player(0, -1);
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                if let Some(cave) = game_state().town.mine.cave_mut() {
                    cave.move_player(0, 1);
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Left, .. }) => {
                if let Some(cave) = game_state().town.mine.cave_mut() {
                    cave.move_player(-1, 0);
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Right, .. }) => {
                if let Some(cave) = game_state().town.mine.cave_mut() {
                    cave.move_player(1, 0);
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Char(' '), .. }) => {
                let gs = game_state();

                // Check if on exit first
                if let Some(cave) = gs.town.mine.cave() {
                    if cave.is_on_exit() {
                        gs.current_screen = Id::Town;
                        return None;
                    }
                }

                // Otherwise, try to mine adjacent rock
                if let Some(cave) = gs.town.mine.cave_mut() {
                    if let Some(rock_type) = cave.mine_adjacent_rock() {
                        let loot_table = rock_type.loot_table();

                        // Roll drops (0 magic find, spawn items from game state)
                        let drops = loot_table.roll_drops_with_spawner(0, |item_id| {
                            gs.spawn_item(item_id)
                        });

                        // Collect drops into player inventory with toast notifications
                        collect_loot_drops(
                            &mut gs.player,
                            &drops,
                            Some(&mut gs.toasts),
                        );
                    }
                }
                None
            }
            _ => None
        }
    }
}
