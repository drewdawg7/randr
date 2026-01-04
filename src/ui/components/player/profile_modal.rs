//! Player profile modal - displays player stats, progression, and resources
//! in a parchment-styled fantasy book aesthetic.

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    Frame,
};
use tuirealm::event::Key;

use crate::{
    combat::{Combatant, DealsDamage, HasGold, Named},
    entities::progression::{HasProgression, Progression},
    inventory::HasInventory,
    magic::effect::PassiveEffect,
    player::Player,
    stats::{HasStats, StatType},
    system::game_state,
    ui::components::utilities::{COIN, CROSSED_SWORDS, HEART, PICKAXE, SHIELD},
    ui::components::widgets::selection::ListSelection,
    ui::theme::{CREAM_WOOD, DARK_WALNUT, GREEN, LIGHT_BEIGE, OAK_BROWN, TAN_WOOD, WHITE, WOOD_BROWN, YELLOW},
};

// Parchment background color (warm tan/beige tint)
const PARCHMENT_BG: Color = Color::Rgb(58, 52, 46);

// Border patterns for parchment/book aesthetic
const TOP_PATTERN: &str = "▄█▓▒░▒▓█";
const BOTTOM_PATTERN: &str = "▀█▓▒░▒▓█";
const LEFT_PATTERN: &[char] = &['║', '┃', '│', '┆', '┊'];
const RIGHT_PATTERN: &[char] = &['║', '┃', '│', '┆', '┊'];

// Parchment colors cycling for border characters
const PARCHMENT_COLORS: [Color; 9] = [
    DARK_WALNUT,
    WOOD_BROWN,
    OAK_BROWN,
    TAN_WOOD,
    LIGHT_BEIGE,
    CREAM_WOOD,
    LIGHT_BEIGE,
    TAN_WOOD,
    OAK_BROWN,
];

/// Parchment texture pattern using MEDIUM density characters
/// Characters like ~, *, :, %, # create visible paper fiber texture
const PARCHMENT_PATTERN: &[&str] = &[
    "~:~*~:~~:*~:~*~~:~*~:~~:*~:~*~",
    ":*:~:*::~*:~:*::*:~:*::~*:~:*:",
    "*~~:*~~*:~~:*~~*~~:*~~*:~~:*~~",
    "~:*~:~*~:*~~:*~:~*~:*~~:*~:~*~",
    ":~~*:~~:*~::~~*:~~:*~::~~*:~~:",
    "*:~*:*~*:~**:~*:*~*:~**:~*:*~*",
    "~*:~~*:~:*~~*:~~*:~:*~~*:~~*:~",
    ":~*::~*:*~::~*::~*:*~::~*::~*:",
    "*~:~*~:~~:*~:~*~:~~:*~:~*~:~~:",
    "~:~~:~*~:*~~:~~:~*~:*~~:~~:~*~",
];

const PARCHMENT_PATTERN_WIDTH: usize = 30;
const PARCHMENT_PATTERN_HEIGHT: usize = 10;

/// A modal displaying the player's profile with stats and passive effects.
/// Shows effective stats, gold, progression, and scrollable passive effects list.
pub struct ProfileModal {
    /// Selection state for passive effects list
    selection: ListSelection,
}

impl ProfileModal {
    pub fn new() -> Self {
        Self {
            selection: ListSelection::new(0),
        }
    }

    /// Reset state when modal opens
    pub fn reset(&mut self) {
        self.selection.reset();
    }

    /// Render the profile modal
    pub fn render(&mut self, frame: &mut Frame) {
        let frame_area = frame.area();

        // Calculate modal dimensions (wider for two-column layout)
        let modal_width = (frame_area.width * 75 / 100).min(80).max(60);
        let modal_height = (frame_area.height * 65 / 100).min(24).max(16);

        let x = (frame_area.width.saturating_sub(modal_width)) / 2;
        let y = (frame_area.height.saturating_sub(modal_height)) / 2;

        let border_area = Rect::new(x, y, modal_width, modal_height);

        // Render parchment background first
        self.render_parchment_background(frame, border_area);

        // Render the ASCII border on top
        self.render_ascii_border(frame, border_area);

        // Calculate inner area for content
        let inner_area = Rect::new(
            x + 2,
            y + 1,
            modal_width.saturating_sub(4),
            modal_height.saturating_sub(2),
        );

        // Render content directly to buffer (preserves background)
        self.render_content_to_buffer(frame, inner_area);
    }

