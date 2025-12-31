use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::ui::theme as colors;

pub struct Modal {
    lines: Vec<String>,
    width: u16,
    height: u16,
}

impl Modal {
    pub fn new(lines: Vec<String>) -> Self {
        // Calculate width from longest line + borders + padding
        let max_line_len = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16;
        let width = max_line_len + 4; // +2 for borders, +2 for padding

        // Calculate height from line count + borders
        let height = lines.len() as u16 + 2; // +2 for borders

        Self {
            lines,
            width,
            height,
        }
    }

    pub fn size(mut self, width: u16, height: u16) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    fn centered_rect(&self, frame_area: Rect) -> Rect {
        let x = frame_area.x + (frame_area.width.saturating_sub(self.width)) / 2;
        let y = frame_area.y + (frame_area.height.saturating_sub(self.height)) / 2;
        Rect::new(x, y, self.width, self.height)
    }

    pub fn render(&self, frame: &mut Frame) {
        let area = self.centered_rect(frame.area());

        // Clear the area behind the modal
        frame.render_widget(Clear, area);

        // Build the bordered block
        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(colors::HEADER_BG).fg(colors::WHITE));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Render content as multiple lines
        let text: Vec<Line> = self.lines.iter().map(|s| Line::from(s.as_str())).collect();
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(colors::WHITE));
        frame.render_widget(paragraph, inner);
    }
}
