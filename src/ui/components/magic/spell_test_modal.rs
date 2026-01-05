//! Spell testing modal - "god mode" for testing word combinations.
//! Allows entering any words to see the resulting spell effect.

use ratatui::{
    layout::Rect,
    style::Color,
    Frame,
};
use tuirealm::event::Key;

use crate::{
    magic::{
        page::Page,
        spell::compute_spell,
        word::WordId,
    },
    system::game_state,
    ui::theme::{CREAM_WOOD, CYAN, DARK_WALNUT, GREEN, LIGHT_BEIGE, OAK_BROWN, RED, TAN_WOOD, WHITE, WOOD_BROWN, YELLOW},
};

// Modal styling
const MODAL_BG: Color = Color::Rgb(45, 40, 35);
const INPUT_BG: Color = Color::Rgb(35, 30, 25);

/// State for the spell test modal
#[derive(Default)]
pub struct SpellTestModal {
    /// Current text input
    input: String,
    /// Parsed words from input
    parsed_words: Vec<WordId>,
    /// Unknown words that couldn't be parsed
    unknown_words: Vec<String>,
    /// Result message
    result: Option<SpellTestResult>,
    /// Selected page slot (0, 1, or 2)
    selected_page: usize,
    /// Message after inscription attempt
    inscribe_message: Option<(String, bool)>, // (message, is_success)
}

#[derive(Clone)]
enum SpellTestResult {
    Success {
        spell_name: String,
        description: String,
        is_backfire: bool,
    },
    Fizzle {
        reason: String,
    },
    ParseError {
        unknown: Vec<String>,
    },
}

impl SpellTestModal {
    pub fn new() -> Self {
        Self::default()
    }

    /// Reset state when modal opens
    pub fn reset(&mut self) {
        self.input.clear();
        self.parsed_words.clear();
        self.unknown_words.clear();
        self.result = None;
        self.selected_page = 0;
        self.inscribe_message = None;
    }

    /// Render the modal
    pub fn render(&self, frame: &mut Frame) {
        let frame_area = frame.area();

        // Modal dimensions
        let modal_width = (frame_area.width * 70 / 100).min(60).max(40);
        let modal_height = (frame_area.height * 70 / 100).min(24).max(16);

        let x = (frame_area.width.saturating_sub(modal_width)) / 2;
        let y = (frame_area.height.saturating_sub(modal_height)) / 2;

        let area = Rect::new(x, y, modal_width, modal_height);

        // Render background
        self.render_background(frame, area);

        // Render content
        self.render_content(frame, area);
    }

    fn render_background(&self, frame: &mut Frame, area: Rect) {
        let buf = frame.buffer_mut();

        // Fill background
        for row in 0..area.height {
            for col in 0..area.width {
                let cell_x = area.x + col;
                let cell_y = area.y + row;

                if let Some(cell) = buf.cell_mut((cell_x, cell_y)) {
                    cell.set_bg(MODAL_BG);
                    cell.set_char(' ');
                }
            }
        }

        // Simple border
        let border_color = WOOD_BROWN;

        // Top and bottom
        for col in 0..area.width {
            if let Some(cell) = buf.cell_mut((area.x + col, area.y)) {
                cell.set_char('─');
                cell.set_fg(border_color);
            }
            if let Some(cell) = buf.cell_mut((area.x + col, area.y + area.height - 1)) {
                cell.set_char('─');
                cell.set_fg(border_color);
            }
        }

        // Sides
        for row in 1..area.height - 1 {
            if let Some(cell) = buf.cell_mut((area.x, area.y + row)) {
                cell.set_char('│');
                cell.set_fg(border_color);
            }
            if let Some(cell) = buf.cell_mut((area.x + area.width - 1, area.y + row)) {
                cell.set_char('│');
                cell.set_fg(border_color);
            }
        }

        // Corners
        if let Some(cell) = buf.cell_mut((area.x, area.y)) {
            cell.set_char('┌');
            cell.set_fg(border_color);
        }
        if let Some(cell) = buf.cell_mut((area.x + area.width - 1, area.y)) {
            cell.set_char('┐');
            cell.set_fg(border_color);
        }
        if let Some(cell) = buf.cell_mut((area.x, area.y + area.height - 1)) {
            cell.set_char('└');
            cell.set_fg(border_color);
        }
        if let Some(cell) = buf.cell_mut((area.x + area.width - 1, area.y + area.height - 1)) {
            cell.set_char('┘');
            cell.set_fg(border_color);
        }
    }

