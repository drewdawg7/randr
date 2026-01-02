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
    combat::{self, Combatant, DealsDamage, HasGold, IsKillable, Named},
    dungeon::{Direction, RoomType},
    entities::{mob::MobId, progression::HasProgression},
    inventory::HasInventory,
    stats::HasStats,
    system::{game_state, CombatSource},
    ui::{
        components::{
            backgrounds::render_stone_wall,
            dungeon::{campfire_art::render_campfire_art, dragon_art, minimap},
            utilities::{selection_prefix, HEART, RETURN_ARROW},
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
    /// Player is in a rest room (can heal)
    RestRoom,
    /// Player is in the boss room (trapped until defeated)
    BossRoom,
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
            DungeonState::RestRoom => 2, // Rest/Heal + Leave
            DungeonState::BossRoom => 1, // Attack only (no escape!)
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
            // Rest rooms go to RestRoom state, others go to Navigation
            if room_type == Some(RoomType::Rest) {
                self.state = DungeonState::RestRoom;
            } else {
                self.state = DungeonState::Navigation;
            }
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
            RoomType::Boss => {
                // Boss room - spawn boss once and transition to BossRoom state (trapped!)
                let needs_spawn = gs.dungeon().map(|d| d.boss.is_none()).unwrap_or(false);
                if needs_spawn {
                    // Spawn the dragon boss
                    let dragon = gs.spawn_mob(MobId::Dragon);
                    if let Some(dungeon) = gs.dungeon_mut() {
                        dungeon.boss = Some(dragon);
                    }
                }
                self.boss_combat_log.clear();
                self.state = DungeonState::BossRoom;
                self.reset_selection();
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
                // Check if entering a boss room - go directly to boss fight
                let is_boss_room = dungeon
                    .current_room()
                    .map(|r| r.room_type == RoomType::Boss && !r.is_cleared)
                    .unwrap_or(false);

                if is_boss_room {
                    // Spawn boss if needed
                    if dungeon.boss.is_none() {
                        let dragon = gs.spawn_mob(MobId::Dragon);
                        if let Some(d) = gs.dungeon_mut() {
                            d.boss = Some(dragon);
                        }
                    }
                    self.boss_combat_log.clear();
                    self.state = DungeonState::BossRoom;
                } else {
                    self.state = DungeonState::RoomEntry;
                }
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

        // Reset screen state if entering a fresh dungeon
        // A fresh dungeon has only 1 visited room (the start) and is at start position
        if let Some(dungeon) = gs.dungeon() {
            let visited_count = dungeon.rooms.iter()
                .flat_map(|row| row.iter())
                .filter_map(|r| r.as_ref())
                .filter(|r| r.is_visited)
                .count();

            // Fresh dungeon: only start room visited, and we're in a non-RoomEntry state
            // This happens when returning after death with stale screen state
            if visited_count == 1 && self.state != DungeonState::RoomEntry {
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
                // Boss room cleared - transition to Navigation
                if room.is_cleared && self.state == DungeonState::BossRoom {
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
            DungeonState::RestRoom => self.render_rest_room(frame, chunks[1]),
            DungeonState::BossRoom => self.render_boss_room(frame, chunks[1]),
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
                    DungeonState::RoomEntry => self.handle_room_entry_submit(),
                    DungeonState::Navigation => self.handle_navigation_submit(),
                    DungeonState::RestRoom => self.handle_rest_submit(),
                    DungeonState::BossRoom => self.handle_boss_submit(),
                }
                CmdResult::Submit(tuirealm::State::None)
            }
            Cmd::Cancel => {
                // ESC to leave dungeon (but not from boss room!)
                if self.state != DungeonState::BossRoom {
                    game_state().leave_dungeon();
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

    fn render_rest_room(&self, frame: &mut Frame, area: Rect) {
        use crate::ui::components::dungeon::campfire_art::campfire_width;

        let gs = game_state();
        let player = &gs.player;

        // Get campfire art
        let campfire_lines = render_campfire_art();
        let campfire_height = campfire_lines.len() as u16;
        let campfire_w = campfire_width();

        // HP bar constants
        const HP_BAR_WIDTH: u16 = 20;

        // Calculate total content height: campfire + spacing + title + HP bar + spacing + menu
        let total_height = campfire_height + 1 + 1 + 1 + 1 + 2; // campfire + gap + title + hp + gap + 2 menu items

        // Center everything vertically
        let y_offset = area.y + area.height.saturating_sub(total_height) / 2;

        // Center campfire horizontally
        let campfire_x = area.x + area.width.saturating_sub(campfire_w) / 2;

        // Render campfire lines directly to buffer, skipping spaces to preserve background
        let buf = frame.buffer_mut();
        for (i, line) in campfire_lines.into_iter().enumerate() {
            let y = y_offset + i as u16;
            if y >= area.y + area.height {
                break;
            }
            // Render each span, skipping space characters to preserve background
            let mut x = campfire_x;
            for span in line.spans {
                for ch in span.content.chars() {
                    if x >= area.x + area.width {
                        break;
                    }
                    if ch != ' ' {
                        buf.set_string(x, y, ch.to_string(), span.style);
                    }
                    x += 1;
                }
            }
        }

        // Title "Rest Area" centered below campfire
        let title_y = y_offset + campfire_height + 1;
        let title = Paragraph::new(Line::from(vec![Span::styled(
            "Rest Area",
            Style::default().fg(colors::CYAN),
        )]))
        .centered();
        let title_area = Rect {
            x: area.x,
            y: title_y,
            width: area.width,
            height: 1,
        };
        frame.render_widget(title, title_area);

        // HP bar below title
        let hp = player.hp();
        let max_hp = player.max_hp();
        let hp_percent = if max_hp > 0 {
            (hp as f32 / max_hp as f32 * 100.0) as u16
        } else {
            100
        };

        let hp_color = if hp_percent > 60 {
            colors::GREEN
        } else if hp_percent > 30 {
            colors::YELLOW
        } else {
            colors::RED
        };

        // Create HP bar: [████████░░░░░░░░░░░░] 81/100
        let filled_chars = ((HP_BAR_WIDTH as f32) * (hp as f32 / max_hp as f32)).round() as u16;
        let empty_chars = HP_BAR_WIDTH.saturating_sub(filled_chars);
        let filled_bar = "█".repeat(filled_chars as usize);
        let empty_bar = "░".repeat(empty_chars as usize);

        let hp_bar_line = Line::from(vec![
            Span::styled(format!("{} ", HEART), Style::default().fg(colors::RED)),
            Span::styled("[", Style::default().fg(colors::WHITE)),
            Span::styled(filled_bar, Style::default().fg(hp_color)),
            Span::styled(empty_bar, Style::default().fg(colors::DARK_STONE)),
            Span::styled("] ", Style::default().fg(colors::WHITE)),
            Span::styled(format!("{}/{}", hp, max_hp), Style::default().fg(hp_color)),
        ]);

        let hp_y = title_y + 1;
        let hp_area = Rect {
            x: area.x,
            y: hp_y,
            width: area.width,
            height: 1,
        };
        frame.render_widget(Paragraph::new(hp_bar_line).centered(), hp_area);

        // Menu below HP bar
        let heal_amount = (max_hp as f32 * 0.5).round() as i32;
        let can_heal = hp < max_hp;

        let rest_style = if self.rest_selection == 0 {
            Style::default().fg(colors::YELLOW)
        } else if can_heal {
            Style::default().fg(colors::GREEN)
        } else {
            Style::default().fg(colors::DARK_STONE)
        };

        let leave_style = if self.rest_selection == 1 {
            Style::default().fg(colors::YELLOW)
        } else {
            Style::default().fg(colors::WHITE)
        };

        let rest_text = if can_heal {
            format!("Rest (+{} HP)", heal_amount.min(max_hp - hp))
        } else {
            "Rest (Full HP)".to_string()
        };

        let menu_y = hp_y + 2;
        let menu_width: u16 = 20;
        let menu_x = area.x + area.width.saturating_sub(menu_width) / 2;

        let menu_items: Vec<ListItem> = vec![
            ListItem::new(Line::from(vec![
                selection_prefix(self.rest_selection == 0),
                Span::styled(rest_text, rest_style),
            ])),
            ListItem::new(Line::from(vec![
                selection_prefix(self.rest_selection == 1),
                Span::styled(format!("{} Continue", RETURN_ARROW), leave_style),
            ])),
        ];

        let menu_area = Rect {
            x: menu_x,
            y: menu_y,
            width: menu_width,
            height: 2,
        };
        let menu = List::new(menu_items);
        frame.render_widget(menu, menu_area);
    }

    fn handle_rest_submit(&mut self) {
        let gs = game_state();

        match self.rest_selection {
            0 => {
                // Rest/Heal: restore 50% of max HP
                let player = &mut gs.player;
                let max_hp = player.max_hp();
                let current_hp = player.hp();

                if current_hp < max_hp {
                    let heal_amount = (max_hp as f32 * 0.5).round() as i32;
                    let actual_heal = heal_amount.min(max_hp - current_hp);
                    player.increase_health(actual_heal);
                    gs.toasts.success(format!("Rested and recovered {} HP!", actual_heal));
                } else {
                    gs.toasts.info("Already at full health!");
                }
            }
            1 => {
                // Continue: go to navigation
                self.state = DungeonState::Navigation;
                self.reset_selection();
            }
            _ => {}
        }
    }

    fn render_boss_room(&self, frame: &mut Frame, area: Rect) {
        let gs = game_state();

        // Get dragon art
        let dragon_lines = dragon_art::render_dragon_art();
        let dragon_height = dragon_art::DRAGON_HEIGHT;
        let dragon_w = dragon_art::DRAGON_WIDTH;

        // HP bar constants
        const HP_BAR_WIDTH: u16 = 30;

        // Get boss HP from stored boss (or show 0 if dead/missing)
        let (hp, max_hp) = if let Some(dungeon) = gs.dungeon() {
            if let Some(boss) = &dungeon.boss {
                (boss.hp(), boss.max_hp())
            } else {
                (0, 1)
            }
        } else {
            (0, 1)
        };

        // Calculate total content height
        let combat_log_height = self.boss_combat_log.len().min(3) as u16;
        let total_height = dragon_height + 1 + 1 + 1 + 1 + combat_log_height + 2;

        // Center everything vertically
        let y_offset = area.y + area.height.saturating_sub(total_height) / 2;

        // Center dragon horizontally
        let dragon_x = area.x + area.width.saturating_sub(dragon_w) / 2;

        // Render dragon lines directly to buffer, skipping spaces to preserve background
        let buf = frame.buffer_mut();
        for (i, line) in dragon_lines.into_iter().enumerate() {
            let y = y_offset + i as u16;
            if y >= area.y + area.height {
                break;
            }
            let mut x = dragon_x;
            for span in line.spans {
                for ch in span.content.chars() {
                    if x >= area.x + area.width {
                        break;
                    }
                    if ch != ' ' {
                        buf.set_string(x, y, ch.to_string(), span.style);
                    }
                    x += 1;
                }
            }
        }

        // Title "DRAGON" centered below dragon
        let title_y = y_offset + dragon_height + 1;
        let title = Paragraph::new(Line::from(vec![Span::styled(
            "DRAGON",
            Style::default().fg(colors::EMBER_RED),
        )]))
        .centered();
        let title_area = Rect {
            x: area.x,
            y: title_y,
            width: area.width,
            height: 1,
        };
        frame.render_widget(title, title_area);

        // Boss HP bar
        let hp_percent = if max_hp > 0 {
            (hp as f32 / max_hp as f32 * 100.0) as u16
        } else {
            0
        };
        let hp_color = if hp_percent > 60 {
            colors::EMBER_RED
        } else if hp_percent > 30 {
            colors::FLAME_ORANGE
        } else {
            colors::BRIGHT_YELLOW
        };

        let filled_chars = ((HP_BAR_WIDTH as f32) * (hp as f32 / max_hp as f32)).round() as u16;
        let empty_chars = HP_BAR_WIDTH.saturating_sub(filled_chars);
        let filled_bar = "█".repeat(filled_chars as usize);
        let empty_bar = "░".repeat(empty_chars as usize);

        let hp_bar_line = Line::from(vec![
            Span::styled(format!("{} ", HEART), Style::default().fg(colors::EMBER_RED)),
            Span::styled("[", Style::default().fg(colors::WHITE)),
            Span::styled(filled_bar, Style::default().fg(hp_color)),
            Span::styled(empty_bar, Style::default().fg(colors::DARK_STONE)),
            Span::styled("] ", Style::default().fg(colors::WHITE)),
            Span::styled(format!("{}/{}", hp, max_hp), Style::default().fg(hp_color)),
        ]);

        let hp_y = title_y + 1;
        let hp_area = Rect {
            x: area.x,
            y: hp_y,
            width: area.width,
            height: 1,
        };
        frame.render_widget(Paragraph::new(hp_bar_line).centered(), hp_area);

        // Combat log (last few messages)
        let log_y = hp_y + 2;
        for (i, msg) in self.boss_combat_log.iter().rev().take(3).rev().enumerate() {
            let log_line = Paragraph::new(Line::from(vec![Span::styled(
                msg.clone(),
                Style::default().fg(colors::WHITE),
            )]))
            .centered();
            let log_area = Rect {
                x: area.x,
                y: log_y + i as u16,
                width: area.width,
                height: 1,
            };
            frame.render_widget(log_line, log_area);
        }

        // Attack menu below combat log
        let attack_style = Style::default().fg(colors::YELLOW);
        let menu_y = log_y + combat_log_height.max(1) + 1;
        let menu_width: u16 = 20;
        let menu_x = area.x + area.width.saturating_sub(menu_width) / 2;

        let menu_items: Vec<ListItem> = vec![ListItem::new(Line::from(vec![
            selection_prefix(true),
            Span::styled("Attack", attack_style),
        ]))];

        let menu_area = Rect {
            x: menu_x,
            y: menu_y,
            width: menu_width,
            height: 1,
        };
        let menu = List::new(menu_items);
        frame.render_widget(menu, menu_area);
    }

    fn handle_boss_submit(&mut self) {
        let gs = game_state();

        // Check if boss is alive
        let boss_alive = gs
            .dungeon()
            .and_then(|d| d.boss.as_ref())
            .map(|b| b.is_alive())
            .unwrap_or(false);

        if !boss_alive {
            return;
        }

        // Get player attack stats
        let player_attack = gs.player.get_attack();

        // Player attacks boss
        let (player_damage, boss_died) = {
            if let Some(dungeon) = gs.dungeon_mut() {
                if let Some(boss) = dungeon.boss.as_mut() {
                    let raw_damage = player_attack.roll_damage();
                    let defense = boss.effective_defense();
                    let damage = combat::apply_defense(raw_damage, defense);
                    boss.take_damage(damage);
                    let died = !boss.is_alive();
                    (damage, died)
                } else {
                    (0, true)
                }
            } else {
                (0, true)
            }
        };

        self.boss_combat_log
            .push(format!("You deal {} damage to Dragon!", player_damage));

        if boss_died {
            // Victory! Get death rewards from boss
            let death_result = {
                if let Some(dungeon) = gs.dungeon_mut() {
                    dungeon.boss.as_mut().map(|boss| boss.on_death())
                } else {
                    None
                }
            };

            if let Some(death_result) = death_result {
                // Apply gold with goldfind bonus
                let gf = gs.player.effective_goldfind();
                let multiplier = 1.0 + (gf as f64 / 100.0);
                let gold_with_bonus =
                    ((death_result.gold_dropped as f64) * multiplier).round() as i32;
                gs.player.add_gold(gold_with_bonus);

                // Award XP
                gs.player.gain_xp(death_result.xp_dropped);

                // Add loot to inventory
                for loot in &death_result.loot_drops {
                    for _ in 0..loot.quantity {
                        let _ = gs.player.add_to_inv(loot.item.clone());
                    }
                    gs.toasts
                        .success(format!("Obtained: {} x{}", loot.item.name, loot.quantity));
                }

                gs.toasts.success(format!(
                    "Dragon defeated! +{} gold, +{} XP",
                    gold_with_bonus, death_result.xp_dropped
                ));
            }

            // Clear the boss room
            if let Some(dungeon) = gs.dungeon_mut() {
                if let Some(room) = dungeon.current_room_mut() {
                    room.clear();
                }
                dungeon.boss = None;
            }

            self.boss_combat_log
                .push("Dragon has been slain!".to_string());
            self.state = DungeonState::Navigation;
            self.reset_selection();
            return;
        }

        // Boss attacks player (if boss still alive)
        let (boss_attack_range, boss_name) = {
            if let Some(dungeon) = gs.dungeon() {
                if let Some(boss) = &dungeon.boss {
                    (Some(boss.get_attack()), boss.name().to_string())
                } else {
                    (None, String::new())
                }
            } else {
                (None, String::new())
            }
        };

        if let Some(attack_range) = boss_attack_range {
            let raw_damage = attack_range.roll_damage();
            let defense = gs.player.effective_defense();
            let damage = combat::apply_defense(raw_damage, defense);
            gs.player.take_damage(damage);

            self.boss_combat_log
                .push(format!("{} deals {} damage to you!", boss_name, damage));

            // Check if player died
            if !gs.player.is_alive() {
                // Player died - process death and kick out of dungeon
                gs.player.on_death();
                gs.toasts.error("You were slain by the Dragon!");
                gs.reset_dungeon();
                gs.leave_dungeon();
            }
        }
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