    /// Render parchment texture background
    fn render_parchment_background(&self, frame: &mut Frame, area: Rect) {
        let buf = frame.buffer_mut();

        // Parchment fiber colors - warm tans and creams
        let fiber_colors = [
            Color::Rgb(70, 60, 50),   // Dark fibers
            Color::Rgb(85, 72, 58),   // Medium fibers
            Color::Rgb(95, 80, 65),   // Light fibers
        ];

        for row in 0..area.height {
            let pattern_row = (row as usize) % PARCHMENT_PATTERN_HEIGHT;
            let pattern_chars: Vec<char> = PARCHMENT_PATTERN[pattern_row].chars().collect();

            for col in 0..area.width {
                let pattern_col = (col as usize) % PARCHMENT_PATTERN_WIDTH;
                let ch = pattern_chars.get(pattern_col).copied().unwrap_or(' ');

                let cell_x = area.x + col;
                let cell_y = area.y + row;

                if let Some(cell) = buf.cell_mut((cell_x, cell_y)) {
                    cell.set_bg(PARCHMENT_BG);
                    cell.set_char(ch);
                    // Vary color based on position for depth
                    let color_idx = ((col as usize / 4) + (row as usize / 2)) % fiber_colors.len();
                    cell.set_fg(fiber_colors[color_idx]);
                }
            }
        }
    }

    /// Render border directly to buffer (preserves parchment background in middle)
    fn render_ascii_border(&self, frame: &mut Frame, area: Rect) {
        let buf = frame.buffer_mut();
        let width = area.width as usize;
        let height = area.height;

        // Top border with title
        let title = " Character Profile ";
        let title_start = (width.saturating_sub(title.len())) / 2;
        let title_chars: Vec<char> = title.chars().collect();
        let top_pattern_chars: Vec<char> = TOP_PATTERN.chars().collect();

        for col in 0..width {
            let x = area.x + col as u16;
            let y = area.y;

            if let Some(cell) = buf.cell_mut((x, y)) {
                if col >= title_start && col < title_start + title.len() {
                    let ch = title_chars.get(col - title_start).copied().unwrap_or(' ');
                    cell.set_char(ch);
                    cell.set_fg(CREAM_WOOD);
                } else {
                    let ch = top_pattern_chars.get(col % top_pattern_chars.len()).copied().unwrap_or(' ');
                    cell.set_char(ch);
                    cell.set_fg(PARCHMENT_COLORS[col % PARCHMENT_COLORS.len()]);
                }
                cell.set_bg(PARCHMENT_BG);
            }
        }

        // Side borders only (don't fill middle - preserve parchment)
        for row in 1..height.saturating_sub(1) {
            let left_ch = LEFT_PATTERN[row as usize % LEFT_PATTERN.len()];
            let right_ch = RIGHT_PATTERN[row as usize % RIGHT_PATTERN.len()];
            let left_color = PARCHMENT_COLORS[row as usize % PARCHMENT_COLORS.len()];
            let right_color = PARCHMENT_COLORS[(row as usize + 3) % PARCHMENT_COLORS.len()];

            let y = area.y + row;

            // Left border
            if let Some(cell) = buf.cell_mut((area.x, y)) {
                cell.set_char(left_ch);
                cell.set_fg(left_color);
                cell.set_bg(PARCHMENT_BG);
            }

            // Right border
            let right_x = area.x + area.width - 1;
            if let Some(cell) = buf.cell_mut((right_x, y)) {
                cell.set_char(right_ch);
                cell.set_fg(right_color);
                cell.set_bg(PARCHMENT_BG);
            }
        }

        // Bottom border
        let bottom_pattern_chars: Vec<char> = BOTTOM_PATTERN.chars().collect();
        let bottom_y = area.y + height - 1;

        for col in 0..width {
            let x = area.x + col as u16;

            if let Some(cell) = buf.cell_mut((x, bottom_y)) {
                let ch = bottom_pattern_chars.get(col % bottom_pattern_chars.len()).copied().unwrap_or(' ');
                cell.set_char(ch);
                cell.set_fg(PARCHMENT_COLORS[col % PARCHMENT_COLORS.len()]);
                cell.set_bg(PARCHMENT_BG);
            }
        }
    }

