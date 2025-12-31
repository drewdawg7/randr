use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};
use rand::Rng;

use crate::ui::theme::{self as colors, ColorExt};
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State,
};

use crate::combat::IsKillable;
use crate::item::Item;
use crate::mine::rock::RockId;
use crate::system::game_state;
use crate::ui::Id;
use crate::ui::components::utilities::{render_location_header, PICKAXE, RETURN_ARROW};
use crate::ui::components::widgets::stone_border;
use crate::HasInventory;

/// Get color for rock type
fn rock_color(rock_id: RockId) -> ratatui::style::Color {
    match rock_id {
        RockId::Copper => colors::COPPER_ORE,
        RockId::Coal => colors::COAL_BLACK,
        RockId::Tin => colors::TIN_ORE,
    }
}

// HP bar block characters
const BLOCK_FULL: char = '█';
const BLOCK_DARK: char = '▓';
const BLOCK_MED: char = '▒';
const BLOCK_LIGHT: char = '░';

const ROCK_ART: &[&str] = &[
    "        ██████        ",
    "    ████▒▒▒▒▒▒████    ",
    "  ██▒▒░░░░░░░░░░▒▒██  ",
    "  ██▒▒▒▒▒▒▒▒▒▒▓▓▓▓██  ",
    "██▓▓▒▒▒▒▒▒▒▒▓▓▓▓▓▓▓▓██",
    "██▓▓▒▒▒▒▒▒▒▒▓▓████▓▓██",
    "██▓▓▓▓▒▒▒▒▓▓▓▓▓▓▓▓████",
    "  ████▓▓▓▓▓▓▓▓██████  ",
];

pub struct MineScreen {
    props: Props,
    selected_row: usize,    // 0 = mine options, 1 = back button
    selected_mine: usize,   // 0, 1, or 2 for which mine option
    active_button: usize,   // Which mine button is currently active (0, 1, or 2)
    recent_drops: Vec<Item>, // Items dropped from the last rock kill
}

impl MineScreen {
    pub fn new() -> Self {
        Self {
            props: Props::default(),
            selected_row: 0,
            selected_mine: 0,
            active_button: 0,
            recent_drops: Vec::new(),
        }
    }

    /// Pick a new random active button (different from current)
    fn pick_new_active_button(&mut self) {
        let mut rng = rand::thread_rng();
        let mut new_button = rng.gen_range(0..3);
        while new_button == self.active_button {
            new_button = rng.gen_range(0..3);
        }
        self.active_button = new_button;
    }
}

impl Default for MineScreen {
    fn default() -> Self {
        Self::new()
    }
}

fn mine_header() -> Vec<Line<'static>> {
    let gs = game_state();
    let effective_mining = gs.player.get_effective_mining();

    vec![
        Line::from(vec![
            Span::styled("The Village Mine", Style::default().color(colors::GRANITE)),
        ]),
        Line::from(vec![
            Span::styled(format!("{} ", PICKAXE), Style::default().color(colors::LIGHT_STONE)),
            Span::raw(format!("{}", effective_mining)),
        ]),
    ]
}

fn render_hp_bar(current: i32, max: i32) -> Line<'static> {
    let segments: i32 = 10;
    let filled = if max > 0 { ((current * segments) / max).max(0) as usize } else { 0 };
    let hp_percent = if max > 0 { (current * 100) / max } else { 0 };

    let color = if hp_percent > 60 {
        colors::GREEN
    } else if hp_percent > 30 {
        colors::YELLOW
    } else {
        colors::RED
    };

    let mut spans = vec![Span::raw("[")];
    for i in 0..segments as usize {
        let ch = if filled == 0 {
            ' '
        } else if i < filled.saturating_sub(3) {
            BLOCK_FULL
        } else if i == filled.saturating_sub(3) {
            BLOCK_DARK
        } else if i == filled.saturating_sub(2) {
            BLOCK_MED
        } else if i == filled.saturating_sub(1) {
            BLOCK_LIGHT
        } else {
            ' '
        };
        spans.push(Span::styled(ch.to_string(), Style::default().color(color)));
    }
    spans.push(Span::raw("]"));
    Line::from(spans)
}

fn render_hp_text(current: i32, max: i32) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{}/{}", current, max), Style::default().color(colors::LIGHT_STONE)),
    ])
}

