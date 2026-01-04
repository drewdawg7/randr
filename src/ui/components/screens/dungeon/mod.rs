//! Dungeon exploration screen.
//!
//! This screen handles dungeon exploration with multiple states:
//! - RoomEntry: Initial interaction when entering a room
//! - Navigation: Compass-based movement between rooms
//! - RestRoom: Rest areas where the player can heal
//! - BossRoom: Boss fights (player is trapped until victory or death)

mod boss_room;
mod navigation;
mod rest_room;
mod room_entry;

use ratatui::{
    layout::{Constraint, Direction as LayoutDirection, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, ListState, Paragraph},
    Frame,
};
use tuirealm::{
    command::{Cmd, CmdResult, Direction as CmdDirection},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

use crate::{
    commands::{apply_result, execute, GameCommand},
    dungeon::RoomType,
    system::game_state,
    ui::{
        components::{
            backgrounds::render_stone_wall,
            dungeon::minimap,
            widgets::border::BorderTheme,
        },
        theme as colors,
    },
};

/// Dungeon screen states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DungeonState {
    /// Player just entered a room, needs to interact
    RoomEntry,
    /// Room is cleared, player can navigate
    Navigation,
    /// Player is in a rest room (can heal)
    RestRoom,
    /// Player is in the boss room (trapped until defeated)
    BossRoom,
}

/// Compass position for navigation selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompassPosition {
    North,
    East,
    South,
    West,
    Center, // Leave Dungeon
}

pub struct DungeonScreen {
    props: Props,
    state: DungeonState,
    list_state: ListState,
    compass_selection: CompassPosition,
    rest_selection: usize, // 0 = Rest/Heal, 1 = Leave
    /// Last combat message for boss fight (player attack, boss attack)
    boss_combat_log: Vec<String>,
}

impl DungeonScreen {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            props: Props::default(),
            state: DungeonState::RoomEntry,
            list_state,
            compass_selection: CompassPosition::Center,
            rest_selection: 0,
            boss_combat_log: Vec::new(),
        }
    }

    fn reset_selection(&mut self) {
        self.list_state.select(Some(0));
        self.compass_selection = CompassPosition::Center;
        self.rest_selection = 0;
    }

    /// Called when returning from combat.
    /// Note: Room clearing is handled by the command layer (ReturnFromCombat/PlayerAttack),
    /// so this method only needs to update UI state.
    pub fn on_combat_return(&mut self, victory: bool) {
        if victory {
            // Room is already cleared by command layer
            self.state = DungeonState::Navigation;
        }
        // On defeat, stay in RoomEntry state to retry
        self.reset_selection();
    }

    /// Transition to a new state with selection reset.
    fn transition_to(&mut self, new_state: DungeonState) {
        self.state = new_state;
        if new_state == DungeonState::BossRoom {
            self.boss_combat_log.clear();
        }
        self.reset_selection();
    }
}

impl Default for DungeonScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl MockComponent for DungeonScreen {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let frame_size = frame.area();

        // Fill background with MINE_BG to match border
        let bg_fill = Block::default().style(Style::default().bg(colors::MINE_BG));
        frame.render_widget(bg_fill, area);

        // Render stone wall pattern on top
        render_stone_wall(frame, area);

        // Calculate inner area (inside the border - 1px on each side)
        let inner_area = Rect {
            x: area.x + 1,
            y: area.y + 1,
            width: area.width.saturating_sub(2),
            height: area.height.saturating_sub(2),
        };

        let gs = game_state();

        // Reset screen state if entering a fresh dungeon
        // A fresh dungeon has only 1 visited room (the start) which is NOT cleared yet
        if let Some(dungeon) = gs.dungeon() {
            let visited_count = dungeon.rooms.iter()
                .flat_map(|row| row.iter())
                .filter_map(|r| r.as_ref())
                .filter(|r| r.is_visited)
                .count();

            let current_room_not_cleared = dungeon
                .current_room()
                .map(|r| !r.is_cleared)
                .unwrap_or(false);

            // Fresh dungeon: only start room visited, current room not cleared,
            // and we're in a non-RoomEntry state (stale from previous dungeon)
            if visited_count == 1
                && current_room_not_cleared
                && self.state != DungeonState::RoomEntry
            {
                self.state = DungeonState::RoomEntry;
                self.boss_combat_log.clear();
                self.reset_selection();
            }
        }

        // Check if we need to transition state after combat victory
        if let Some(dungeon) = gs.dungeon() {
            if let Some(room) = dungeon.current_room() {
                if room.is_cleared && self.state == DungeonState::RoomEntry {
                    // Rest rooms go to RestRoom state, others to Navigation
                    if room.room_type == RoomType::Rest {
                        self.state = DungeonState::RestRoom;
                    } else {
                        self.state = DungeonState::Navigation;
                    }
                    self.reset_selection();
                }
            }
        }

        // Main layout: header + content
        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([Constraint::Length(4), Constraint::Min(10)])
            .split(inner_area);

        // Render header
        self.render_header(frame, chunks[0]);

        // Render content based on state
        match self.state {
            DungeonState::RoomEntry => {
                room_entry::render(frame, chunks[1], &mut self.list_state);
            }
            DungeonState::Navigation => {
                navigation::render(frame, chunks[1], self.compass_selection);
            }
            DungeonState::RestRoom => {
                rest_room::render(frame, chunks[1], self.rest_selection);
            }
            DungeonState::BossRoom => {
                boss_room::render(frame, chunks[1], &self.boss_combat_log);
            }
        }