    fn render_content(&self, frame: &mut Frame, area: Rect) {
        let buf = frame.buffer_mut();
        let inner_x = area.x + 2;
        let inner_width = area.width.saturating_sub(4);
        let mut current_y = area.y + 1;

        // Title
        let title = "✦ Spell Tester (God Mode) ✦";
        let title_x = inner_x + (inner_width.saturating_sub(title.len() as u16)) / 2;
        for (i, ch) in title.chars().enumerate() {
            if let Some(cell) = buf.cell_mut((title_x + i as u16, current_y)) {
                cell.set_char(ch);
                cell.set_fg(CREAM_WOOD);
            }
        }
        current_y += 2;

        // Instructions
        let instructions = "Enter words separated by spaces, then press Enter";
        for (i, ch) in instructions.chars().enumerate() {
            if (i as u16) < inner_width {
                if let Some(cell) = buf.cell_mut((inner_x + i as u16, current_y)) {
                    cell.set_char(ch);
                    cell.set_fg(TAN_WOOD);
                }
            }
        }
        current_y += 2;

        // Input field background
        let input_width = inner_width;
        for col in 0..input_width {
            if let Some(cell) = buf.cell_mut((inner_x + col, current_y)) {
                cell.set_bg(INPUT_BG);
                cell.set_char(' ');
            }
        }

        // Input text
        let display_input = if self.input.is_empty() {
            "fire bolt power...".to_string()
        } else {
            self.input.clone()
        };
        let input_color = if self.input.is_empty() { TAN_WOOD } else { WHITE };

        for (i, ch) in display_input.chars().enumerate() {
            if (i as u16) < input_width {
                if let Some(cell) = buf.cell_mut((inner_x + i as u16, current_y)) {
                    cell.set_char(ch);
                    cell.set_fg(input_color);
                    cell.set_bg(INPUT_BG);
                }
            }
        }

        // Cursor
        let cursor_pos = self.input.len().min(input_width as usize - 1);
        if let Some(cell) = buf.cell_mut((inner_x + cursor_pos as u16, current_y)) {
            cell.set_char('▌');
            cell.set_fg(WHITE);
            cell.set_bg(INPUT_BG);
        }

        current_y += 2;

        // Separator
        for col in 0..inner_width {
            if let Some(cell) = buf.cell_mut((inner_x + col, current_y)) {
                cell.set_char('─');
                cell.set_fg(DARK_WALNUT);
            }
        }
        current_y += 1;

        // Result section
        if let Some(result) = &self.result {
            match result {
                SpellTestResult::Success { spell_name, description, is_backfire } => {
                    // Spell name
                    let name_label = if *is_backfire { "⚠ BACKFIRE: " } else { "✓ Spell: " };
                    let name_color = if *is_backfire { RED } else { GREEN };

                    let mut col = 0u16;
                    for ch in name_label.chars() {
                        if col < inner_width {
                            if let Some(cell) = buf.cell_mut((inner_x + col, current_y)) {
                                cell.set_char(ch);
                                cell.set_fg(name_color);
                            }
                            col += 1;
                        }
                    }
                    for ch in spell_name.chars() {
                        if col < inner_width {
                            if let Some(cell) = buf.cell_mut((inner_x + col, current_y)) {
                                cell.set_char(ch);
                                cell.set_fg(CREAM_WOOD);
                            }
                            col += 1;
                        }
                    }
                    current_y += 2;

                    // Description (word wrap)
                    let desc_lines = word_wrap(description, inner_width as usize);
                    for line in desc_lines.iter().take(4) {
                        for (i, ch) in line.chars().enumerate() {
                            if (i as u16) < inner_width {
                                if let Some(cell) = buf.cell_mut((inner_x + i as u16, current_y)) {
                                    cell.set_char(ch);
                                    cell.set_fg(LIGHT_BEIGE);
                                }
                            }
                        }
                        current_y += 1;
                    }
                }
                SpellTestResult::Fizzle { reason } => {
                    let fizzle = "✗ Fizzle: ";
                    let mut col = 0u16;
                    for ch in fizzle.chars() {
                        if col < inner_width {
                            if let Some(cell) = buf.cell_mut((inner_x + col, current_y)) {
                                cell.set_char(ch);
                                cell.set_fg(YELLOW);
                            }
                            col += 1;
                        }
                    }
                    for ch in reason.chars() {
                        if col < inner_width {
                            if let Some(cell) = buf.cell_mut((inner_x + col, current_y)) {
                                cell.set_char(ch);
                                cell.set_fg(TAN_WOOD);
                            }
                            col += 1;
                        }
                    }
                }
                SpellTestResult::ParseError { unknown } => {
                    let error = "✗ Unknown words: ";
                    let mut col = 0u16;
                    for ch in error.chars() {
                        if col < inner_width {
                            if let Some(cell) = buf.cell_mut((inner_x + col, current_y)) {
                                cell.set_char(ch);
                                cell.set_fg(RED);
                            }
                            col += 1;
                        }
                    }
                    for word in unknown {
                        for ch in word.chars() {
                            if col < inner_width {
                                if let Some(cell) = buf.cell_mut((inner_x + col, current_y)) {
                                    cell.set_char(ch);
                                    cell.set_fg(CREAM_WOOD);
                                }
                                col += 1;
                            }
                        }
                        // Space between words
                        if col < inner_width {
                            col += 1;
                        }
                    }
                }
            }
        } else {
            // Show available words
            let available = "Available words:";
            for (i, ch) in available.chars().enumerate() {
                if (i as u16) < inner_width {
                    if let Some(cell) = buf.cell_mut((inner_x + i as u16, current_y)) {
                        cell.set_char(ch);
                        cell.set_fg(OAK_BROWN);
                    }
                }
            }
            current_y += 1;

            // List all words
            let all_words: Vec<&str> = WordId::all().iter().map(|w| w.name()).collect();
            let words_str = all_words.join(", ");
            let word_lines = word_wrap(&words_str, inner_width as usize);

            for line in word_lines.iter().take(3) {
                for (i, ch) in line.chars().enumerate() {
                    if (i as u16) < inner_width {
                        if let Some(cell) = buf.cell_mut((inner_x + i as u16, current_y)) {
                            cell.set_char(ch);
                            cell.set_fg(TAN_WOOD);
                        }
                    }
                }
                current_y += 1;
            }
        }

        // Page selection and inscribe message (above footer)
        let page_y = area.y + area.height - 4;

        // Show inscribe message if any
        if let Some((msg, is_success)) = &self.inscribe_message {
            let msg_color = if *is_success { GREEN } else { RED };
            for (i, ch) in msg.chars().enumerate() {
                if (i as u16) < inner_width {
                    if let Some(cell) = buf.cell_mut((inner_x + i as u16, page_y)) {
                        cell.set_char(ch);
                        cell.set_fg(msg_color);
                    }
                }
            }
        } else if self.result.is_some() {
            // Show page selection when a valid spell is tested
            let page_label = "Inscribe to page: ";
            let mut col = 0u16;
            for ch in page_label.chars() {
                if col < inner_width {
                    if let Some(cell) = buf.cell_mut((inner_x + col, page_y)) {
                        cell.set_char(ch);
                        cell.set_fg(TAN_WOOD);
                    }
                    col += 1;
                }
            }

            // Get active page index from tome
            let active_page_idx = {
                let gs = game_state();
                gs.player.equipped_tome().map(|t| t.active_page_index()).unwrap_or(0)
            };

            // Draw page slots [1] [2] [3] with active indicator
            for i in 0..3 {
                let is_selected = i == self.selected_page;
                let is_active = i == active_page_idx;
                let bracket_color = if is_selected { CYAN } else { DARK_WALNUT };
                let num_color = if is_active { GREEN } else if is_selected { WHITE } else { TAN_WOOD };

                if col < inner_width {
                    if let Some(cell) = buf.cell_mut((inner_x + col, page_y)) {
                        cell.set_char('[');
                        cell.set_fg(bracket_color);
                    }
                    col += 1;
                }
                if col < inner_width {
                    if let Some(cell) = buf.cell_mut((inner_x + col, page_y)) {
                        cell.set_char(char::from_digit((i + 1) as u32, 10).unwrap_or('?'));
                        cell.set_fg(num_color);
                    }
                    col += 1;
                }
                // Show * for active page
                if is_active {
                    if col < inner_width {
                        if let Some(cell) = buf.cell_mut((inner_x + col, page_y)) {
                            cell.set_char('*');
                            cell.set_fg(GREEN);
                        }
                        col += 1;
                    }
                }
                if col < inner_width {
                    if let Some(cell) = buf.cell_mut((inner_x + col, page_y)) {
                        cell.set_char(']');
                        cell.set_fg(bracket_color);
                    }
                    col += 1;
                }
                if col < inner_width {
                    col += 1; // space between
                }
            }
        }

        // Footer
        let footer_y = area.y + area.height - 2;
        let footer = if self.result.is_some() && !matches!(&self.result, Some(SpellTestResult::ParseError { .. })) {
            "[Tab] Inscribe  [A] Activate  [←→] Page  [Esc] Close"
        } else {
            "[Enter] Test  [←→] Page  [A] Activate  [Esc] Close"
        };
        let footer_x = inner_x + (inner_width.saturating_sub(footer.len() as u16)) / 2;
        for (i, ch) in footer.chars().enumerate() {
            if let Some(cell) = buf.cell_mut((footer_x + i as u16, footer_y)) {
                cell.set_char(ch);
                cell.set_fg(DARK_WALNUT);
            }
        }
    }

