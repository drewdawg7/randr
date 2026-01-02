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
    dungeon::{Direction, RoomType},
    inventory::HasInventory,
    system::{game_state, CombatSource},
    ui::{
        components::{
            backgrounds::render_stone_wall,
            dungeon::minimap,
            utilities::{selection_prefix, RETURN_ARROW},
            widgets::border::BorderTheme,
        },
        theme::{self as colors, ColorExt},
        Id,
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

/// Compass position for navigation selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CompassPosition {
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
        }
    }

    fn reset_selection(&mut self) {
        self.list_state.select(Some(0));
        self.compass_selection = CompassPosition::Center;
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

        match self.compass_selection {
            CompassPosition::Center => {
                gs.leave_dungeon();
            }
            CompassPosition::North => self.try_move(Direction::North),
            CompassPosition::East => self.try_move(Direction::East),
            CompassPosition::South => self.try_move(Direction::South),
            CompassPosition::West => self.try_move(Direction::West),
        }
    }

    fn try_move(&mut self, direction: Direction) {
        let gs = game_state();
        if let Some(dungeon) = gs.dungeon_mut() {
            if dungeon.move_player(direction).is_ok() {
                self.state = DungeonState::RoomEntry;
                self.reset_selection();
            }
        }
    }

    /// Navigate compass selection based on arrow key
    fn compass_move(&mut self, cmd_dir: CmdDirection) {
        let gs = game_state();
        let available = if let Some(dungeon) = gs.dungeon() {
            dungeon.available_directions()
        } else {
            vec![]
        };

        let has_north = available.contains(&Direction::North);
        let has_south = available.contains(&Direction::South);
        let has_east = available.contains(&Direction::East);
        let has_west = available.contains(&Direction::West);

        self.compass_selection = match (self.compass_selection, cmd_dir) {
            // From Center
            (CompassPosition::Center, CmdDirection::Up) if has_north => CompassPosition::North,
            (CompassPosition::Center, CmdDirection::Down) if has_south => CompassPosition::South,
            (CompassPosition::Center, CmdDirection::Left) if has_west => CompassPosition::West,
            (CompassPosition::Center, CmdDirection::Right) if has_east => CompassPosition::East,

            // From North
            (CompassPosition::North, CmdDirection::Down) => CompassPosition::Center,
            (CompassPosition::North, CmdDirection::Left) if has_west => CompassPosition::West,
            (CompassPosition::North, CmdDirection::Right) if has_east => CompassPosition::East,

            // From South
            (CompassPosition::South, CmdDirection::Up) => CompassPosition::Center,
            (CompassPosition::South, CmdDirection::Left) if has_west => CompassPosition::West,
            (CompassPosition::South, CmdDirection::Right) if has_east => CompassPosition::East,

            // From West
            (CompassPosition::West, CmdDirection::Right) => CompassPosition::Center,
            (CompassPosition::West, CmdDirection::Up) if has_north => CompassPosition::North,
            (CompassPosition::West, CmdDirection::Down) if has_south => CompassPosition::South,

            // From East
            (CompassPosition::East, CmdDirection::Left) => CompassPosition::Center,
            (CompassPosition::East, CmdDirection::Up) if has_north => CompassPosition::North,
            (CompassPosition::East, CmdDirection::Down) if has_south => CompassPosition::South,

            // No change for invalid moves
            (current, _) => current,
        };
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
            .split(inner_area);

        // Render header
        self.render_header(frame, chunks[0]);

        // Render content based on state
        match self.state {
            DungeonState::RoomEntry => self.render_room_entry(frame, chunks[1]),
            DungeonState::Navigation => self.render_navigation(frame, chunks[1]),
        }

        // Render minimap in bottom-left corner (inside border)
        self.render_minimap(frame, inner_area);

        // Render ASCII art border (Stone theme like dungeon tab)
        let border = BorderTheme::Stone;
        let border_style = Style::default().on_color(colors::MINE_BG);

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
                        // RoomEntry only has one option, no movement needed
                    }
                    DungeonState::Navigation => {
                        self.compass_move(dir);
                    }
                }
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

        let available = if let Some(dungeon) = gs.dungeon() {
            dungeon.available_directions()
        } else {
            vec![]
        };

        let has_north = available.contains(&Direction::North);
        let has_south = available.contains(&Direction::South);
        let has_east = available.contains(&Direction::East);
        let has_west = available.contains(&Direction::West);

        // Compass layout dimensions
        const BUTTON_WIDTH: u16 = 16;
        const BUTTON_HEIGHT: u16 = 1;
        const COMPASS_WIDTH: u16 = BUTTON_WIDTH * 3 + 4; // 3 columns + spacing
        const COMPASS_HEIGHT: u16 = 7; // title + north + middle row + south + padding

        // Center the compass
        let content_area = centered_rect(COMPASS_WIDTH, COMPASS_HEIGHT, area);

        // Navigation title
        let title = Paragraph::new(Line::from(vec![Span::styled(
            "Choose Direction",
            Style::default().fg(colors::CYAN),
        )]))
        .centered();

        let title_area = Rect {
            height: 1,
            ..content_area
        };
        frame.render_widget(title, title_area);

        // Compass grid: 3 rows (North, West-Center-East, South)
        let compass_y = content_area.y + 2;

        // Row 1: North (centered)
        if has_north {
            let north_area = Rect {
                x: content_area.x + BUTTON_WIDTH + 2,
                y: compass_y,
                width: BUTTON_WIDTH,
                height: BUTTON_HEIGHT,
            };
            self.render_compass_button(frame, north_area, "North", CompassPosition::North, &gs);
        }

        // Row 2: West - Center - East
        let middle_y = compass_y + 2;

        if has_west {
            let west_area = Rect {
                x: content_area.x,
                y: middle_y,
                width: BUTTON_WIDTH,
                height: BUTTON_HEIGHT,
            };
            self.render_compass_button(frame, west_area, "West", CompassPosition::West, &gs);
        }

        // Center (Leave Dungeon) - always available
        let center_area = Rect {
            x: content_area.x + BUTTON_WIDTH + 2,
            y: middle_y,
            width: BUTTON_WIDTH,
            height: BUTTON_HEIGHT,
        };
        self.render_leave_button(frame, center_area);

        if has_east {
            let east_area = Rect {
                x: content_area.x + (BUTTON_WIDTH + 2) * 2,
                y: middle_y,
                width: BUTTON_WIDTH,
                height: BUTTON_HEIGHT,
            };
            self.render_compass_button(frame, east_area, "East", CompassPosition::East, &gs);
        }

        // Row 3: South (centered)
        if has_south {
            let south_area = Rect {
                x: content_area.x + BUTTON_WIDTH + 2,
                y: middle_y + 2,
                width: BUTTON_WIDTH,
                height: BUTTON_HEIGHT,
            };
            self.render_compass_button(frame, south_area, "South", CompassPosition::South, &gs);
        }
    }

    fn render_compass_button(
        &self,
        frame: &mut Frame,
        area: Rect,
        label: &str,
        position: CompassPosition,
        gs: &crate::system::GameState,
    ) {
        let is_selected = self.compass_selection == position;

        // Check if room is cleared
        let direction = match position {
            CompassPosition::North => Direction::North,
            CompassPosition::East => Direction::East,
            CompassPosition::South => Direction::South,
            CompassPosition::West => Direction::West,
            CompassPosition::Center => return, // Should not happen
        };

        let is_cleared = if let Some(dungeon) = gs.dungeon() {
            let (dx, dy) = direction.offset();
            let (px, py) = dungeon.player_position;
            dungeon
                .get_room(px + dx, py + dy)
                .map(|r| r.is_cleared)
                .unwrap_or(false)
        } else {
            false
        };

        let text = if is_cleared {
            format!("{} ✓", label)
        } else {
            label.to_string()
        };

        let style = if is_selected {
            Style::default().fg(colors::YELLOW)
        } else if is_cleared {
            Style::default().fg(colors::DARK_STONE)
        } else {
            Style::default().fg(colors::WHITE)
        };

        let line = Line::from(vec![selection_prefix(is_selected), Span::styled(text, style)]);
        let paragraph = Paragraph::new(line).centered();
        frame.render_widget(paragraph, area);
    }

    fn render_leave_button(&self, frame: &mut Frame, area: Rect) {
        let is_selected = self.compass_selection == CompassPosition::Center;

        let style = if is_selected {
            Style::default().fg(colors::YELLOW)
        } else {
            Style::default().fg(colors::WHITE)
        };

        let line = Line::from(vec![
            selection_prefix(is_selected),
            Span::styled(format!("{} Leave", RETURN_ARROW), style),
        ]);
        let paragraph = Paragraph::new(line).centered();
        frame.render_widget(paragraph, area);
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
