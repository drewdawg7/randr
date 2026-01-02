use ratatui::{
    layout::{Constraint, Direction as LayoutDirection, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use tuirealm::{
    command::{Cmd, CmdResult, Direction as CmdDirection},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

use crate::{
    dungeon::RoomType,
    inventory::HasInventory,
    system::{game_state, CombatSource},
    ui::{
        components::utilities::{list_move_down, list_move_up, selection_prefix, RETURN_ARROW},
        theme as colors, Id,
    },
};

/// Dungeon screen states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DungeonState {
    /// Player just entered a room, needs to interact
    RoomEntry,
    /// Room is cleared, player can navigate
    Navigation,
}

pub struct DungeonScreen {
    props: Props,
    state: DungeonState,
    list_state: ListState,
}

impl DungeonScreen {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            props: Props::default(),
            state: DungeonState::RoomEntry,
            list_state,
        }
    }

    fn reset_selection(&mut self) {
        self.list_state.select(Some(0));
    }

    /// Get the current menu size based on state
    fn menu_size(&self) -> usize {
        let gs = game_state();
        match self.state {
            DungeonState::RoomEntry => 1, // Just the action button
            DungeonState::Navigation => {
                if let Some(dungeon) = gs.dungeon() {
                    // Directions + Leave Dungeon
                    dungeon.available_directions().len() + 1
                } else {
                    1
                }
            }
        }
    }

    fn handle_room_entry_submit(&mut self) {
        let gs = game_state();

        // First, check if room is cleared and get room type
        let (is_cleared, room_type) = {
            if let Some(dungeon) = gs.dungeon() {
                if let Some(room) = dungeon.current_room() {
                    (room.is_cleared, Some(room.room_type))
                } else {
                    (false, None)
                }
            } else {
                (false, None)
            }
        };

        if is_cleared {
            self.state = DungeonState::Navigation;
            self.reset_selection();
            return;
        }

        let Some(room_type) = room_type else {
            return;
        };

        match room_type {
            RoomType::Monster => {
                // Spawn a mob and start combat
                let mob_result = {
                    if let Some(dungeon) = gs.dungeon() {
                        dungeon.spawn_mob()
                    } else {
                        return;
                    }
                };

                match mob_result {
                    Ok(mob) => {
                        gs.combat_source = CombatSource::Dungeon;
                        gs.start_combat(mob);
                        gs.current_screen = Id::Fight;
                    }
                    Err(_) => {
                        gs.toasts.error("No enemies to fight!");
                    }
                }
            }
            RoomType::Chest => {
                // Open the chest and get loot
                let loot_drops = {
                    if let Some(dungeon) = gs.dungeon_mut() {
                        if let Some(room) = dungeon.current_room_mut() {
                            let drops = room.open_chest();
                            room.clear();
                            drops
                        } else {
                            vec![]
                        }
                    } else {
                        vec![]
                    }
                };

                // Now add items to inventory and show toasts
                for loot_drop in &loot_drops {
                    for _ in 0..loot_drop.quantity {
                        let _ = gs.player.add_to_inv(loot_drop.item.clone());
                    }
                    gs.toasts.success(format!(
                        "Found: {} x{}",
                        loot_drop.item.name, loot_drop.quantity
                    ));
                }
                if loot_drops.is_empty() {
                    gs.toasts.info("The chest was empty.");
                }

                self.state = DungeonState::Navigation;
                self.reset_selection();
            }
            _ => {
                // For other room types, just clear and move to navigation
                if let Some(dungeon) = gs.dungeon_mut() {
                    if let Some(room) = dungeon.current_room_mut() {
                        room.clear();
                    }
                }
                self.state = DungeonState::Navigation;
                self.reset_selection();
            }
        }
    }

    fn handle_navigation_submit(&mut self) {
        let gs = game_state();
        let selected = self.list_state.selected().unwrap_or(0);

        if let Some(dungeon) = gs.dungeon_mut() {
            let directions = dungeon.available_directions();

            if selected < directions.len() {
                // Move in the selected direction
                let direction = directions[selected];
                if dungeon.move_player(direction).is_ok() {
                    self.state = DungeonState::RoomEntry;
                    self.reset_selection();
                }
            } else {
                // Leave dungeon
                gs.leave_dungeon();
            }
        }
    }

    /// Called when returning from combat
    pub fn on_combat_return(&mut self, victory: bool) {
        if victory {
            let gs = game_state();
            if let Some(dungeon) = gs.dungeon_mut() {
                if let Some(room) = dungeon.current_room_mut() {
                    room.clear();
                }
            }
            self.state = DungeonState::Navigation;
        }
        // On defeat, stay in RoomEntry state to retry
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
        let gs = game_state();

        // Check if we need to transition state after combat victory
        if let Some(dungeon) = gs.dungeon() {
            if let Some(room) = dungeon.current_room() {
                if room.is_cleared && self.state == DungeonState::RoomEntry {
                    self.state = DungeonState::Navigation;
                    self.reset_selection();
                }
            }
        }

        // Main layout: header + content
        let chunks = Layout::default()
            .direction(LayoutDirection::Vertical)
            .constraints([Constraint::Length(4), Constraint::Min(10)])
            .split(area);

        // Render header
        self.render_header(frame, chunks[0]);

        // Render content based on state
        match self.state {
            DungeonState::RoomEntry => self.render_room_entry(frame, chunks[1]),
            DungeonState::Navigation => self.render_navigation(frame, chunks[1]),
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
        let menu_size = self.menu_size();

        match cmd {
            Cmd::Move(CmdDirection::Up) => {
                list_move_up(&mut self.list_state, menu_size);
                CmdResult::Changed(tuirealm::State::None)
            }
            Cmd::Move(CmdDirection::Down) => {
                list_move_down(&mut self.list_state, menu_size);
                CmdResult::Changed(tuirealm::State::None)
            }
            Cmd::Submit => {
                match self.state {
                    DungeonState::RoomEntry => self.handle_room_entry_submit(),
                    DungeonState::Navigation => self.handle_navigation_submit(),
                }
                CmdResult::Submit(tuirealm::State::None)
            }
            Cmd::Cancel => {
                // ESC to leave dungeon
                game_state().leave_dungeon();
                CmdResult::Submit(tuirealm::State::None)
            }
            _ => CmdResult::None,
        }
    }
}

impl DungeonScreen {
    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let gs = game_state();
        let text_style = Style::default().fg(colors::WHITE);

        let (dungeon_name, progress) = if let Some(dungeon) = gs.dungeon() {
            let cleared = dungeon.cleared_count();
            let total = dungeon.room_count();
            (
                dungeon.name.clone(),
                format!("Rooms: {}/{}", cleared, total),
            )
        } else {
            ("Unknown".to_string(), "".to_string())
        };

        let position = if let Some(dungeon) = gs.dungeon() {
            let (x, y) = dungeon.player_position;
            format!("Position: ({}, {})", x, y)
        } else {
            "".to_string()
        };

        let header = Paragraph::new(vec![
            Line::from(vec![
                Span::styled(dungeon_name, Style::default().fg(colors::LIGHT_STONE)),
            ]),
            Line::from(vec![
                Span::styled(progress, text_style),
                Span::styled("  |  ", text_style),
                Span::styled(position, text_style),
            ]),
        ])
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(colors::GRANITE)),
        );

        frame.render_widget(header, area);
    }

    fn render_room_entry(&mut self, frame: &mut Frame, area: Rect) {
        let gs = game_state();
        let text_style = Style::default().fg(colors::WHITE);

        let (room_type_name, action_text) = if let Some(dungeon) = gs.dungeon() {
            if let Some(room) = dungeon.current_room() {
                let type_name = match room.room_type {
                    RoomType::Monster => "Monster Room",
                    RoomType::Boss => "Boss Room",
                    RoomType::Chest => "Treasure Chest",
                    RoomType::Rest => "Rest Area",
                    RoomType::Trap => "Trap Room",
                    RoomType::Treasure => "Treasure Room",
                };
                let action = match room.room_type {
                    RoomType::Monster | RoomType::Boss => "Fight",
                    RoomType::Chest | RoomType::Treasure => "Open",
                    _ => "Proceed",
                };
                (type_name, action)
            } else {
                ("Unknown", "Proceed")
            }
        } else {
            ("Unknown", "Proceed")
        };

        // Center the content
        let content_area = centered_rect(30, 8, area);

        // Room type display
        let room_info = Paragraph::new(vec![
            Line::from(vec![Span::styled(
                room_type_name,
                Style::default().fg(colors::YELLOW),
            )]),
            Line::from(""),
        ])
        .centered();

        let info_area = Rect {
            height: 2,
            ..content_area
        };
        frame.render_widget(room_info, info_area);

        // Action menu
        let selected = self.list_state.selected().unwrap_or(0);
        let menu_items: Vec<ListItem> = vec![ListItem::new(Line::from(vec![
            selection_prefix(selected == 0),
            Span::styled(action_text, text_style),
        ]))];

        let menu = List::new(menu_items);
        let menu_area = Rect {
            y: content_area.y + 3,
            height: 1,
            ..content_area
        };
        frame.render_stateful_widget(menu, menu_area, &mut self.list_state);
    }

    fn render_navigation(&mut self, frame: &mut Frame, area: Rect) {
        let gs = game_state();
        let text_style = Style::default().fg(colors::WHITE);

        let directions = if let Some(dungeon) = gs.dungeon() {
            dungeon.available_directions()
        } else {
            vec![]
        };

        // Center the content
        let content_area = centered_rect(30, 10, area);

        // Navigation title
        let title = Paragraph::new(vec![
            Line::from(vec![Span::styled(
                "Choose Direction",
                Style::default().fg(colors::CYAN),
            )]),
            Line::from(""),
        ])
        .centered();

        let title_area = Rect {
            height: 2,
            ..content_area
        };
        frame.render_widget(title, title_area);

        // Direction menu
        let selected = self.list_state.selected().unwrap_or(0);
        let mut menu_items: Vec<ListItem> = directions
            .iter()
            .enumerate()
            .map(|(i, dir)| {
                // Check if the room in that direction is cleared
                let room_status = if let Some(dungeon) = gs.dungeon() {
                    let (dx, dy) = dir.offset();
                    let (px, py) = dungeon.player_position;
                    if let Some(room) = dungeon.get_room(px + dx, py + dy) {
                        if room.is_cleared {
                            " (cleared)"
                        } else {
                            ""
                        }
                    } else {
                        ""
                    }
                } else {
                    ""
                };

                ListItem::new(Line::from(vec![
                    selection_prefix(selected == i),
                    Span::styled(dir.name(), text_style),
                    Span::styled(room_status, Style::default().fg(colors::DARK_STONE)),
                ]))
            })
            .collect();

        // Add "Leave Dungeon" option
        menu_items.push(ListItem::new(Line::from(vec![
            selection_prefix(selected == directions.len()),
            Span::styled(format!("{} Leave Dungeon", RETURN_ARROW), text_style),
        ])));

        let menu = List::new(menu_items);
        let menu_area = Rect {
            y: content_area.y + 3,
            height: (directions.len() + 2) as u16,
            ..content_area
        };
        frame.render_stateful_widget(menu, menu_area, &mut self.list_state);
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
            Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                self.perform(Cmd::Submit);
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                self.perform(Cmd::Cancel);
                None
            }
            _ => None,
        }
    }
}

/// Helper function to create a centered rect
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width.min(area.width), height.min(area.height))
}
