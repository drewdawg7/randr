use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::ui::theme as colors;

/// Width of the dragon ASCII art
pub const DRAGON_WIDTH: u16 = 41;

/// Height of the dragon ASCII art
pub const DRAGON_HEIGHT: u16 = 19;

/// Renders the dragon ASCII art with rich colors and shading
pub fn render_dragon_art() -> Vec<Line<'static>> {
    // Dragon color palette
    let scale_dark = Style::default().fg(colors::DARK_FOREST);      // Deep shadows
    let scale = Style::default().fg(colors::FOREST_GREEN);          // Main body
    let scale_light = Style::default().fg(colors::LIME_GREEN);      // Highlights/edges
    let scale_pale = Style::default().fg(colors::PALE_GREEN);       // Bright accents
    let eye = Style::default().fg(colors::EMBER_RED);               // Glowing eyes
    let fire = Style::default().fg(colors::FLAME_ORANGE);           // Fire breath accents
    let fire_hot = Style::default().fg(colors::BRIGHT_YELLOW);      // Hot fire
    let claw = Style::default().fg(colors::LIGHT_STONE);            // Claws/horns
    let teeth = Style::default().fg(colors::WHITE);                 // Teeth
    let inner = Style::default().fg(colors::DEEP_ORANGE);           // Inner glow

    vec![
        // Row 1: Wings spread wide
        Line::from(vec![
            Span::styled("       ", scale),
            Span::styled("\\", scale_light),
            Span::styled("(", scale),
            Span::styled("_", scale_dark),
            Span::styled("_", scale),
            Span::styled("_", scale_dark),
            Span::styled("_", scale),
            Span::styled("_", scale_dark),
            Span::styled("_", scale),
            Span::styled("     ", scale_dark),
            Span::styled("_", scale),
            Span::styled("_", scale_dark),
            Span::styled("_", scale),
            Span::styled("_", scale_dark),
            Span::styled("_", scale),
            Span::styled("_", scale_dark),
            Span::styled(")", scale),
            Span::styled("/", scale_light),
        ]),
        // Row 2: Wing membranes
        Line::from(vec![
            Span::styled("       /", scale_light),
            Span::styled("`", scale_pale),
            Span::styled(".", scale),
            Span::styled("-", scale_dark),
            Span::styled("-", scale),
            Span::styled("-", scale_dark),
            Span::styled("-", scale),
            Span::styled(".", scale),
            Span::styled("\\", scale_light),
            Span::styled("   ", scale_dark),
            Span::styled("/", scale_light),
            Span::styled(".", scale),
            Span::styled("-", scale_dark),
            Span::styled("-", scale),
            Span::styled("-", scale_dark),
            Span::styled("-", scale),
            Span::styled(".", scale),
            Span::styled("`", scale_pale),
            Span::styled("\\", scale_light),
        ]),
        // Row 3: Upper body with spikes
        Line::from(vec![
            Span::styled("      ", scale),
            Span::styled("}", scale_light),
            Span::styled(" /", scale),
            Span::styled("######", scale_dark),
            Span::styled(":", fire),
            Span::styled("}", scale_light),
            Span::styled(" ", scale),
            Span::styled("{", scale_light),
            Span::styled(":", fire),
            Span::styled("######", scale_dark),
            Span::styled("\\ ", scale),
            Span::styled("{", scale_light),
        ]),
        // Row 4: Body bulk
        Line::from(vec![
            Span::styled("     ", scale),
            Span::styled("/ ", scale_light),
            Span::styled("{", scale),
            Span::styled("@@@@@@@@", scale_dark),
            Span::styled("}", scale_light),
            Span::styled(" ", scale),
            Span::styled("{", scale_light),
            Span::styled("@@@@@@@@", scale_dark),
            Span::styled("} ", scale),
            Span::styled("\\", scale_light),
        ]),
        // Row 5: Mid body with detail
        Line::from(vec![
            Span::styled("     ", scale),
            Span::styled("}", scale_light),
            Span::styled(" }", scale),
            Span::styled("@@@@@@", scale_dark),
            Span::styled(") ", fire),
            Span::styled("}", scale_light),
            Span::styled(" ", scale),
            Span::styled("{", scale_light),
            Span::styled(" (", fire),
            Span::styled("@@@@@@", scale_dark),
            Span::styled("{ ", scale),
            Span::styled("{", scale_light),
        ]),
        // Row 6: Horns with detail
        Line::from(vec![
            Span::styled("    ", scale),
            Span::styled("/ ", scale_light),
            Span::styled("{", scale),
            Span::styled("@@@@@@", scale_dark),
            Span::styled("/|\\", claw),
            Span::styled("}", scale_light),
            Span::styled("!", fire_hot),
            Span::styled("{", scale_light),
            Span::styled("/|\\", claw),
            Span::styled("@@@@@@", scale_dark),
            Span::styled("} ", scale),
            Span::styled("\\", scale_light),
        ]),
        // Row 7: Face crown
        Line::from(vec![
            Span::styled("    ", scale),
            Span::styled("}", scale_light),
            Span::styled(" }", scale),
            Span::styled("@@@@@", scale_dark),
            Span::styled("( (", scale),
            Span::styled(".\"", scale_pale),
            Span::styled("^", fire),
            Span::styled("\".", scale_pale),
            Span::styled(") )", scale),
            Span::styled("@@@@@", scale_dark),
            Span::styled("{ ", scale),
            Span::styled("{", scale_light),
        ]),
        // Row 8: Eyes - the most important row!
        Line::from(vec![
            Span::styled("   ", scale),
            Span::styled("/ ", scale_light),
            Span::styled("{", scale),
            Span::styled("@@@@@@@", scale_dark),
            Span::styled("(", scale),
            Span::styled("d", eye),
            Span::styled("\\", teeth),
            Span::styled("@@@", inner),
            Span::styled("/", teeth),
            Span::styled("b", eye),
            Span::styled(")", scale),
            Span::styled("@@@@@@@", scale_dark),
            Span::styled("} ", scale),
            Span::styled("\\", scale_light),
        ]),
        // Row 9: Snout with fire
        Line::from(vec![
            Span::styled("   ", scale),
            Span::styled("}", scale_light),
            Span::styled(" }", scale),
            Span::styled("@@@@@@@", scale_dark),
            Span::styled("|\\", scale),
            Span::styled("~", fire),
            Span::styled("@@@", inner),
            Span::styled("~", fire),
            Span::styled("/|", scale),
            Span::styled("@@@@@@@", scale_dark),
            Span::styled("{ ", scale),
            Span::styled("{", scale_light),
        ]),
        // Row 10: Mouth area
        Line::from(vec![
            Span::styled("  ", scale),
            Span::styled("/ /", scale_light),
            Span::styled("@@@@@@@@", scale_dark),
            Span::styled("| )", scale),
            Span::styled("@@@", inner),
            Span::styled("( |", scale),
            Span::styled("@@@@@@@@", scale_dark),
            Span::styled("\\ \\", scale_light),
        ]),
        // Row 11: Neck
        Line::from(vec![
            Span::styled(" ", scale),
            Span::styled("{ {", scale_light),
            Span::styled("@@@@@@@@", scale_dark),
            Span::styled("_)(", scale),
            Span::styled(",", fire),
            Span::styled("@@@", inner),
            Span::styled(",", fire),
            Span::styled(")(_", scale),
            Span::styled("@@@@@@@@", scale_dark),
            Span::styled("} }", scale_light),
        ]),
        // Row 12: Upper chest
        Line::from(vec![
            Span::styled("  ", scale),
            Span::styled("}", scale_light),
            Span::styled(" }", scale),
            Span::styled("@@@@@@", scale_dark),
            Span::styled("//", scale),
            Span::styled("  `\";\"` ", scale_pale),
            Span::styled(" \\\\", scale),
            Span::styled("@@@@@@", scale_dark),
            Span::styled("{ ", scale),
            Span::styled("{", scale_light),
        ]),
        // Row 13: Lower chest
        Line::from(vec![
            Span::styled(" ", scale),
            Span::styled("/ /", scale_light),
            Span::styled("@@@@@@", scale_dark),
            Span::styled("//", scale),
            Span::styled("@@@@@@@@@@@", scale_dark),
            Span::styled("\\\\", scale),
            Span::styled("@@@@@@", scale_dark),
            Span::styled("\\ \\", scale_light),
        ]),
        // Row 14: Belly
        Line::from(vec![
            Span::styled("{ {", scale_light),
            Span::styled("@@@@@@", scale_dark),
            Span::styled("{(", scale),
            Span::styled("@@@@@", scale_dark),
            Span::styled("-=)", fire),
            Span::styled("@@@@@", scale_dark),
            Span::styled(")}", scale),
            Span::styled("@@@@@@", scale_dark),
            Span::styled("} }", scale_light),
        ]),
        // Row 15: Lower body
        Line::from(vec![
            Span::styled(" ", scale),
            Span::styled("\\ \\", scale_light),
            Span::styled("@@@@@", scale_dark),
            Span::styled("/)", scale),
            Span::styled("@@@@", scale_dark),
            Span::styled("-=(=-", fire),
            Span::styled("@@@@@", scale_dark),
            Span::styled("(\\", scale),
            Span::styled("@@@@", scale_dark),
            Span::styled("/ /", scale_light),
        ]),
        // Row 16: Tail base
        Line::from(vec![
            Span::styled("  ", scale),
            Span::styled("`\\\\", scale_light),
            Span::styled("@@", scale_dark),
            Span::styled("/'/", scale),
            Span::styled("@@@@", scale_dark),
            Span::styled("/-=|\\-\\", fire),
            Span::styled("@@@@", scale_dark),
            Span::styled("\\`\\", scale),
            Span::styled("@@", scale_dark),
            Span::styled("//'", scale_light),
        ]),
        // Row 17: Lower tail
        Line::from(vec![
            Span::styled("    ", scale),
            Span::styled("`\\{", scale_light),
            Span::styled("@@", scale_dark),
            Span::styled("|", scale),
            Span::styled("@@@", scale_dark),
            Span::styled("( ", scale),
            Span::styled("-===-", fire_hot),
            Span::styled(" )", scale),
            Span::styled("@@@", scale_dark),
            Span::styled("|", scale),
            Span::styled("@@", scale_dark),
            Span::styled("}/'", scale_light),
        ]),
        // Row 18: Tail tip
        Line::from(vec![
            Span::styled("      ", scale),
            Span::styled("`", scale_light),
            Span::styled("@@", scale_dark),
            Span::styled("_\\", scale),
            Span::styled("@@@", scale_dark),
            Span::styled("\\", scale),
            Span::styled("-===-", fire),
            Span::styled("/", scale),
            Span::styled("@@@", scale_dark),
            Span::styled("/_", scale),
            Span::styled("@@", scale_dark),
            Span::styled("'", scale_light),
        ]),
        // Row 19: Feet and tail end
        Line::from(vec![
            Span::styled("        ", scale),
            Span::styled("(_(_(_)", claw),
            Span::styled("'", scale_pale),
            Span::styled("-=-", fire_hot),
            Span::styled("'", scale_pale),
            Span::styled("(_)_)_)", claw),
        ]),
    ]
}
