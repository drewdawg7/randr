use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    Frame,
};

use crate::ui::theme as colors;

// Parchment/scroll colors
const SCROLL_DARK: ratatui::style::Color = colors::TAN_WOOD;
const SCROLL_LIGHT: ratatui::style::Color = colors::CREAM_WOOD;
const SCROLL_MID: ratatui::style::Color = colors::LIGHT_BEIGE;

/// Render content inside an ASCII scroll border
///
/// The scroll looks like:
/// ```text
///    ______________________________
///  / \                             \.
/// |   | <content line 1>           |.
///  \_ | <content line 2>           |.
///     | <content line 3>           |.
///     |   _________________________|___
///     |  /                            /.
///     \_/dc__________________________/.
/// ```
pub fn render_scroll_with_content(
    frame: &mut Frame,
    area: Rect,
    content_lines: Vec<String>,
) {
    let content_width = content_lines
        .iter()
        .map(|s| s.chars().count())
        .max()
        .unwrap_or(0)
        .max(18) as u16; // minimum width of 18

    let mut lines: Vec<Line> = Vec::new();
    let mut content_iter = content_lines.iter();

    // Line 1: Top edge with underscores
    lines.push(generate_top_line(content_width));

    // Line 2: Open fold top
    lines.push(generate_fold_top(content_width));

    // Line 3: First side line WITH content
    let content1 = content_iter.next().map(|s| s.as_str()).unwrap_or("");
    lines.push(generate_first_side_with_content(content1, content_width));

    // Line 4: Fold corner WITH content
    let content2 = content_iter.next().map(|s| s.as_str()).unwrap_or("");
    lines.push(generate_fold_corner_with_content(content2, content_width));

    // Remaining content lines
    for content in content_iter {
        lines.push(generate_content_line(content, content_width));
    }

    // Bottom section
    lines.push(generate_bottom_fold_start(content_width));
    lines.push(generate_bottom_fold_mid(content_width));
    lines.push(generate_bottom_line(content_width));

    // Calculate the area needed
    let total_height = lines.len() as u16;
    let total_width = (content_width + 10).min(area.width);

    let scroll_area = Rect::new(
        area.x,
        area.y,
        total_width,
        total_height.min(area.height),
    );

    // Render each line
    for (i, line) in lines.into_iter().enumerate() {
        if i as u16 >= scroll_area.height {
            break;
        }
        let line_area = Rect::new(
            scroll_area.x,
            scroll_area.y + i as u16,
            scroll_area.width,
            1,
        );
        frame.render_widget(ratatui::widgets::Paragraph::new(line), line_area);
    }
}

/// Top line: "   " + "_" repeated
fn generate_top_line(content_width: u16) -> Line<'static> {
    let mut spans = vec![
        Span::styled("   ", Style::default()),
    ];
    let underscores = "_".repeat((content_width + 2) as usize);
    spans.push(Span::styled(underscores, Style::default().fg(SCROLL_LIGHT)));
    Line::from(spans)
}

/// Fold top: " / \" + spaces + "\."
fn generate_fold_top(content_width: u16) -> Line<'static> {
    let spaces = " ".repeat((content_width + 1) as usize);
    Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled("/", Style::default().fg(SCROLL_DARK)),
        Span::styled(" ", Style::default()),
        Span::styled("\\", Style::default().fg(SCROLL_DARK)),
        Span::styled(spaces, Style::default()),
        Span::styled("\\", Style::default().fg(SCROLL_DARK)),
        Span::styled(".", Style::default().fg(SCROLL_MID)),
    ])
}

/// First side with content: "|   |" + content + "|."
fn generate_first_side_with_content(content: &str, content_width: u16) -> Line<'static> {
    let padded = format!("{:<width$}", content, width = content_width as usize);
    Line::from(vec![
        Span::styled("|", Style::default().fg(SCROLL_DARK)),
        Span::styled("   ", Style::default()),
        Span::styled("|", Style::default().fg(SCROLL_DARK)),
        Span::styled(padded, Style::default().fg(colors::WHITE)),
        Span::styled("|", Style::default().fg(SCROLL_DARK)),
        Span::styled(".", Style::default().fg(SCROLL_MID)),
    ])
}

