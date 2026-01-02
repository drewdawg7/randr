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

pub fn render_campfire_art() -> Vec<Line<'static>> {
    let frame = get_animation_frame();

    // Fire colors
    let spark_style = Style::default().fg(colors::HOT_WHITE);
    let tip_yellow = Style::default().fg(colors::BRIGHT_YELLOW);
    let tip_orange = Style::default().fg(colors::FLAME_ORANGE);
    let flame_orange = Style::default().fg(colors::FLAME_ORANGE);
    let flame_deep = Style::default().fg(colors::DEEP_ORANGE);
    let flame_red = Style::default().fg(colors::EMBER_RED);
    let coal_glow = Style::default().fg(colors::EMBER_RED);
    let coal_dark = Style::default().fg(colors::COAL_BLACK);
    let wood_brown = Style::default().fg(colors::WOOD_BROWN);
    let wood_dark = Style::default().fg(colors::DARK_WALNUT);
    let stone = Style::default().fg(colors::GRANITE);
    let stone_dark = Style::default().fg(colors::DARK_STONE);

    // Animated spark patterns (wide spread)
    let spark_row = match frame {
        0 => Line::from(vec![
            Span::styled("        *", spark_style),
            Span::styled("    .", tip_yellow),
            Span::styled("    '", tip_orange),
            Span::styled("    .", tip_yellow),
            Span::styled("    *        ", spark_style),
        ]),
        1 => Line::from(vec![
            Span::styled("        .", tip_yellow),
            Span::styled("    *", spark_style),
            Span::styled("    .", tip_yellow),
            Span::styled("    '", tip_orange),
            Span::styled("    .        ", tip_yellow),
        ]),
        2 => Line::from(vec![
            Span::styled("        '", tip_orange),
            Span::styled("    .", tip_yellow),
            Span::styled("    *", spark_style),
            Span::styled("    .", tip_yellow),
            Span::styled("    '        ", tip_orange),
        ]),
        _ => Line::from(vec![
            Span::styled("        .", tip_yellow),
            Span::styled("    '", tip_orange),
            Span::styled("    .", tip_yellow),
            Span::styled("    *", spark_style),
            Span::styled("    .        ", tip_yellow),
        ]),
    };

    // Flame tips (row 2)
    let flame_tips = match frame {
        0 => Line::from(vec![
            Span::styled("          '", tip_yellow),
            Span::styled("  `", tip_orange),
            Span::styled("~^~^~^~", flame_orange),
            Span::styled("`", tip_orange),
            Span::styled("  '          ", tip_yellow),
        ]),
        1 => Line::from(vec![
            Span::styled("          `", tip_orange),
            Span::styled("  '", tip_yellow),
            Span::styled("^~^~^~^", flame_orange),
            Span::styled("'", tip_yellow),
            Span::styled("  `          ", tip_orange),
        ]),
        2 => Line::from(vec![
            Span::styled("          '", tip_yellow),
            Span::styled("  '", tip_yellow),
            Span::styled("~^~^~^~", flame_orange),
            Span::styled("`", tip_orange),
            Span::styled("  '          ", tip_yellow),
        ]),
        _ => Line::from(vec![
            Span::styled("          `", tip_orange),
            Span::styled("  `", tip_orange),
            Span::styled("^~^~^~^", flame_orange),
            Span::styled("'", tip_yellow),
            Span::styled("  `          ", tip_orange),
        ]),
    };

    // Upper flames (row 3)
    let upper_flames = match frame {
        0 => Line::from(vec![
            Span::styled("        '", tip_yellow),
            Span::styled("  ~*~", flame_orange),
            Span::styled("'\"'^\"'", flame_deep),
            Span::styled("~*~", flame_orange),
            Span::styled("  '        ", tip_yellow),
        ]),
        1 => Line::from(vec![
            Span::styled("        `", tip_orange),
            Span::styled("  *~*", flame_orange),
            Span::styled("\"'\"'\"'", flame_deep),
            Span::styled("*~*", flame_orange),
            Span::styled("  `        ", tip_orange),
        ]),
        2 => Line::from(vec![
            Span::styled("        '", tip_yellow),
            Span::styled("  ~o~", flame_orange),
            Span::styled("'\"'^\"'", flame_deep),
            Span::styled("~o~", flame_orange),
            Span::styled("  '        ", tip_yellow),
        ]),
        _ => Line::from(vec![
            Span::styled("        `", tip_orange),
            Span::styled("  o~o", flame_orange),
            Span::styled("\"'\"'\"'", flame_deep),
            Span::styled("o~o", flame_orange),
            Span::styled("  `        ", tip_orange),
        ]),
    };

    // Mid-upper flames (row 4)
    let mid_upper_flames = match frame {
        0 => Line::from(vec![
            Span::styled("      .", tip_yellow),
            Span::styled("  '~\"~", flame_deep),
            Span::styled("*~@#@~*", flame_red),
            Span::styled("~\"~'", flame_deep),
            Span::styled("  .      ", tip_yellow),
        ]),
        1 => Line::from(vec![
            Span::styled("      '", tip_orange),
            Span::styled("  ~\"~'", flame_deep),
            Span::styled("~@#@#@~", flame_red),
            Span::styled("'~\"~", flame_deep),
            Span::styled("  '      ", tip_orange),
        ]),
        2 => Line::from(vec![
            Span::styled("      .", tip_yellow),
            Span::styled("  \"~'~", flame_deep),
            Span::styled("*@#@#@*", flame_red),
            Span::styled("~'~\"", flame_deep),
            Span::styled("  .      ", tip_yellow),
        ]),
        _ => Line::from(vec![
            Span::styled("      '", tip_orange),
            Span::styled("  ~'~\"", flame_deep),
            Span::styled("~#@#@#~", flame_red),
            Span::styled("\"~'~", flame_deep),
            Span::styled("  '      ", tip_orange),
        ]),
    };

    // Mid flames (row 5)
    let mid_flames = match frame {
        0 => Line::from(vec![
            Span::styled("    '", tip_orange),
            Span::styled("  `~^~", flame_deep),
            Span::styled("'\"@#@#@\"'", flame_red),
            Span::styled("~^~`", flame_deep),
            Span::styled("  '    ", tip_orange),
        ]),
        1 => Line::from(vec![
            Span::styled("    `", tip_orange),
            Span::styled("  ~^~`", flame_deep),
            Span::styled("\"'#@#@#'\"", flame_red),
            Span::styled("`~^~", flame_deep),
            Span::styled("  `    ", tip_orange),
        ]),
        2 => Line::from(vec![
            Span::styled("    '", tip_orange),
            Span::styled("  ^~`~", flame_deep),
            Span::styled("'@#@#@#@'", flame_red),
            Span::styled("~`~^", flame_deep),
            Span::styled("  '    ", tip_orange),
        ]),
        _ => Line::from(vec![
            Span::styled("    `", tip_orange),
            Span::styled("  ~`~^", flame_deep),
            Span::styled("\"#@#@#@#\"", flame_red),
            Span::styled("^~`~", flame_deep),
            Span::styled("  `    ", tip_orange),
        ]),
    };

    // Lower flames (row 6)
    let lower_flames = match frame {
        0 => Line::from(vec![
            Span::styled("   ", flame_red),
            Span::styled("'~^~`", flame_deep),
            Span::styled("'\"@#O#O#@\"'", flame_red),
            Span::styled("`~^~'", flame_deep),
            Span::styled("   ", flame_red),
        ]),
        1 => Line::from(vec![
            Span::styled("   ", flame_red),
            Span::styled("~^~`'", flame_deep),
            Span::styled("\"'#O#O#O#'\"", flame_red),
            Span::styled("'`~^~", flame_deep),
            Span::styled("   ", flame_red),
        ]),
        2 => Line::from(vec![
            Span::styled("   ", flame_red),
            Span::styled("^~`~'", flame_deep),
            Span::styled("'O#O#O#O#O'", flame_red),
            Span::styled("'~`~^", flame_deep),
            Span::styled("   ", flame_red),
        ]),
        _ => Line::from(vec![
            Span::styled("   ", flame_red),
            Span::styled("~`~^'", flame_deep),
            Span::styled("\"#O#O#O#O#\"", flame_red),
            Span::styled("'^~`~", flame_deep),
            Span::styled("   ", flame_red),
        ]),
    };

    // Coals/embers with animated glow (row 7)
    let coals = match frame {
        0 => Line::from(vec![
            Span::styled("   ", coal_dark),
            Span::styled("▄", coal_dark),
            Span::styled("░▓", coal_glow),
            Span::styled("█▄██▄█", coal_dark),
            Span::styled("▓░▓", coal_glow),
            Span::styled("██▄██", coal_dark),
            Span::styled("▓░", coal_glow),
            Span::styled("▄", coal_dark),
            Span::styled("   ", coal_dark),
        ]),
        1 => Line::from(vec![
            Span::styled("   ", coal_dark),
            Span::styled("▄", coal_dark),
            Span::styled("▓░", coal_glow),
            Span::styled("██▄██▄", coal_dark),
            Span::styled("░▓░", coal_glow),
            Span::styled("█▄██▄", coal_dark),
            Span::styled("░▓", coal_glow),
            Span::styled("▄", coal_dark),
            Span::styled("   ", coal_dark),
        ]),
        2 => Line::from(vec![
            Span::styled("   ", coal_dark),
            Span::styled("▄", coal_dark),
            Span::styled("░▓", coal_glow),
            Span::styled("█▄██▄█", coal_dark),
            Span::styled("▓░▓", coal_glow),
            Span::styled("██▄██", coal_dark),
            Span::styled("▓░", coal_glow),
            Span::styled("▄", coal_dark),
            Span::styled("   ", coal_dark),
        ]),
        _ => Line::from(vec![
            Span::styled("   ", coal_dark),
            Span::styled("▄", coal_dark),
            Span::styled("▓░", coal_glow),
            Span::styled("██▄██▄", coal_dark),
            Span::styled("░▓░", coal_glow),
            Span::styled("█▄██▄", coal_dark),
            Span::styled("░▓", coal_glow),
            Span::styled("▄", coal_dark),
            Span::styled("   ", coal_dark),
        ]),
    };

    // Log arrangement - crossing logs (row 8)
    let logs_top = Line::from(vec![
        Span::styled(" \\", wood_dark),
        Span::styled("▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄", wood_brown),
        Span::styled("/ ", wood_dark),
    ]);

    // Bottom logs (row 9)
    let logs_bottom = Line::from(vec![
        Span::styled("  /", wood_dark),
        Span::styled("▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀", wood_brown),
        Span::styled("\\  ", wood_dark),
    ]);

    // Stone circle around fire (row 10)
    let stones = Line::from(vec![
        Span::styled(".", stone_dark),
        Span::styled("o", stone),
        Span::styled("O", stone_dark),
        Span::styled("o", stone),
        Span::styled(".", stone_dark),
        Span::styled("O", stone),
        Span::styled("o", stone_dark),
        Span::styled(".", stone),
        Span::styled("O", stone_dark),
        Span::styled("o", stone),
        Span::styled(".", stone_dark),
        Span::styled("O", stone),
        Span::styled("o", stone_dark),
        Span::styled(".", stone),
        Span::styled("O", stone_dark),
        Span::styled("o", stone),
        Span::styled(".", stone_dark),
        Span::styled("O", stone),
        Span::styled("o", stone_dark),
        Span::styled(".", stone),
        Span::styled("O", stone_dark),
        Span::styled("o", stone),
        Span::styled(".", stone_dark),
        Span::styled("O", stone),
        Span::styled("o", stone_dark),
    ]);

    vec![
        spark_row,
        flame_tips,
        upper_flames,
        mid_upper_flames,
        mid_flames,
        lower_flames,
        coals,
        logs_top,
        logs_bottom,
        stones,
    ]
}

/// Returns the width of the campfire art
pub fn campfire_width() -> u16 {
    27
}
