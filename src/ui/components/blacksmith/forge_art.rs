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

pub fn render_forge_art(padding: usize) -> Vec<Line<'static>> {
    let frame = get_animation_frame();
    // Spark colors (top)
    let spark_style = Style::default().fg(colors::HOT_WHITE);
    let tip_yellow = Style::default().fg(colors::BRIGHT_YELLOW);
    let tip_orange = Style::default().fg(colors::FLAME_ORANGE);
    let flame_orange = Style::default().fg(colors::FLAME_ORANGE);
    let flame_deep = Style::default().fg(colors::DEEP_ORANGE);
    let flame_red = Style::default().fg(colors::EMBER_RED);
    let coal_glow = Style::default().fg(colors::EMBER_RED);
    let coal_dark = Style::default().fg(colors::COAL_BLACK);
    let stone = Style::default().fg(colors::GRANITE);
    let stone_dark = Style::default().fg(colors::DARK_STONE);
    let stone_light = Style::default().fg(colors::LIGHT_STONE);
    let border = Style::default().fg(colors::GRANITE);

    // Base padding for the forge structure
    let pad = " ".repeat(padding);
    let pad7 = " ".repeat(padding + 7);  // sparks row offset
    let pad5 = " ".repeat(padding + 5);  // flame tips offset
    let pad4 = " ".repeat(padding + 4);  // upper flames offset
    let pad3 = " ".repeat(padding + 3);  // mid flames offset
    let pad2 = " ".repeat(padding + 2);  // lower flames offset

    // Animated spark patterns (cycle through frames)
    let spark_row = match frame {
        0 => Line::from(vec![
            Span::raw(pad7.clone()),
            Span::styled("*", spark_style),
            Span::styled("  .", tip_yellow),
            Span::styled("  '", tip_orange),
            Span::styled("  .", tip_yellow),
            Span::styled("  *", spark_style),
        ]),
        1 => Line::from(vec![
            Span::raw(pad7.clone()),
            Span::styled(".", tip_yellow),
            Span::styled("  *", spark_style),
            Span::styled("  .", tip_yellow),
            Span::styled("  '", tip_orange),
            Span::styled("  .", tip_yellow),
        ]),
        2 => Line::from(vec![
            Span::raw(pad7.clone()),
            Span::styled("'", tip_orange),
            Span::styled("  .", tip_yellow),
            Span::styled("  *", spark_style),
            Span::styled("  .", tip_yellow),
            Span::styled("  '", tip_orange),
        ]),
        _ => Line::from(vec![
            Span::raw(pad7.clone()),
            Span::styled(".", tip_yellow),
            Span::styled("  '", tip_orange),
            Span::styled("  .", tip_yellow),
            Span::styled("  *", spark_style),
            Span::styled("  .", tip_yellow),
        ]),
    };

    // Animated flame tip patterns
    let flame_tips_row = match frame {
        0 => Line::from(vec![
            Span::raw(pad5.clone()),
            Span::styled("'", tip_yellow),
            Span::styled("  `", tip_orange),
            Span::styled("~^~^~", flame_orange),
            Span::styled("`", tip_orange),
            Span::styled("  '", tip_yellow),
        ]),
        1 => Line::from(vec![
            Span::raw(pad5.clone()),
            Span::styled("`", tip_orange),
            Span::styled("  '", tip_yellow),
            Span::styled("^~^~^", flame_orange),
            Span::styled("'", tip_yellow),
            Span::styled("  `", tip_orange),
        ]),
        2 => Line::from(vec![
            Span::raw(pad5.clone()),
            Span::styled("'", tip_yellow),
            Span::styled("  '", tip_yellow),
            Span::styled("~^~^~", flame_orange),
            Span::styled("`", tip_orange),
            Span::styled("  '", tip_yellow),
        ]),
        _ => Line::from(vec![
            Span::raw(pad5.clone()),
            Span::styled("`", tip_orange),
            Span::styled("  `", tip_orange),
            Span::styled("^~^~^", flame_orange),
            Span::styled("'", tip_yellow),
            Span::styled("  `", tip_orange),
        ]),
    };

    // Animated upper flames
    let upper_flames_row = match frame {
        0 => Line::from(vec![
            Span::raw(pad4.clone()),
            Span::styled("`", tip_orange),
            Span::styled("  ~*~", flame_orange),
            Span::styled("'\"", flame_deep),
            Span::styled("~*~", flame_orange),
            Span::styled("  `", tip_orange),
        ]),
        1 => Line::from(vec![
            Span::raw(pad4.clone()),
            Span::styled("'", tip_yellow),
            Span::styled("  *~*", flame_orange),
            Span::styled("\"'", flame_deep),
            Span::styled("*~*", flame_orange),
            Span::styled("  '", tip_yellow),
        ]),
        2 => Line::from(vec![
            Span::raw(pad4.clone()),
            Span::styled("`", tip_orange),
            Span::styled("  ~o~", flame_orange),
            Span::styled("'\"", flame_deep),
            Span::styled("~o~", flame_orange),
            Span::styled("  `", tip_orange),
        ]),
        _ => Line::from(vec![
            Span::raw(pad4.clone()),
            Span::styled("'", tip_yellow),
            Span::styled("  o~o", flame_orange),
            Span::styled("\"'", flame_deep),
            Span::styled("o~o", flame_orange),
            Span::styled("  '", tip_yellow),
        ]),
    };

    // Animated mid flames
    let mid_flames_row = match frame {
        0 => Line::from(vec![
            Span::raw(pad3.clone()),
            Span::styled(".", tip_yellow),
            Span::styled(" '~\"~", flame_deep),
            Span::styled("*~*", flame_red),
            Span::styled("~\"~'", flame_deep),
            Span::styled(" .", tip_yellow),
        ]),
        1 => Line::from(vec![
            Span::raw(pad3.clone()),
            Span::styled("'", tip_orange),
            Span::styled(" ~\"~'", flame_deep),
            Span::styled("~*~", flame_red),
            Span::styled("'~\"~", flame_deep),
            Span::styled(" '", tip_orange),
        ]),
        2 => Line::from(vec![
            Span::raw(pad3.clone()),
            Span::styled(".", tip_yellow),
            Span::styled(" \"~'~", flame_deep),
            Span::styled("*~*", flame_red),
            Span::styled("~'~\"", flame_deep),
            Span::styled(" .", tip_yellow),
        ]),
        _ => Line::from(vec![
            Span::raw(pad3.clone()),
            Span::styled("'", tip_orange),
            Span::styled(" ~'~\"", flame_deep),
            Span::styled("~o~", flame_red),
            Span::styled("\"~'~", flame_deep),
            Span::styled(" '", tip_orange),
        ]),
    };

    // Animated lower flames
    let lower_flames_row = match frame {
        0 => Line::from(vec![
            Span::raw(pad2.clone()),
            Span::styled("'", tip_orange),
            Span::styled(" `~^~", flame_deep),
            Span::styled("'\"'\"'", flame_red),
            Span::styled("~^~`", flame_deep),
            Span::styled(" '", tip_orange),
        ]),
        1 => Line::from(vec![
            Span::raw(pad2.clone()),
            Span::styled("`", tip_orange),
            Span::styled(" ~^~`", flame_deep),
            Span::styled("\"'\"'\"", flame_red),
            Span::styled("`~^~", flame_deep),
            Span::styled(" `", tip_orange),
        ]),
        2 => Line::from(vec![
            Span::raw(pad2.clone()),
            Span::styled("'", tip_orange),
            Span::styled(" ^~`~", flame_deep),
            Span::styled("'\"'\"'", flame_red),
            Span::styled("~`~^", flame_deep),
            Span::styled(" '", tip_orange),
        ]),
        _ => Line::from(vec![
            Span::raw(pad2.clone()),
            Span::styled("`", tip_orange),
            Span::styled(" ~`~^", flame_deep),
            Span::styled("\"'\"'\"", flame_red),
            Span::styled("^~`~", flame_deep),
            Span::styled(" `", tip_orange),
        ]),
    };

    vec![
        // Sparks (row 1)
        spark_row,
        // Flame tips (row 2)
        flame_tips_row,
        // Upper flames (row 3)
        upper_flames_row,
        // Mid flames (row 4)
        mid_flames_row,
        // Lower flames (row 5)
        lower_flames_row,
        // Furnace top border (26 chars: ╔ + 24x═ + ╗)
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("╔════════════════════════╗", border),
        ]),
        // Furnace wall top (24 chars of ▒)
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("║", border),
            Span::styled("▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒", stone),
            Span::styled("║", border),
        ]),
        // Furnace opening top border (2 + 20 + 2 = 24 inner)
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("║", border),
            Span::styled("▒▒", stone_dark),
            Span::styled("╔══════════════════╗", stone_light),
            Span::styled("▒▒", stone_dark),
            Span::styled("║", border),
        ]),
        // Fire row 1 - wispy flame tips (18 chars inside inner box)
        match frame {
            0 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("'~^", tip_yellow),
                Span::styled("\"~o~", flame_orange),
                Span::styled("\"'^~", tip_yellow),
                Span::styled("^'~o", flame_orange),
                Span::styled("~\"^", tip_yellow),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            1 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("~^'", tip_yellow),
                Span::styled("~o~\"", flame_orange),
                Span::styled("'^~\"", tip_yellow),
                Span::styled("'~o^", flame_orange),
                Span::styled("\"^~", tip_yellow),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            2 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("^'~", tip_yellow),
                Span::styled("o~\"~", flame_orange),
                Span::styled("^~\"'", tip_yellow),
                Span::styled("~o^'", flame_orange),
                Span::styled("^~\"", tip_yellow),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            _ => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("\"~'", tip_yellow),
                Span::styled("~\"o~", flame_orange),
                Span::styled("~\"'^", tip_yellow),
                Span::styled("o^'~", flame_orange),
                Span::styled("~\"'", tip_yellow),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
        },
        // Fire row 2 - orange flames with more body
        match frame {
            0 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("~o@", flame_orange),
                Span::styled("O#~^~#O", flame_deep),
                Span::styled("@o~^~o@O", flame_orange),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            1 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("o@~", flame_orange),
                Span::styled("#~^~#OO", flame_deep),
                Span::styled("o~^~o@O@", flame_orange),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            2 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("@~o", flame_orange),
                Span::styled("~^~#OO#", flame_deep),
                Span::styled("~^~o@O@o", flame_orange),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            _ => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("~@o", flame_orange),
                Span::styled("^~#OO#~", flame_deep),
                Span::styled("^~o@O@o~", flame_orange),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
        },
        // Fire row 3 - deep orange/red flames
        match frame {
            0 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("@O#", flame_deep),
                Span::styled("o@#O@#O@#o", flame_red),
                Span::styled("O@#o@", flame_deep),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            1 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("O#@", flame_deep),
                Span::styled("@#O@#O@#o@", flame_red),
                Span::styled("@#o@O", flame_deep),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            2 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("#@O", flame_deep),
                Span::styled("#O@#O@#o@#", flame_red),
                Span::styled("#o@O@", flame_deep),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            _ => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("@#O", flame_deep),
                Span::styled("O@#O@#o@#O", flame_red),
                Span::styled("o@O@#", flame_deep),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
        },
        // Fire row 4 - red flames near coals
        match frame {
            0 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("#O@o", flame_red),
                Span::styled("@#O##O#@", coal_glow),
                Span::styled("o@O#@o", flame_red),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            1 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("O@o#", flame_red),
                Span::styled("#O##O#@@", coal_glow),
                Span::styled("@O#@oO", flame_red),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            2 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("@o#O", flame_red),
                Span::styled("O##O#@@#", coal_glow),
                Span::styled("O#@oO@", flame_red),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            _ => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("o#O@", flame_red),
                Span::styled("##O#@@#O", coal_glow),
                Span::styled("#@oO@o", flame_red),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
        },
        // Fire row 5 - glowing embers meeting coals
        match frame {
            0 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("O#@", flame_red),
                Span::styled("#O@O##O@O#@O", coal_glow),
                Span::styled("@#O", flame_red),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            1 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("#@O", flame_red),
                Span::styled("O@O##O@O#@O#", coal_glow),
                Span::styled("#O@", flame_red),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            2 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("@O#", flame_red),
                Span::styled("@O##O@O#@O#O", coal_glow),
                Span::styled("O@#", flame_red),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            _ => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("O@#", flame_red),
                Span::styled("O##O@O#@O#O@", coal_glow),
                Span::styled("@#O", flame_red),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
        },
        // Coal row 1 - animated glow (18 chars: 2+2+3+2+3+3+3)
        match frame {
            0 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("▄█", coal_dark),
                Span::styled("▓░", coal_glow),
                Span::styled("▄██", coal_dark),
                Span::styled("▓░", coal_glow),
                Span::styled("███", coal_dark),
                Span::styled("▄░▓", coal_glow),
                Span::styled("█▄▓", coal_dark),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            1 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("█▄", coal_dark),
                Span::styled("░▓", coal_glow),
                Span::styled("██▄", coal_dark),
                Span::styled("░▓", coal_glow),
                Span::styled("███", coal_dark),
                Span::styled("░▓▄", coal_glow),
                Span::styled("▄▓█", coal_dark),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            2 => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("▄█", coal_dark),
                Span::styled("░▓", coal_glow),
                Span::styled("▄██", coal_dark),
                Span::styled("▓▄", coal_glow),
                Span::styled("███", coal_dark),
                Span::styled("▓░▄", coal_glow),
                Span::styled("█▄▓", coal_dark),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
            _ => Line::from(vec![
                Span::raw(pad.clone()),
                Span::styled("║", border),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", stone_light),
                Span::styled("█▄", coal_dark),
                Span::styled("▓▄", coal_glow),
                Span::styled("██▄", coal_dark),
                Span::styled("░▓", coal_glow),
                Span::styled("███", coal_dark),
                Span::styled("▄▓░", coal_glow),
                Span::styled("▄▓█", coal_dark),
                Span::styled("║", stone_light),
                Span::styled("▒▒", stone_dark),
                Span::styled("║", border),
            ]),
        },
        // Coal row 2
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("║", border),
            Span::styled("▒▒", stone_dark),
            Span::styled("║", stone_light),
            Span::styled("██████████████████", coal_dark),
            Span::styled("║", stone_light),
            Span::styled("▒▒", stone_dark),
            Span::styled("║", border),
        ]),
        // Furnace opening bottom border
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("║", border),
            Span::styled("▒▒", stone_dark),
            Span::styled("╚══════════════════╝", stone_light),
            Span::styled("▒▒", stone_dark),
            Span::styled("║", border),
        ]),
        // Furnace bottom
        Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("╚════════════════════════╝", border),
        ]),
        // Base
        Line::from(vec![
            Span::raw(pad3),
            Span::styled("▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀", stone_dark),
        ]),
    ]
}