impl MockComponent for MineScreen {
    fn view(&mut self, frame: &mut Frame, _area: Rect) {
        // Ensure a rock exists when viewing the mine
        game_state().town.mine.ensure_rock_exists();

        let frame_size = frame.area();

        // Border offsets
        let y_offset: u16 = 1;
        let x_offset: u16 = 1;

        // Reserve space for top border (1) and bottom border (1)
        let content_height = frame_size.height.saturating_sub(2);
        let content_area = Rect {
            x: x_offset,
            y: y_offset,
            width: frame_size.width.saturating_sub(x_offset * 2),
            height: content_height,
        };

        // Fill background with MINE_BG
        let bg_fill = Block::default().style(Style::default().on_color(colors::MINE_BG));
        frame.render_widget(bg_fill, content_area);

        // Render header and get content area
        let header_lines = mine_header();
        let remaining_area = render_location_header(
            frame,
            content_area,
            header_lines,
            colors::MINE_BG,
            colors::GRANITE,
        );

        // Render recent drops on the left side below header
        if !self.recent_drops.is_empty() {
            let gs = game_state();
            let mut drop_lines: Vec<Line> = vec![
                Line::from(Span::styled("Drops:", Style::default().color(colors::YELLOW))),
            ];
            for drop in &self.recent_drops {
                let item_name = gs.get_item_name(drop.kind);
                drop_lines.push(Line::from(Span::styled(
                    format!("  {}", item_name),
                    Style::default().color(colors::WHITE),
                )));
            }
            let drops_area = Rect {
                x: remaining_area.x + 1,
                y: remaining_area.y,
                width: 20,
                height: drop_lines.len() as u16,
            };
            frame.render_widget(Paragraph::new(drop_lines), drops_area);
        }

        // Split remaining area: rock art, HP bar, HP text, horizontal mine options, and back button
        let rock_height = ROCK_ART.len() as u16;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(rock_height + 2), // Rock + padding
                Constraint::Length(1),               // HP bar
                Constraint::Length(1),               // HP text
                Constraint::Length(1),               // Horizontal mine options
                Constraint::Min(1),                  // Back button
            ])
            .split(remaining_area);

        let rock_area = chunks[0];
        let hp_bar_area = chunks[1];
        let hp_text_area = chunks[2];
        let mine_options_area = chunks[3];
        let back_button_area = chunks[4];

        // Render rock art centered with color based on rock type
        let rock_width = ROCK_ART.iter().map(|s| s.chars().count()).max().unwrap_or(0) as u16;
        let rock_x = rock_area.x + rock_area.width.saturating_sub(rock_width) / 2;
        let rock_y = rock_area.y + 1; // 1 line padding from top

        // Get rock color based on current rock type
        let current_rock_color = game_state()
            .town
            .mine
            .current_rock
            .as_ref()
            .map(|r| rock_color(r.rock_id))
            .unwrap_or(colors::GRANITE);

        let rock_lines: Vec<Line> = ROCK_ART
            .iter()
            .map(|line| {
                Line::from(vec![
                    Span::styled(*line, Style::default().color(current_rock_color)),
                ])
            })
            .collect();

        let rock_render_area = Rect {
            x: rock_x,
            y: rock_y,
            width: rock_width,
            height: rock_height,
        };
        frame.render_widget(Paragraph::new(rock_lines), rock_render_area);

        // Render HP bar centered below rock
        let gs = game_state();
        let (hp_bar, hp_text, current_hp, max_hp) = if let Some(rock) = &gs.town.mine.current_rock {
            let current = rock.stats.value(crate::stats::StatType::Health);
            let max = rock.stats.max_value(crate::stats::StatType::Health);
            (render_hp_bar(current, max), render_hp_text(current, max), current, max)
        } else {
            (render_hp_bar(0, 1), render_hp_text(0, 1), 0, 1) // Fallback if no rock
        };
        let hp_bar_width = 12u16; // Width: "[██████████]"
        let hp_bar_x = hp_bar_area.x + hp_bar_area.width.saturating_sub(hp_bar_width) / 2;
        let centered_hp_bar_area = Rect {
            x: hp_bar_x,
            y: hp_bar_area.y,
            width: hp_bar_width,
            height: 1,
        };
        frame.render_widget(Paragraph::new(hp_bar), centered_hp_bar_area);

        // Render HP text centered below HP bar
        let hp_text_str = format!("{}/{}", current_hp, max_hp);
        let hp_text_width = hp_text_str.len() as u16;
        let hp_text_x = hp_text_area.x + hp_text_area.width.saturating_sub(hp_text_width) / 2;
        let centered_hp_text_area = Rect {
            x: hp_text_x,
            y: hp_text_area.y,
            width: hp_text_width,
            height: 1,
        };
        frame.render_widget(Paragraph::new(hp_text), centered_hp_text_area);

        // Render 3 horizontal "Mine" options centered below the rock
        let active_text = format!("{} Mine", PICKAXE);
        let inactive_text = "  X  ";
        let spacing = "  "; // Space between options

        // Build spans with selection indicators
        let mut spans = Vec::new();
        for i in 0..3 {
            let is_selected = self.selected_row == 0 && self.selected_mine == i;
            let is_active = i == self.active_button;
            let prefix = if is_selected { "> " } else { "  " };
            let prefix_style = Style::default().color(colors::YELLOW);

            let (option_text, option_style) = if is_active {
                let style = if is_selected {
                    Style::default().color(colors::WHITE)
                } else {
                    Style::default().color(colors::LIGHT_STONE)
                };
                (active_text.as_str(), style)
            } else {
                let style = if is_selected {
                    Style::default().color(colors::DARK_GRAY)
                } else {
                    Style::default().color(colors::DARK_GRAY)
                };
                (inactive_text, style)
            };

            if i > 0 {
                spans.push(Span::raw(spacing));
            }
            spans.push(Span::styled(prefix, prefix_style));
            spans.push(Span::styled(option_text.to_string(), option_style));
        }
        let horizontal_menu = Line::from(spans);

        // Calculate width for centering (approximate)
        let single_option_width = 2 + active_text.chars().count(); // prefix + option
        let menu_text_width = (single_option_width * 3 + spacing.len() * 2) as u16;
        let menu_x = mine_options_area.x + mine_options_area.width.saturating_sub(menu_text_width) / 2;
        let centered_menu_area = Rect {
            x: menu_x,
            y: mine_options_area.y,
            width: menu_text_width,
            height: 1,
        };
        frame.render_widget(Paragraph::new(horizontal_menu), centered_menu_area);

        // Render the back button centered below
        let back_selected = self.selected_row == 1;
        let back_prefix = if back_selected { "> " } else { "  " };
        let back_text = format!("{} Back", RETURN_ARROW);
        let back_line = Line::from(vec![
            Span::styled(back_prefix, Style::default().color(colors::YELLOW)),
            Span::styled(&back_text, if back_selected {
                Style::default().color(colors::WHITE)
            } else {
                Style::default().color(colors::LIGHT_STONE)
            }),
        ]);

        let back_width = (back_prefix.len() + back_text.chars().count()) as u16;
        let back_x = back_button_area.x + back_button_area.width.saturating_sub(back_width) / 2;
        let centered_back_area = Rect {
            x: back_x,
            y: back_button_area.y,
            width: back_width,
            height: 1,
        };
        frame.render_widget(Paragraph::new(back_line), centered_back_area);

        // Render stone borders
        let total_border_width = content_area.width + 2;
        let bottom_y = frame_size.height.saturating_sub(1);
        let border_height = content_height;

        // Border style with themed background
        let border_style = Style::default().on_color(colors::MINE_BG);

        // Top and bottom borders
        let border_area_top = Rect { x: 0, y: 0, width: total_border_width, height: 1 };
        let border_area_bottom = Rect { x: 0, y: bottom_y, width: total_border_width, height: 1 };

        let top_border = stone_border::generate_top_border(total_border_width);
        let bottom_border = stone_border::generate_bottom_border(total_border_width);
        frame.render_widget(Paragraph::new(top_border).style(border_style), border_area_top);
        frame.render_widget(Paragraph::new(bottom_border).style(border_style), border_area_bottom);

        // Left and right borders
        for row in 0..border_height {
            let left_char = stone_border::generate_left_border_char(row);
            let right_char = stone_border::generate_right_border_char(row);
            let left_area = Rect { x: 0, y: y_offset + row, width: 1, height: 1 };
            let right_area = Rect { x: x_offset + content_area.width, y: y_offset + row, width: 1, height: 1 };
            frame.render_widget(Paragraph::new(Line::from(left_char)).style(border_style), left_area);
            frame.render_widget(Paragraph::new(Line::from(right_char)).style(border_style), right_area);
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
            Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                if self.selected_row > 0 {
                    self.selected_row -= 1;
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                if self.selected_row < 1 {
                    self.selected_row += 1;
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Left, .. }) => {
                if self.selected_row == 0 && self.selected_mine > 0 {
                    self.selected_mine -= 1;
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Right, .. }) => {
                if self.selected_row == 0 && self.selected_mine < 2 {
                    self.selected_mine += 1;
                }
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                if self.selected_row == 1 {
                    // Back button selected
                    game_state().current_screen = Id::Town;
                } else if self.selected_row == 0 && self.selected_mine == self.active_button {
                    // Active mine button selected - perform mining
                    let gs = game_state();
                    let mining_power = gs.player.get_effective_mining();

                    // Take the rock temporarily to check death
                    if let Some(mut rock) = gs.town.mine.current_rock.take() {
                        rock.take_damage(mining_power);

                        if !rock.is_alive() {
                            // Rock died - roll drops and add to inventory
                            let drops = rock.roll_drops();
                            self.recent_drops = drops.clone();
                            for drop in drops {
                                let _ = gs.player.add_to_inv(drop);
                            }
                            // Spawn a new rock
                            gs.town.mine.spawn_rock();
                        } else {
                            // Rock still alive - put it back
                            gs.town.mine.current_rock = Some(rock);
                        }
                    }

                    // Pick a new random active button
                    self.pick_new_active_button();
                }
                None
            }
            _ => None
        }
    }
}
