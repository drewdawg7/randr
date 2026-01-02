use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::ui::theme as colors;

/// Width of the dragon ASCII art
pub const DRAGON_WIDTH: u16 = 41;

/// Height of the dragon ASCII art
pub const DRAGON_HEIGHT: u16 = 19;

/// Renders the dragon ASCII art with colors
pub fn render_dragon_art() -> Vec<Line<'static>> {
    // Dragon colors
    let scale_dark = Style::default().fg(colors::DARK_FOREST);
    let scale = Style::default().fg(colors::FOREST_GREEN);
    let scale_light = Style::default().fg(colors::LIME_GREEN);
    let eye = Style::default().fg(colors::EMBER_RED);
    let accent = Style::default().fg(colors::FLAME_ORANGE);
    let claw = Style::default().fg(colors::LIGHT_STONE);
    let highlight = Style::default().fg(colors::PALE_GREEN);

    vec![
        // Row 1: Wings spread
        Line::from(vec![
            Span::styled("       ", scale),
            Span::styled("\\", scale_light),
            Span::styled("(______", scale),
            Span::styled("     ", scale),
            Span::styled("______", scale),
            Span::styled(")", scale_light),
            Span::styled("/", scale_light),
        ]),
        // Row 2
        Line::from(vec![
            Span::styled("       /", scale_light),
            Span::styled("`", highlight),
            Span::styled(".----.", scale),
            Span::styled("\\", scale_light),
            Span::styled("   ", scale),
            Span::styled("/", scale_light),
            Span::styled(".----.", scale),
            Span::styled("`", highlight),
            Span::styled("\\", scale_light),
        ]),
        // Row 3
        Line::from(vec![
            Span::styled("      ", scale),
            Span::styled("}", scale_light),
            Span::styled(" /      ", scale),
            Span::styled(":", accent),
            Span::styled("}", scale_light),
            Span::styled(" ", scale),
            Span::styled("{", scale_light),
            Span::styled(":", accent),
            Span::styled("      \\ ", scale),
            Span::styled("{", scale_light),
        ]),
        // Row 4
        Line::from(vec![
            Span::styled("     ", scale),
            Span::styled("/ ", scale_light),
            Span::styled("{        ", scale_dark),
            Span::styled("}", scale_light),
            Span::styled(" ", scale),
            Span::styled("{", scale_light),
            Span::styled("        } ", scale_dark),
            Span::styled("\\", scale_light),
        ]),
        // Row 5
        Line::from(vec![
            Span::styled("     ", scale),
            Span::styled("}", scale_light),
            Span::styled(" }      ", scale_dark),
            Span::styled(") ", accent),
            Span::styled("}", scale_light),
            Span::styled(" ", scale),
            Span::styled("{", scale_light),
            Span::styled(" (", accent),
            Span::styled("      { ", scale_dark),
            Span::styled("{", scale_light),
        ]),
        // Row 6: Horns
        Line::from(vec![
            Span::styled("    ", scale),
            Span::styled("/ ", scale_light),
            Span::styled("{      ", scale_dark),
            Span::styled("/|\\", claw),
            Span::styled("}", scale_light),
            Span::styled("!", accent),
            Span::styled("{", scale_light),
            Span::styled("/|\\", claw),
            Span::styled("      } ", scale_dark),
            Span::styled("\\", scale_light),
        ]),
        // Row 7: Face top
        Line::from(vec![
            Span::styled("    ", scale),
            Span::styled("}", scale_light),
            Span::styled(" }     ", scale_dark),
            Span::styled("( (", scale),
            Span::styled(".\"^\".", highlight),
            Span::styled(") )", scale),
            Span::styled("     { ", scale_dark),
            Span::styled("{", scale_light),
        ]),
        // Row 8: Eyes
        Line::from(vec![
            Span::styled("   ", scale),
            Span::styled("/ ", scale_light),
            Span::styled("{       ", scale_dark),
            Span::styled("(", scale),
            Span::styled("d", eye),
            Span::styled("\\", scale),
            Span::styled("   ", scale_dark),
            Span::styled("/", scale),
            Span::styled("b", eye),
            Span::styled(")", scale),
            Span::styled("       } ", scale_dark),
            Span::styled("\\", scale_light),
        ]),
        // Row 9: Snout
        Line::from(vec![
            Span::styled("   ", scale),
            Span::styled("}", scale_light),
            Span::styled(" }       ", scale_dark),
            Span::styled("|\\", scale),
            Span::styled("~", accent),
            Span::styled("   ", scale_dark),
            Span::styled("~", accent),
            Span::styled("/|", scale),
            Span::styled("       { ", scale_dark),
            Span::styled("{", scale_light),
        ]),
        // Row 10
        Line::from(vec![
            Span::styled("  ", scale),
            Span::styled("/ /", scale_light),
            Span::styled("        ", scale_dark),
            Span::styled("| )", scale),
            Span::styled("   ", scale_dark),
            Span::styled("( |", scale),
            Span::styled("        ", scale_dark),
            Span::styled("\\ \\", scale_light),
        ]),
        // Row 11
        Line::from(vec![
            Span::styled(" ", scale),
            Span::styled("{ {", scale_light),
            Span::styled("        ", scale_dark),
            Span::styled("_)(", scale),
            Span::styled(",   ,", accent),
            Span::styled(")(_", scale),
            Span::styled("        ", scale_dark),
            Span::styled("} }", scale_light),
        ]),
        // Row 12
        Line::from(vec![
            Span::styled("  ", scale),
            Span::styled("}", scale_light),
            Span::styled(" }      ", scale_dark),
            Span::styled("//", scale),
            Span::styled("  `\";\"` ", highlight),
            Span::styled(" \\\\", scale),
            Span::styled("      { ", scale_dark),
            Span::styled("{", scale_light),
        ]),
        // Row 13
        Line::from(vec![
            Span::styled(" ", scale),
            Span::styled("/ /", scale_light),
            Span::styled("      ", scale_dark),
            Span::styled("//", scale),
            Span::styled("     (     ", scale_dark),
            Span::styled("\\\\", scale),
            Span::styled("      ", scale_dark),
            Span::styled("\\ \\", scale_light),
        ]),
        // Row 14
        Line::from(vec![
            Span::styled("{ {", scale_light),
            Span::styled("      ", scale_dark),
            Span::styled("{(", scale),
            Span::styled("     ", scale_dark),
            Span::styled("-=)", accent),
            Span::styled("     ", scale_dark),
            Span::styled(")}", scale),
            Span::styled("      ", scale_dark),
            Span::styled("} }", scale_light),
        ]),
        // Row 15
        Line::from(vec![
            Span::styled(" ", scale),
            Span::styled("\\ \\", scale_light),
            Span::styled("     ", scale_dark),
            Span::styled("/)", scale),
            Span::styled("    ", scale_dark),
            Span::styled("-=(=-", accent),
            Span::styled("     ", scale_dark),
            Span::styled("(\\", scale),
            Span::styled("    ", scale_dark),
            Span::styled("/ /", scale_light),
        ]),
        // Row 16
        Line::from(vec![
            Span::styled("  ", scale),
            Span::styled("`\\\\", scale_light),
            Span::styled("  ", scale_dark),
            Span::styled("/'/", scale),
            Span::styled("    ", scale_dark),
            Span::styled("/-=|\\-\\", accent),
            Span::styled("    ", scale_dark),
            Span::styled("\\`\\", scale),
            Span::styled("  ", scale_dark),
            Span::styled("//'", scale_light),
        ]),
        // Row 17
        Line::from(vec![
            Span::styled("    ", scale),
            Span::styled("`\\{", scale_light),
            Span::styled("  |   ", scale_dark),
            Span::styled("( ", scale),
            Span::styled("-===-", accent),
            Span::styled(" )", scale),
            Span::styled("   |  ", scale_dark),
            Span::styled("}/'", scale_light),
        ]),
        // Row 18
        Line::from(vec![
            Span::styled("      ", scale),
            Span::styled("`", scale_light),
            Span::styled("  _\\   ", scale_dark),
            Span::styled("\\", scale),
            Span::styled("-===-", accent),
            Span::styled("/", scale),
            Span::styled("   /_  ", scale_dark),
            Span::styled("'", scale_light),
        ]),
        // Row 19: Feet/tail
        Line::from(vec![
            Span::styled("        ", scale),
            Span::styled("(_(_(_)", claw),
            Span::styled("'", highlight),
            Span::styled("-=-", accent),
            Span::styled("'", highlight),
            Span::styled("(_)_)_)", claw),
        ]),
    ]
}
