//! Player profile modal - displays player stats, progression, and resources
//! in a parchment-styled fantasy book aesthetic.

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
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

// Parchment background (warm tan/beige tint)
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

/// A non-interactive modal displaying the player's profile.
/// Shows effective stats, gold, progression in a parchment-styled frame.
pub struct ProfileModal;

impl ProfileModal {
    pub fn new() -> Self {
        Self
    }

    /// Reset state when modal opens (no-op for stateless modal)
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

        // Render the ASCII border
        self.render_ascii_border(frame, border_area);

        // Calculate inner area for content
        let inner_area = Rect::new(
            x + 2,
            y + 1,
            modal_width.saturating_sub(4),
            modal_height.saturating_sub(2),
        );

        // Render content
        self.render_content(frame, inner_area);
    }

    fn render_ascii_border(&self, frame: &mut Frame, area: Rect) {
        let width = area.width as usize;
        let height = area.height;

        let mut lines: Vec<Line> = Vec::new();

        // Top border with title
        let title = " Character Profile ";
        let title_start = (width.saturating_sub(title.len())) / 2;
        let top_spans: Vec<Span> = (0..width)
            .map(|i| {
                if i >= title_start && i < title_start + title.len() {
                    let ch = title.chars().nth(i - title_start).unwrap_or(' ');
                    Span::styled(
                        ch.to_string(),
                        Style::default().fg(CREAM_WOOD).bg(PARCHMENT_BG),
                    )
                } else {
                    let pattern_ch = TOP_PATTERN
                        .chars()
                        .nth(i % TOP_PATTERN.chars().count())
                        .unwrap_or(' ');
                    let color = PARCHMENT_COLORS[i % PARCHMENT_COLORS.len()];
                    Span::styled(
                        pattern_ch.to_string(),
                        Style::default().fg(color).bg(PARCHMENT_BG),
                    )
                }
            })
            .collect();
        lines.push(Line::from(top_spans));

        // Middle rows with side borders and background
        for row in 1..height.saturating_sub(1) {
            let left_ch = LEFT_PATTERN[row as usize % LEFT_PATTERN.len()];
            let right_ch = RIGHT_PATTERN[row as usize % RIGHT_PATTERN.len()];
            let left_color = PARCHMENT_COLORS[row as usize % PARCHMENT_COLORS.len()];
            let right_color = PARCHMENT_COLORS[(row as usize + 3) % PARCHMENT_COLORS.len()];

            let left = Span::styled(
                left_ch.to_string(),
                Style::default().fg(left_color).bg(PARCHMENT_BG),
            );
            let right = Span::styled(
                right_ch.to_string(),
                Style::default().fg(right_color).bg(PARCHMENT_BG),
            );
            let middle = Span::styled(
                " ".repeat(width.saturating_sub(2)),
                Style::default().bg(PARCHMENT_BG),
            );

            lines.push(Line::from(vec![left, middle, right]));
        }

        // Bottom border
        let bottom_spans: Vec<Span> = BOTTOM_PATTERN
            .chars()
            .cycle()
            .take(width)
            .enumerate()
            .map(|(i, ch)| {
                let color = PARCHMENT_COLORS[i % PARCHMENT_COLORS.len()];
                Span::styled(ch.to_string(), Style::default().fg(color).bg(PARCHMENT_BG))
            })
            .collect();
        lines.push(Line::from(bottom_spans));

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);
    }

    fn render_content(&self, frame: &mut Frame, area: Rect) {
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

        // Build content lines with explicit styles for readability on parchment background
        let text_style = Style::default().fg(WHITE).bg(PARCHMENT_BG);
        let label_style = Style::default().fg(CREAM_WOOD).bg(PARCHMENT_BG);
        let bonus_style = Style::default().fg(GREEN).bg(PARCHMENT_BG);

        let mut lines: Vec<Line> = Vec::new();

        // Name and Level
        lines.push(Line::from(vec![
            Span::styled(format!("{}", name), label_style),
            Span::styled("  ", text_style),
            Span::styled(format!("Lv. {}", level), text_style),
        ]));

        // Separator
        lines.push(Line::from(Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(TAN_WOOD).bg(PARCHMENT_BG),
        )));

        // Health
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ", HEART),
                Style::default()
                    .fg(Color::Rgb(244, 67, 54))
                    .bg(PARCHMENT_BG),
            ),
            Span::styled("Health:  ", label_style),
            Span::styled(format!("{}/{}", health, max_health), text_style),
        ]));

        // Attack with equipment bonus
        let attack_line = if attack_bonus > 0 {
            Line::from(vec![
                Span::styled(format!("{} ", CROSSED_SWORDS), text_style),
                Span::styled("Attack:  ", label_style),
                Span::styled(format!("{}", attack), text_style),
                Span::styled(format!(" (+{})", attack_bonus), bonus_style),
            ])
        } else {
            Line::from(vec![
                Span::styled(format!("{} ", CROSSED_SWORDS), text_style),
                Span::styled("Attack:  ", label_style),
                Span::styled(format!("{}", attack), text_style),
            ])
        };
        lines.push(attack_line);

        // Defense with equipment bonus
        let defense_line = if defense_bonus > 0 {
            Line::from(vec![
                Span::styled(format!("{} ", SHIELD), text_style),
                Span::styled("Defense: ", label_style),
                Span::styled(format!("{}", defense), text_style),
                Span::styled(format!(" (+{})", defense_bonus), bonus_style),
            ])
        } else {
            Line::from(vec![
                Span::styled(format!("{} ", SHIELD), text_style),
                Span::styled("Defense: ", label_style),
                Span::styled(format!("{}", defense), text_style),
            ])
        };
        lines.push(defense_line);

        // Separator
        lines.push(Line::from(Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(TAN_WOOD).bg(PARCHMENT_BG),
        )));

        // Gold
        lines.push(Line::from(vec![
            Span::styled(
                format!("{} ", COIN),
                Style::default()
                    .fg(Color::Rgb(255, 204, 0))
                    .bg(PARCHMENT_BG),
            ),
            Span::styled("Gold:      ", label_style),
            Span::styled(format!("{}", gold), text_style),
        ]));

        // GoldFind
        lines.push(Line::from(vec![
            Span::styled("✧ ", Style::default().fg(Color::Rgb(255, 204, 0)).bg(PARCHMENT_BG)),
            Span::styled("GoldFind:  ", label_style),
            Span::styled(format!("+{}%", goldfind), text_style),
        ]));

        // MagicFind
        lines.push(Line::from(vec![
            Span::styled("✦ ", Style::default().fg(Color::Rgb(180, 100, 255)).bg(PARCHMENT_BG)),
            Span::styled("MagicFind: ", label_style),
            Span::styled(format!("+{}%", magicfind), text_style),
        ]));

        // Mining
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", PICKAXE), text_style),
            Span::styled("Mining:    ", label_style),
            Span::styled(format!("{}", mining), text_style),
        ]));

        // Separator
        lines.push(Line::from(Span::styled(
            "─".repeat(area.width as usize),
            Style::default().fg(TAN_WOOD).bg(PARCHMENT_BG),
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
            Span::styled("XP: ", label_style),
            Span::styled("[", text_style),
            Span::styled("█".repeat(filled), Style::default().fg(GREEN).bg(PARCHMENT_BG)),
            Span::styled("░".repeat(empty), Style::default().fg(TAN_WOOD).bg(PARCHMENT_BG)),
            Span::styled("] ", text_style),
            Span::styled(format!("{}/{}", xp, xp_needed), text_style),
        ]));

        // Empty line and close hint
        lines.push(Line::from(Span::styled("", text_style)));
        lines.push(Line::from(Span::styled(
            "Press [p] or [Esc] to close",
            Style::default().fg(TAN_WOOD).bg(PARCHMENT_BG),
        )));

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);
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
