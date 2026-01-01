use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::ui::theme as colors;

pub fn render_anvil_art(padding: usize) -> Vec<Line<'static>> {
    let anvil_style = Style::default().fg(colors::LIGHT_STONE);
    let pad = " ".repeat(padding);

    vec![
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("⠀⠀⠀⠀⠀⠀⠀⢰⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⣶⡄⠀⠀⠀⠀⠀", anvil_style),
        ]),
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("⠀⠹⣿⣿⣿⣿⡇⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⢠⣄⡀⠀⠀", anvil_style),
        ]),
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("⠀⠀⠙⢿⣿⣿⡇⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⢸⣿⣿⡶⠀", anvil_style),
        ]),
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("⠀⠀⠀⠀⠉⠛⠇⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠸⠟⠋⠀⠀", anvil_style),
        ]),
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("⠀⠀⠀⠀⠀⠀⠀⠸⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠿⠇⠀⠀⠀⠀⠀", anvil_style),
        ]),
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣶⣶⣶⣶⣶⣶⣶⣶⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀", anvil_style),
        ]),
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⣿⣿⣿⣿⣿⣿⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀", anvil_style),
        ]),
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣄⠀⠀⠀⠀⠀⠀⠀", anvil_style),
        ]),
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("⠀⠀⠀⠀⠀⠀⣀⣀⣈⣉⣉⣉⣉⣉⣉⣉⣉⣉⣉⣉⣉⣉⣉⣁⣀⣀⠀⠀⠀⠀", anvil_style),
        ]),
        Line::from(vec![
            Span::raw(pad),
            Span::styled("⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠀⠀⠀⠀", anvil_style),
        ]),
    ]
}