    /// Handle keyboard input, returns true if modal should close
    pub fn handle_input(&mut self, key: Key) -> bool {
        match key {
            Key::Esc => true,
            Key::Char('A') => {
                // Activate the selected page (Shift+A)
                self.activate_page();
                false
            }
            Key::Char(c) if c.is_alphanumeric() || c == ' ' => {
                if self.input.len() < 50 {
                    self.input.push(c);
                    self.inscribe_message = None;
                }
                false
            }
            Key::Backspace => {
                self.input.pop();
                self.result = None;
                self.inscribe_message = None;
                false
            }
            Key::Enter => {
                self.test_spell();
                self.inscribe_message = None;
                false
            }
            Key::Tab => {
                self.inscribe_spell();
                false
            }
            Key::Left => {
                if self.selected_page > 0 {
                    self.selected_page -= 1;
                }
                false
            }
            Key::Right => {
                if self.selected_page < 2 {
                    self.selected_page += 1;
                }
                false
            }
            _ => false,
        }
    }

    /// Activate the selected page (set it as the active spell page)
    fn activate_page(&mut self) {
        let gs = game_state();
        let Some(tome) = gs.player.equipped_tome_mut() else {
            self.inscribe_message = Some(("No tome equipped!".to_string(), false));
            return;
        };

        if let Err(_) = tome.set_active_page(self.selected_page) {
            self.inscribe_message = Some(("Invalid page!".to_string(), false));
            return;
        }

        self.inscribe_message = Some((
            format!("Page {} is now active!", self.selected_page + 1),
            true,
        ));
    }

