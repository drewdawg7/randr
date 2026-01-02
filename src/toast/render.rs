use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::ui::theme;
use super::definition::Toast;

const TOAST_WIDTH: u16 = 35;
const TOAST_HEIGHT: u16 = 3;
const TOAST_MARGIN: u16 = 1;
const RIGHT_MARGIN: u16 = 2;
const TOP_MARGIN: u16 = 1;

pub fn render_toasts(frame: &mut Frame, toasts: &[Toast]) {
    if toasts.is_empty() {
        return;
    }

    let frame_area = frame.area();

    for (i, toast) in toasts.iter().enumerate() {
        let y = TOP_MARGIN + (i as u16 * (TOAST_HEIGHT + TOAST_MARGIN));

        // Stop if we'd render off-screen
        if y + TOAST_HEIGHT > frame_area.height {
            break;
        }

        let x = frame_area.width.saturating_sub(TOAST_WIDTH + RIGHT_MARGIN);
        let area = Rect::new(x, y, TOAST_WIDTH, TOAST_HEIGHT);

        render_single_toast(frame, area, toast);
    }
}

fn render_single_toast(frame: &mut Frame, area: Rect, toast: &Toast) {
    let color = toast.toast_type.color();

    // Clear area behind toast
    frame.render_widget(Clear, area);

    // Block with colored border
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
        .style(Style::default().bg(theme::HEADER_BG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Content: "[!] ERROR: message"
    let icon_span = Span::styled(
        format!("{} ", toast.toast_type.icon()),
        Style::default().fg(color),
    );
    let label_span = Span::styled(
        format!("{}: ", toast.toast_type.label()),
        Style::default().fg(color),
    );
    let message_span = Span::styled(
        truncate_message(&toast.message, inner.width as usize - 15),
        Style::default().fg(theme::WHITE),
    );

    let line = Line::from(vec![icon_span, label_span, message_span]);
    let paragraph = Paragraph::new(line);
    frame.render_widget(paragraph, inner);
}

fn truncate_message(msg: &str, max_len: usize) -> String {
    if msg.len() <= max_len {
        msg.to_string()
    } else {
        format!("{}...", &msg[..max_len.saturating_sub(3)])
    }
}
