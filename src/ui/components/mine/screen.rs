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

use super::cave_art::{self, CaveLayout};

pub struct MineScreen {
    props: Props,
    cave: Option<CaveLayout>,
}

impl MineScreen {
    pub fn new() -> Self {
        Self {
            props: Props::default(),
            cave: None,
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

        // Generate new cave when entering the screen
        if gs.screen_lifecycle().just_entered() {
            self.cave = Some(CaveLayout::generate());
        }

        // Ensure we have a cave (fallback)
        if self.cave.is_none() {
            self.cave = Some(CaveLayout::generate());
        }

        let frame_size = frame.area();

        // Fill entire screen with cave floor background
        let bg_fill = Block::default().style(Style::default().bg(colors::CAVE_FLOOR_BG));
        frame.render_widget(bg_fill, frame_size);

        // Render the cave art centered
        if let Some(cave) = &self.cave {
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
                if let Some(cave) = &mut self.cave {
                    cave.move_player(0, -1);
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                if let Some(cave) = &mut self.cave {
                    cave.move_player(0, 1);
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Left, .. }) => {
                if let Some(cave) = &mut self.cave {
                    cave.move_player(-1, 0);
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Right, .. }) => {
                if let Some(cave) = &mut self.cave {
                    cave.move_player(1, 0);
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Char(' '), .. }) => {
                if let Some(cave) = &mut self.cave {
                    // Check if on exit first
                    if cave.is_on_exit() {
                        // Exit the mine
                        game_state().current_screen = Id::Town;
                        return None;
                    }

                    // Otherwise, try to mine adjacent rock
                    if let Some(rock_type) = cave.mine_adjacent_rock() {
                        let gs = game_state();
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
