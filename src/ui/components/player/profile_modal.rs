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
    stats::{HasStats, StatType},
    system::game_state,
    ui::components::utilities::{COIN, CROSSED_SWORDS, HEART, PICKAXE, SHIELD},
    ui::theme::{CREAM_WOOD, DARK_WALNUT, GREEN, LIGHT_BEIGE, OAK_BROWN, TAN_WOOD, WHITE, WOOD_BROWN},
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

/// A non-interactive modal displaying the player's profile.
/// Shows effective stats, gold, progression in a parchment-styled frame.
pub struct ProfileModal;

impl ProfileModal {
    pub fn new() -> Self {
        Self
    }

    /// Reset state when modal opens (no-op for stateless modal)
    #[allow(dead_code)]
    pub fn reset(&mut self) {}

    /// Render the profile modal
    pub fn render(&self, frame: &mut Frame) {
        let frame_area = frame.area();

        // Calculate modal dimensions (~60% of screen, centered)
        let modal_width = (frame_area.width * 60 / 100).min(50).max(35);
        let modal_height = (frame_area.height * 60 / 100).min(20).max(14);

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
    fn render_content_to_buffer(&self, frame: &mut Frame, area: Rect) {
        let player = &game_state().player;

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

        // Empty line and close hint
        lines.push(Line::from(Span::raw("")));
        lines.push(Line::from(Span::styled(
            "Press [p] or [Esc] to close",
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

    /// Handle keyboard input, returns true if modal should close
    pub fn handle_input(&self, key: Key) -> bool {
        matches!(key, Key::Esc | Key::Char('p'))
    }
}

impl Default for ProfileModal {
    fn default() -> Self {
        Self::new()
    }
}