    /// Render content directly to buffer to preserve parchment background
    fn render_content_to_buffer(&mut self, frame: &mut Frame, area: Rect) {
        let player = &game_state().player;

        // Split area into two columns with separator
        let left_width = area.width / 2;
        let right_width = area.width - left_width - 1; // -1 for separator

        let left_area = Rect::new(area.x, area.y, left_width, area.height);
        let separator_x = area.x + left_width;
        let right_area = Rect::new(separator_x + 1, area.y, right_width, area.height);

        // Render vertical separator
        Self::render_separator(frame, separator_x, area.y, area.height);

        // Render existing stats in left column
        Self::render_stats_column(frame, left_area, player);

        // Render passive effects in right column
        self.render_passive_column(frame, right_area, player);
    }

    /// Render a vertical separator line
    fn render_separator(frame: &mut Frame, x: u16, start_y: u16, height: u16) {
        let buf = frame.buffer_mut();
        for row in 0..height {
            if let Some(cell) = buf.cell_mut((x, start_y + row)) {
                cell.set_char('│');
                cell.set_fg(TAN_WOOD);
            }
        }
    }

    /// Render the stats column (left side)
    fn render_stats_column(frame: &mut Frame, area: Rect, player: &Player) {
        // Gather player data
        let name = player.name();
        let level = player.level();
        let health = player.hp();
        let max_health = player.max_hp();
        let gold = player.gold();

        // Effective stats (base + equipment)
        let attack = player.effective_attack();
        let defense = player.effective_defense();
        let goldfind = player.effective_goldfind();
        let magicfind = player.effective_magicfind();
        let mining = player.effective_mining();

        // Equipment bonuses for display
        let attack_bonus = player.equipment_attack_bonus();
        let defense_bonus = player.inventory().sum_equipment_stats(StatType::Defense);

        // XP progress
        let progression = player.progression();
        let xp = progression.xp;
        let xp_needed = Progression::xp_to_next_level(level);

        // Build content lines
        let mut lines: Vec<Line> = Vec::new();

        // Name and Level
        lines.push(Line::from(vec![
            Span::styled(format!("{}", name), Style::default().fg(CREAM_WOOD)),
            Span::styled("  ", Style::default().fg(WHITE)),
            Span::styled(format!("Lv. {}", level), Style::default().fg(WHITE)),
        ]));

        // Separator
        lines.push(Line::from(Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(TAN_WOOD),
        )));