        // Render minimap in bottom-left corner (inside border)
        self.render_minimap(frame, inner_area);

        // Render ASCII art border (Stone theme like dungeon tab)
        let border = BorderTheme::Stone;
        let border_style = Style::default().bg(colors::MINE_BG);

        // Top and bottom borders
        let top_border = border.generate_top_border(frame_size.width);
        let bottom_border = border.generate_bottom_border(frame_size.width);
        let top_area = Rect::new(0, 0, frame_size.width, 1);
        let bottom_area = Rect::new(0, frame_size.height.saturating_sub(1), frame_size.width, 1);
        frame.render_widget(Paragraph::new(top_border).style(border_style), top_area);
        frame.render_widget(Paragraph::new(bottom_border).style(border_style), bottom_area);

        // Left and right borders
        let content_height = frame_size.height.saturating_sub(2);
        for row in 0..content_height {
            let left_char = border.generate_left_border_char(row);
            let right_char = border.generate_right_border_char(row);
            let left_area = Rect::new(0, 1 + row, 1, 1);
            let right_area = Rect::new(frame_size.width.saturating_sub(1), 1 + row, 1, 1);
            frame.render_widget(Paragraph::new(Line::from(left_char)).style(border_style), left_area);
            frame.render_widget(Paragraph::new(Line::from(right_char)).style(border_style), right_area);
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
        match cmd {
            Cmd::Move(dir) => {
                match self.state {
                    DungeonState::RoomEntry => {
                        // RoomEntry can have 2 options if bypass is available
                        let max_options = if room_entry::can_bypass_current_room() { 2 } else { 1 };
                        match dir {
                            CmdDirection::Up => {
                                let current = self.list_state.selected().unwrap_or(0);
                                if current > 0 {
                                    self.list_state.select(Some(current - 1));
                                }
                            }
                            CmdDirection::Down => {
                                let current = self.list_state.selected().unwrap_or(0);
                                if current < max_options - 1 {
                                    self.list_state.select(Some(current + 1));
                                }
                            }
                            _ => {}
                        }
                    }
                    DungeonState::Navigation => {
                        self.compass_selection = navigation::compass_move(self.compass_selection, dir);
                    }
                    DungeonState::RestRoom => {
                        // RestRoom has 2 options: Rest/Heal (0) and Leave (1)
                        match dir {
                            CmdDirection::Up => {
                                if self.rest_selection > 0 {
                                    self.rest_selection -= 1;
                                }
                            }
                            CmdDirection::Down => {
                                if self.rest_selection < 1 {
                                    self.rest_selection += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                    DungeonState::BossRoom => {
                        // BossRoom only has one option (Attack), no movement needed
                    }
                }
                CmdResult::Changed(tuirealm::State::None)
            }
            Cmd::Submit => {
                match self.state {
                    DungeonState::RoomEntry => {
                        let selected = self.list_state.selected().unwrap_or(0);
                        if let Some(new_state) = room_entry::handle_submit(selected) {
                            self.transition_to(new_state);
                        }
                    }
                    DungeonState::Navigation => {
                        if let Some(new_state) = navigation::handle_submit(self.compass_selection) {
                            self.transition_to(new_state);
                        }
                    }
                    DungeonState::RestRoom => {
                        if let Some(new_state) = rest_room::handle_submit(self.rest_selection) {
                            self.transition_to(new_state);
                        }
                    }
                    DungeonState::BossRoom => {
                        if let Some(new_state) = boss_room::handle_submit(&mut self.boss_combat_log) {
                            self.transition_to(new_state);
                        }
                    }
                }
                CmdResult::Submit(tuirealm::State::None)
            }
            Cmd::Cancel => {
                // ESC to leave dungeon (but not from boss room!)
                if self.state != DungeonState::BossRoom {
                    let result = execute(GameCommand::LeaveDungeon);
                    apply_result(&result);
                }
                CmdResult::Submit(tuirealm::State::None)
            }
            _ => CmdResult::None,
        }
    }
}

impl DungeonScreen {
    fn render_minimap(&self, frame: &mut Frame, area: Rect) {
        let gs = game_state();
        if let Some(dungeon) = gs.dungeon() {
            let (map_width, map_height) = minimap::minimap_size();

            // Position in bottom-left corner with some padding
            let padding = 1;
            let map_area = Rect {
                x: area.x + padding,
                y: area.y + area.height.saturating_sub(map_height + padding),
                width: map_width,
                height: map_height,
            };

            // Only render if we have enough space
            if map_area.width > 0 && map_area.height > 0 {
                minimap::render_minimap(frame, map_area, dungeon);
            }
        }
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let gs = game_state();

        let dungeon_name = if let Some(dungeon) = gs.dungeon() {
            dungeon.name.clone()
        } else {
            "Unknown".to_string()
        };

        let header = Paragraph::new(vec![Line::from(vec![Span::styled(
            dungeon_name,
            Style::default().fg(colors::LIGHT_STONE),
        )])])
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(colors::GRANITE)),
        );

        frame.render_widget(header, area);
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for DungeonScreen {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                self.perform(Cmd::Move(CmdDirection::Up));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                self.perform(Cmd::Move(CmdDirection::Down));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Left, .. }) => {
                self.perform(Cmd::Move(CmdDirection::Left));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Right, .. }) => {
                self.perform(Cmd::Move(CmdDirection::Right));
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                self.perform(Cmd::Submit);
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Backspace, .. }) => {
                self.perform(Cmd::Cancel);
                None
            }
            _ => None,
        }
    }
}

/// Helper function to create a centered rect
pub fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}