    /// Inscribe the current spell onto the selected page in the player's tome
    fn inscribe_spell(&mut self) {
        // Must have a valid spell result (not parse error)
        let is_valid = matches!(
            &self.result,
            Some(SpellTestResult::Success { .. }) | Some(SpellTestResult::Fizzle { .. })
        );

        if !is_valid {
            self.inscribe_message = Some(("Test a spell first!".to_string(), false));
            return;
        }

        // Get parsed words
        if self.parsed_words.is_empty() {
            // Re-parse from input
            let words: Vec<&str> = self.input.split_whitespace().collect();
            self.parsed_words.clear();
            for word in words {
                if let Some(word_id) = WordId::from_str(word) {
                    self.parsed_words.push(word_id);
                }
            }
        }

        if self.parsed_words.is_empty() {
            self.inscribe_message = Some(("No valid words to inscribe!".to_string(), false));
            return;
        }

        // Create and inscribe the page
        let mut page = Page::new();
        let _result = page.inscribe(self.parsed_words.clone());

        // Now get the mutable tome reference
        let gs = game_state();
        let Some(tome) = gs.player.equipped_tome_mut() else {
            self.inscribe_message = Some(("No tome equipped!".to_string(), false));
            return;
        };

        // Add the page to the tome
        if let Err(_) = tome.set_page(self.selected_page, page) {
            self.inscribe_message = Some(("Failed to set page!".to_string(), false));
            return;
        }

        // Show success message
        let spell_name = match &self.result {
            Some(SpellTestResult::Success { spell_name, .. }) => spell_name.clone(),
            Some(SpellTestResult::Fizzle { .. }) => "Fizzle".to_string(),
            _ => "Unknown".to_string(),
        };

        self.inscribe_message = Some((
            format!("Inscribed '{}' to page {}!", spell_name, self.selected_page + 1),
            true,
        ));
    }

