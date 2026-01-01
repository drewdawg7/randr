use std::time::Instant;

use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::ui::theme as colors;

// Animation timing
const FRAME_DURATION_MS: u64 = 150;
const NUM_FRAMES: u64 = 4;

// Get the current animation frame based on elapsed time
fn get_animation_frame() -> usize {
    static START: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    let start = START.get_or_init(Instant::now);
    let elapsed = start.elapsed().as_millis() as u64;
    ((elapsed / FRAME_DURATION_MS) % NUM_FRAMES) as usize
}

pub fn render_flask_art(padding: usize) -> Vec<Line<'static>> {
    let frame = get_animation_frame();

    // Color styles
    let vapor_style = Style::default().fg(colors::VAPOR_CYAN);
    let vapor_dim = Style::default().fg(colors::PALE_LAVENDER);
    let glow_style = Style::default().fg(colors::MYSTIC_GLOW);
    let bubble_style = Style::default().fg(colors::BUBBLE_GREEN);
    let liquid_bright = Style::default().fg(colors::BRIGHT_VIOLET);
    let liquid_mid = Style::default().fg(colors::MYSTIC_PURPLE);
    let liquid_dark = Style::default().fg(colors::DARK_PURPLE);
    let liquid_deep = Style::default().fg(colors::DEEP_VIOLET);
    let glass_style = Style::default().fg(colors::PALE_LAVENDER);
    let glass_dim = Style::default().fg(colors::DARK_PURPLE);
    let stand_style = Style::default().fg(colors::DARK_STONE);

    // Base padding
    let pad = " ".repeat(padding);
    let pad2 = " ".repeat(padding + 2);
    let pad4 = " ".repeat(padding + 4);
    let pad6 = " ".repeat(padding + 6);
    let pad8 = " ".repeat(padding + 8);
    let pad10 = " ".repeat(padding + 10);

    // Animated vapor patterns (cycling through frames)
    let vapor_row1 = match frame {
        0 => Line::from(vec![
            Span::raw(pad10.clone()),
            Span::styled("~", vapor_style),
            Span::styled("  '", vapor_dim),
            Span::styled("  ~", vapor_style),
        ]),
        1 => Line::from(vec![
            Span::raw(pad10.clone()),
            Span::styled("'", vapor_dim),
            Span::styled("  ~", vapor_style),
            Span::styled("  '", vapor_dim),
        ]),
        2 => Line::from(vec![
            Span::raw(pad10.clone()),
            Span::styled("~", vapor_style),
            Span::styled("  `", vapor_dim),
            Span::styled("  ~", vapor_style),
        ]),
        _ => Line::from(vec![
            Span::raw(pad10.clone()),
            Span::styled("`", vapor_dim),
            Span::styled("  ~", vapor_style),
            Span::styled("  `", vapor_dim),
        ]),
    };

    let vapor_row2 = match frame {
        0 => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("'", vapor_dim),
            Span::styled("  ~", vapor_style),
            Span::styled("  `", vapor_dim),
            Span::styled("  ~", vapor_style),
        ]),
        1 => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("~", vapor_style),
            Span::styled("  `", vapor_dim),
            Span::styled("  ~", vapor_style),
            Span::styled("  '", vapor_dim),
        ]),
        2 => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("`", vapor_dim),
            Span::styled("  ~", vapor_style),
            Span::styled("  '", vapor_dim),
            Span::styled("  ~", vapor_style),
        ]),
        _ => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("~", vapor_style),
            Span::styled("  '", vapor_dim),
            Span::styled("  ~", vapor_style),
            Span::styled("  `", vapor_dim),
        ]),
    };

    // Flask neck with vapor inside
    let neck_vapor = match frame {
        0 => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("╭", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("─", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("─", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("─", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("╮", glass_style),
        ]),
        1 => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("╭", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("─", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("─", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("─", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("╮", glass_style),
        ]),
        2 => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("╭", glass_style),
            Span::styled("─", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("─", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("─", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("─", glass_style),
            Span::styled("╮", glass_style),
        ]),
        _ => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("╭", glass_style),
            Span::styled("─", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("─", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("─", glass_style),
            Span::styled("─", glass_dim),
            Span::styled("─", glass_style),
            Span::styled("╮", glass_style),
        ]),
    };

    // Neck interior with rising vapor
    let neck_interior = match frame {
        0 => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("│", glass_style),
            Span::styled(" ~ ", vapor_style),
            Span::styled("'", vapor_dim),
            Span::styled(" ~ ", vapor_style),
            Span::styled("│", glass_style),
        ]),
        1 => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("│", glass_style),
            Span::styled(" ' ", vapor_dim),
            Span::styled("~", vapor_style),
            Span::styled(" ` ", vapor_dim),
            Span::styled("│", glass_style),
        ]),
        2 => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("│", glass_style),
            Span::styled(" ` ", vapor_dim),
            Span::styled("~", vapor_style),
            Span::styled(" ' ", vapor_dim),
            Span::styled("│", glass_style),
        ]),
        _ => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("│", glass_style),
            Span::styled(" ~ ", vapor_style),
            Span::styled("`", vapor_dim),
            Span::styled(" ~ ", vapor_style),
            Span::styled("│", glass_style),
        ]),
    };

    // Flask body widening
    let body_top = Line::from(vec![
        Span::raw(pad6.clone()),
        Span::styled("/", glass_style),
        Span::raw("            "),
        Span::styled("\\", glass_style),
    ]);

    // Bubbling surface - animated
    let bubble_surface = match frame {
        0 => Line::from(vec![
            Span::raw(pad4.clone()),
            Span::styled("/", glass_style),
            Span::styled("  o", bubble_style),
            Span::styled("  ~~~", glow_style),
            Span::styled("~~~", liquid_bright),
            Span::styled("  o  ", bubble_style),
            Span::styled("\\", glass_style),
        ]),
        1 => Line::from(vec![
            Span::raw(pad4.clone()),
            Span::styled("/", glass_style),
            Span::styled(" o ", bubble_style),
            Span::styled(" ~~~~", glow_style),
            Span::styled("~~~", liquid_bright),
            Span::styled(" o   ", bubble_style),
            Span::styled("\\", glass_style),
        ]),
        2 => Line::from(vec![
            Span::raw(pad4.clone()),
            Span::styled("/", glass_style),
            Span::styled("   o", bubble_style),
            Span::styled(" ~~~", glow_style),
            Span::styled("~~~~", liquid_bright),
            Span::styled(" o   ", bubble_style),
            Span::styled("\\", glass_style),
        ]),
        _ => Line::from(vec![
            Span::raw(pad4.clone()),
            Span::styled("/", glass_style),
            Span::styled("  o ", bubble_style),
            Span::styled("~~~~", glow_style),
            Span::styled("~~~", liquid_bright),
            Span::styled("  o  ", bubble_style),
            Span::styled("\\", glass_style),
        ]),
    };

    // Liquid rows with rising bubbles - animated
    let liquid_row1 = match frame {
        0 => Line::from(vec![
            Span::raw(pad2.clone()),
            Span::styled("/", glass_style),
            Span::styled(" o", bubble_style),
            Span::styled("  ▓▓▓▓▓▓▓▓▓▓▓▓", liquid_bright),
            Span::styled("  o ", bubble_style),
            Span::styled("\\", glass_style),
        ]),
        1 => Line::from(vec![
            Span::raw(pad2.clone()),
            Span::styled("/", glass_style),
            Span::styled("  o", bubble_style),
            Span::styled(" ▓▓▓▓▓▓▓▓▓▓▓▓", liquid_bright),
            Span::styled(" o  ", bubble_style),
            Span::styled("\\", glass_style),
        ]),
        2 => Line::from(vec![
            Span::raw(pad2.clone()),
            Span::styled("/", glass_style),
            Span::styled("   o", bubble_style),
            Span::styled("▓▓▓▓▓▓▓▓▓▓▓▓", liquid_bright),
            Span::styled("o   ", bubble_style),
            Span::styled("\\", glass_style),
        ]),
        _ => Line::from(vec![
            Span::raw(pad2.clone()),
            Span::styled("/", glass_style),
            Span::styled(" o ", bubble_style),
            Span::styled(" ▓▓▓▓▓▓▓▓▓▓▓▓", liquid_bright),
            Span::styled("  o ", bubble_style),
            Span::styled("\\", glass_style),
        ]),
    };

    let liquid_row2 = match frame {
        0 => Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("|", glass_style),
            Span::styled("  ", liquid_mid),
            Span::styled("o", bubble_style),
            Span::styled(" ▓▓▓▓▓▓▓▓▓▓▓▓▓▓", liquid_mid),
            Span::styled(" o  ", bubble_style),
            Span::styled("|", glass_style),
        ]),
        1 => Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("|", glass_style),
            Span::styled(" o", bubble_style),
            Span::styled("  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓", liquid_mid),
            Span::styled("  o ", bubble_style),
            Span::styled("|", glass_style),
        ]),
        2 => Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("|", glass_style),
            Span::styled("   ", liquid_mid),
            Span::styled("o", bubble_style),
            Span::styled("▓▓▓▓▓▓▓▓▓▓▓▓▓▓", liquid_mid),
            Span::styled("o   ", bubble_style),
            Span::styled("|", glass_style),
        ]),
        _ => Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("|", glass_style),
            Span::styled("  o", bubble_style),
            Span::styled(" ▓▓▓▓▓▓▓▓▓▓▓▓▓▓", liquid_mid),
            Span::styled(" o  ", bubble_style),
            Span::styled("|", glass_style),
        ]),
    };

    // Static liquid rows (darker toward bottom)
    let liquid_row3 = Line::from(vec![
        Span::raw(pad.clone()),
        Span::styled("|", glass_style),
        Span::styled("  ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓  ", liquid_dark),
        Span::styled("|", glass_style),
    ]);

    let liquid_row4 = Line::from(vec![
        Span::raw(pad.clone()),
        Span::styled(" \\", glass_style),
        Span::styled("▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓", liquid_deep),
        Span::styled("/ ", glass_style),
    ]);

    // Flask bottom
    let flask_bottom = Line::from(vec![
        Span::raw(pad2.clone()),
        Span::styled("╰", glass_style),
        Span::styled("──────────────────", glass_dim),
        Span::styled("╯", glass_style),
    ]);

    // Stand
    let stand_row = Line::from(vec![
        Span::raw(pad4.clone()),
        Span::styled("══════════════", stand_style),
    ]);

    vec![
        vapor_row1,
        vapor_row2,
        neck_vapor,
        neck_interior,
        body_top,
        bubble_surface,
        liquid_row1,
        liquid_row2,
        liquid_row3,
        liquid_row4,
        flask_bottom,
        stand_row,
    ]
}