        // Health
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", HEART), Style::default().fg(Color::Rgb(244, 67, 54))),
            Span::styled("Health:  ", Style::default().fg(CREAM_WOOD)),
            Span::styled(format!("{}/{}", health, max_health), Style::default().fg(WHITE)),
        ]));

        // Attack with equipment bonus
        let attack_line = if attack_bonus > 0 {
            Line::from(vec![
                Span::styled(format!("{} ", CROSSED_SWORDS), Style::default().fg(WHITE)),
                Span::styled("Attack:  ", Style::default().fg(CREAM_WOOD)),
                Span::styled(format!("{}", attack), Style::default().fg(WHITE)),
                Span::styled(format!(" (+{})", attack_bonus), Style::default().fg(GREEN)),
            ])
        } else {
            Line::from(vec![
                Span::styled(format!("{} ", CROSSED_SWORDS), Style::default().fg(WHITE)),
                Span::styled("Attack:  ", Style::default().fg(CREAM_WOOD)),
                Span::styled(format!("{}", attack), Style::default().fg(WHITE)),
            ])
        };
        lines.push(attack_line);

        // Defense with equipment bonus
        let defense_line = if defense_bonus > 0 {
            Line::from(vec![
                Span::styled(format!("{} ", SHIELD), Style::default().fg(WHITE)),
                Span::styled("Defense: ", Style::default().fg(CREAM_WOOD)),
                Span::styled(format!("{}", defense), Style::default().fg(WHITE)),
                Span::styled(format!(" (+{})", defense_bonus), Style::default().fg(GREEN)),
            ])
        } else {
            Line::from(vec![
                Span::styled(format!("{} ", SHIELD), Style::default().fg(WHITE)),
                Span::styled("Defense: ", Style::default().fg(CREAM_WOOD)),
                Span::styled(format!("{}", defense), Style::default().fg(WHITE)),
            ])
        };
        lines.push(defense_line);

        // Separator
        lines.push(Line::from(Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(TAN_WOOD),
        )));

        // Gold
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", COIN), Style::default().fg(Color::Rgb(255, 204, 0))),
            Span::styled("Gold:      ", Style::default().fg(CREAM_WOOD)),
            Span::styled(format!("{}", gold), Style::default().fg(WHITE)),
        ]));

        // GoldFind
        lines.push(Line::from(vec![
            Span::styled("✧ ", Style::default().fg(Color::Rgb(255, 204, 0))),
            Span::styled("GoldFind:  ", Style::default().fg(CREAM_WOOD)),
            Span::styled(format!("+{}%", goldfind), Style::default().fg(WHITE)),
        ]));

        // MagicFind
        lines.push(Line::from(vec![
            Span::styled("✦ ", Style::default().fg(Color::Rgb(180, 100, 255))),
            Span::styled("MagicFind: ", Style::default().fg(CREAM_WOOD)),
            Span::styled(format!("+{}%", magicfind), Style::default().fg(WHITE)),
        ]));

        // Mining
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", PICKAXE), Style::default().fg(WHITE)),
            Span::styled("Mining:    ", Style::default().fg(CREAM_WOOD)),
            Span::styled(format!("{}", mining), Style::default().fg(WHITE)),
        ]));

        // Separator
        lines.push(Line::from(Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(TAN_WOOD),
        )));

        // XP Bar
        let xp_bar_width: usize = 10;
        let filled = if xp_needed > 0 {
            ((xp as f32 / xp_needed as f32) * xp_bar_width as f32) as usize
        } else {
            0
        };
        let empty = xp_bar_width.saturating_sub(filled);

        lines.push(Line::from(vec![
            Span::styled("XP: ", Style::default().fg(CREAM_WOOD)),
            Span::styled("[", Style::default().fg(WHITE)),
            Span::styled("█".repeat(filled), Style::default().fg(GREEN)),
            Span::styled("░".repeat(empty), Style::default().fg(TAN_WOOD)),
            Span::styled("] ", Style::default().fg(WHITE)),
            Span::styled(format!("{}/{}", xp, xp_needed), Style::default().fg(WHITE)),
        ]));

        // Empty line and close hint with navigation
        lines.push(Line::from(Span::raw("")));
        lines.push(Line::from(Span::styled(
            "[p/Esc] close",
            Style::default().fg(TAN_WOOD),
        )));
        lines.push(Line::from(Span::styled(
            "[↑/↓] scroll",
            Style::default().fg(TAN_WOOD),
        )));

        // Render directly to buffer, skipping spaces to preserve background
        let buf = frame.buffer_mut();
        for (i, line) in lines.iter().enumerate() {
            let y = area.y + i as u16;
            if y >= area.y + area.height {
                break;
            }

            let mut x = area.x;
            for span in line.spans.iter() {
                let has_style = span.style.fg.is_some();
                for ch in span.content.chars() {
                    if x >= area.x + area.width {
                        break;
                    }

                    if let Some(cell) = buf.cell_mut((x, y)) {
                        // Skip spaces in unstyled spans to preserve background
                        if ch == ' ' && !has_style {
                            x += 1;
                            continue;
                        }
                        cell.set_char(ch);
                        if let Some(fg) = span.style.fg {
                            cell.set_fg(fg);
                        }
                        // Don't set background - preserve parchment
                    }
                    x += 1;
                }
            }
        }
    }

    /// Render the passive effects column (right side)
    fn render_passive_column(&mut self, frame: &mut Frame, area: Rect, player: &Player) {
        let passive_effects: Vec<(&str, &PassiveEffect)> = player.tome_passive_effects_with_names();

        // Update selection count
        self.selection.set_count(passive_effects.len());

        let buf = frame.buffer_mut();
        let mut y = area.y;

        // Header
        Self::render_line_to_buffer(buf, area.x, y, area.width,
            &[Span::styled("Passive Effects", Style::default().fg(CREAM_WOOD))]);
        y += 1;

        // Separator line
        Self::render_line_to_buffer(buf, area.x, y, area.width,
            &[Span::styled("─".repeat(area.width as usize), Style::default().fg(TAN_WOOD))]);
        y += 1;

        if passive_effects.is_empty() {
            // Empty state
            Self::render_line_to_buffer(buf, area.x, y, area.width,
                &[Span::styled("No active passive", Style::default().fg(TAN_WOOD))]);
            y += 1;
            Self::render_line_to_buffer(buf, area.x, y, area.width,
                &[Span::styled("effects", Style::default().fg(TAN_WOOD))]);
        } else {
            // Calculate visible window for scrolling
            let max_visible = (area.height.saturating_sub(6)) as usize; // Reserve for header, separator, desc
            let selected = self.selection.selected();
            let start_idx = Self::calculate_scroll_offset(selected, passive_effects.len(), max_visible);

            // Render visible effects
            for (i, (name, _effect)) in passive_effects.iter().enumerate().skip(start_idx).take(max_visible) {
                let is_selected = i == selected;
                let prefix = if is_selected { "> " } else { "  " };
                let style = if is_selected {
                    Style::default().fg(YELLOW)
                } else {
                    Style::default().fg(WHITE)
                };

                Self::render_line_to_buffer(buf, area.x, y, area.width,
                    &[Span::styled(format!("{}{}", prefix, name), style)]);
                y += 1;
            }

            // Leave a blank line
            y += 1;

            // Inline description for selected effect
            if let Some((_name, effect)) = passive_effects.get(selected) {
                let description = effect.describe();
                // Word-wrap description if needed
                let desc_lines = Self::wrap_text(&description, area.width as usize);
                for line in desc_lines.iter().take(2) { // Max 2 lines for description
                    Self::render_line_to_buffer(buf, area.x, y, area.width,
                        &[Span::styled(line.clone(), Style::default().fg(TAN_WOOD))]);
                    y += 1;
                }
            }
        }
    }

    /// Helper to render a single line to buffer (preserving background)
    fn render_line_to_buffer(buf: &mut ratatui::buffer::Buffer, x: u16, y: u16, max_width: u16, spans: &[Span]) {
        let mut col = x;
        for span in spans {
            let has_style = span.style.fg.is_some();
            for ch in span.content.chars() {
                if col >= x + max_width {
                    break;
                }
                if let Some(cell) = buf.cell_mut((col, y)) {
                    if ch == ' ' && !has_style {
                        col += 1;
                        continue;
                    }
                    cell.set_char(ch);
                    if let Some(fg) = span.style.fg {
                        cell.set_fg(fg);
                    }
                }
                col += 1;
            }
        }
    }

    /// Handle keyboard input, returns true if modal should close
    pub fn handle_input(&mut self, key: Key) -> bool {
        match key {
            Key::Esc | Key::Char('p') => true,
            Key::Up => {
                self.selection.move_up();
                false
            }
            Key::Down => {
                self.selection.move_down();
                false
            }
            _ => false,
        }
    }
}

impl Default for ProfileModal {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfileModal {
    /// Simple word wrap for description text
    fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else if current_line.len() + 1 + word.len() <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        lines
    }

    /// Calculate scroll offset to keep selection visible
    fn calculate_scroll_offset(selected: usize, total: usize, visible: usize) -> usize {
        if total <= visible {
            return 0;
        }
        // Keep selection in the middle third of visible area
        let ideal_start = selected.saturating_sub(visible / 2);
        ideal_start.min(total.saturating_sub(visible))
    }
}