    /// Test the current input as a spell
    fn test_spell(&mut self) {
        if self.input.trim().is_empty() {
            return;
        }

        let words: Vec<&str> = self.input.split_whitespace().collect();
        let mut parsed = Vec::new();
        let mut unknown = Vec::new();

        for word in words {
            if let Some(word_id) = WordId::from_str(word) {
                parsed.push(word_id);
            } else {
                unknown.push(word.to_string());
            }
        }

        if !unknown.is_empty() {
            self.parsed_words.clear();
            self.result = Some(SpellTestResult::ParseError { unknown });
            return;
        }

        if parsed.is_empty() {
            return;
        }

        // Save parsed words for inscribing
        self.parsed_words = parsed.clone();

        // Compute the spell
        let spell = compute_spell(&parsed);

        self.result = Some(match spell {
            crate::magic::spell::ComputedSpell::Active { name, description, .. } => {
                SpellTestResult::Success {
                    spell_name: name,
                    description,
                    is_backfire: false,
                }
            }
            crate::magic::spell::ComputedSpell::Passive { name, description, .. } => {
                SpellTestResult::Success {
                    spell_name: format!("{} (Passive)", name),
                    description,
                    is_backfire: false,
                }
            }
            crate::magic::spell::ComputedSpell::Hybrid { name, description, .. } => {
                SpellTestResult::Success {
                    spell_name: format!("{} (Hybrid)", name),
                    description,
                    is_backfire: false,
                }
            }
            crate::magic::spell::ComputedSpell::Backfire { reason, effect } => {
                SpellTestResult::Success {
                    spell_name: "Unstable Magic".to_string(),
                    description: format!("{}: {}", reason, effect.describe()),
                    is_backfire: true,
                }
            }
            crate::magic::spell::ComputedSpell::Fizzle { reason } => {
                SpellTestResult::Fizzle { reason }
            }
        });
    }
}

/// Simple word wrap helper
fn word_wrap(text: &str, max_width: usize) -> Vec<String> {
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