/// Fold corner with content: " \_ |" + content + "|."
fn generate_fold_corner_with_content(content: &str, content_width: u16) -> Line<'static> {
    let padded = format!("{:<width$}", content, width = content_width as usize);
    Line::from(vec![
        Span::styled(" ", Style::default()),
        Span::styled("\\", Style::default().fg(SCROLL_DARK)),
        Span::styled("_", Style::default().fg(SCROLL_LIGHT)),
        Span::styled(" ", Style::default()),
        Span::styled("|", Style::default().fg(SCROLL_DARK)),
        Span::styled(padded, Style::default().fg(colors::WHITE)),
        Span::styled("|", Style::default().fg(SCROLL_DARK)),
        Span::styled(".", Style::default().fg(SCROLL_MID)),
    ])
}

/// Content line: "    |" + content (padded) + "|."
fn generate_content_line(content: &str, content_width: u16) -> Line<'static> {
    let padded = format!("{:<width$}", content, width = content_width as usize);
    Line::from(vec![
        Span::styled("    ", Style::default()),
        Span::styled("|", Style::default().fg(SCROLL_DARK)),
        Span::styled(padded, Style::default().fg(colors::WHITE)),
        Span::styled("|", Style::default().fg(SCROLL_DARK)),
        Span::styled(".", Style::default().fg(SCROLL_MID)),
    ])
}

/// Bottom fold start: "    |   " + "_" repeated + "|___"
fn generate_bottom_fold_start(content_width: u16) -> Line<'static> {
    let underscore_count = (content_width as i32 - 3).max(1) as usize;
    let underscores = "_".repeat(underscore_count);
    Line::from(vec![
        Span::styled("    ", Style::default()),
        Span::styled("|", Style::default().fg(SCROLL_DARK)),
        Span::styled("   ", Style::default()),
        Span::styled(underscores, Style::default().fg(SCROLL_LIGHT)),
        Span::styled("|", Style::default().fg(SCROLL_DARK)),
        Span::styled("___", Style::default().fg(SCROLL_LIGHT)),
    ])
}

/// Bottom fold mid: "    |  /" + spaces + "/."
fn generate_bottom_fold_mid(content_width: u16) -> Line<'static> {
    let spaces = " ".repeat(content_width as usize);
    Line::from(vec![
        Span::styled("    ", Style::default()),
        Span::styled("|", Style::default().fg(SCROLL_DARK)),
        Span::styled("  ", Style::default()),
        Span::styled("/", Style::default().fg(SCROLL_DARK)),
        Span::styled(spaces, Style::default()),
        Span::styled("/", Style::default().fg(SCROLL_DARK)),
        Span::styled(".", Style::default().fg(SCROLL_MID)),
    ])
}

/// Bottom line: "    \_/dc" + "_" repeated + "/."
fn generate_bottom_line(content_width: u16) -> Line<'static> {
    let underscore_count = (content_width as i32 - 4).max(1) as usize;
    let underscores = "_".repeat(underscore_count);
    Line::from(vec![
        Span::styled("    ", Style::default()),
        Span::styled("\\", Style::default().fg(SCROLL_DARK)),
        Span::styled("_", Style::default().fg(SCROLL_LIGHT)),
        Span::styled("/", Style::default().fg(SCROLL_DARK)),
        Span::styled("dc", Style::default().fg(SCROLL_MID)),
        Span::styled(underscores, Style::default().fg(SCROLL_LIGHT)),
        Span::styled("/", Style::default().fg(SCROLL_DARK)),
        Span::styled(".", Style::default().fg(SCROLL_MID)),
    ])
}
