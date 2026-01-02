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

pub fn render_campfire_art(padding: usize) -> Vec<Line<'static>> {
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

    let pad = " ".repeat(padding);
    let pad2 = " ".repeat(padding + 2);
    let pad4 = " ".repeat(padding + 4);
    let pad6 = " ".repeat(padding + 6);
    let pad8 = " ".repeat(padding + 8);

    // Animated spark patterns
    let spark_row = match frame {
        0 => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("*", spark_style),
            Span::styled("  .", tip_yellow),
            Span::styled("  '", tip_orange),
        ]),
        1 => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled(".", tip_yellow),
            Span::styled("  *", spark_style),
            Span::styled("  .", tip_yellow),
        ]),
        2 => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled("'", tip_orange),
            Span::styled("  .", tip_yellow),
            Span::styled("  *", spark_style),
        ]),
        _ => Line::from(vec![
            Span::raw(pad8.clone()),
            Span::styled(".", tip_yellow),
            Span::styled("  '", tip_orange),
            Span::styled("  .", tip_yellow),
        ]),
    };

    // Flame tips
    let flame_tips = match frame {
        0 => Line::from(vec![
            Span::raw(pad6.clone()),
            Span::styled("'", tip_yellow),
            Span::styled(" ^~^", flame_orange),
            Span::styled("~^", tip_yellow),
            Span::styled(" '", tip_orange),
        ]),
        1 => Line::from(vec![
            Span::raw(pad6.clone()),
            Span::styled("`", tip_orange),
            Span::styled(" ~^~", flame_orange),
            Span::styled("^~", tip_yellow),
            Span::styled(" `", tip_yellow),
        ]),
        2 => Line::from(vec![
            Span::raw(pad6.clone()),
            Span::styled("'", tip_yellow),
            Span::styled(" ^~^", flame_orange),
            Span::styled("~'", tip_orange),
            Span::styled(" '", tip_yellow),
        ]),
        _ => Line::from(vec![
            Span::raw(pad6.clone()),
            Span::styled("`", tip_orange),
            Span::styled(" ~^~", flame_orange),
            Span::styled("'^", tip_yellow),
            Span::styled(" `", tip_orange),
        ]),
    };

    // Upper flames
    let upper_flames = match frame {
        0 => Line::from(vec![
            Span::raw(pad4.clone()),
            Span::styled("'", tip_yellow),
            Span::styled(" ~*~", flame_orange),
            Span::styled("'\"'", flame_deep),
            Span::styled("~*~", flame_orange),
            Span::styled(" '", tip_yellow),
        ]),
        1 => Line::from(vec![
            Span::raw(pad4.clone()),
            Span::styled("`", tip_orange),
            Span::styled(" *~*", flame_orange),
            Span::styled("\"'\"", flame_deep),
            Span::styled("*~*", flame_orange),
            Span::styled(" `", tip_orange),
        ]),
        2 => Line::from(vec![
            Span::raw(pad4.clone()),
            Span::styled("'", tip_yellow),
            Span::styled(" ~o~", flame_orange),
            Span::styled("'\"'", flame_deep),
            Span::styled("~o~", flame_orange),
            Span::styled(" '", tip_yellow),
        ]),
        _ => Line::from(vec![
            Span::raw(pad4.clone()),
            Span::styled("`", tip_orange),
            Span::styled(" o~o", flame_orange),
            Span::styled("\"'\"", flame_deep),
            Span::styled("o~o", flame_orange),
            Span::styled(" `", tip_orange),
        ]),
    };

    // Mid flames
    let mid_flames = match frame {
        0 => Line::from(vec![
            Span::raw(pad2.clone()),
            Span::styled(".", tip_yellow),
            Span::styled(" '~\"~", flame_deep),
            Span::styled("@#@", flame_red),
            Span::styled("~\"~'", flame_deep),
            Span::styled(" .", tip_yellow),
        ]),
        1 => Line::from(vec![
            Span::raw(pad2.clone()),
            Span::styled("'", tip_orange),
            Span::styled(" ~\"~'", flame_deep),
            Span::styled("#@#", flame_red),
            Span::styled("'~\"~", flame_deep),
            Span::styled(" '", tip_orange),
        ]),
        2 => Line::from(vec![
            Span::raw(pad2.clone()),
            Span::styled(".", tip_yellow),
            Span::styled(" \"~'~", flame_deep),
            Span::styled("@#@", flame_red),
            Span::styled("~'~\"", flame_deep),
            Span::styled(" .", tip_yellow),
        ]),
        _ => Line::from(vec![
            Span::raw(pad2.clone()),
            Span::styled("'", tip_orange),
            Span::styled(" ~'~\"", flame_deep),
            Span::styled("#@#", flame_red),
            Span::styled("\"~'~", flame_deep),
            Span::styled(" '", tip_orange),
        ]),
    };

    // Lower flames/coals
    let lower_flames = match frame {
        0 => Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("'", tip_orange),
            Span::styled(" `~^~", flame_deep),
            Span::styled("'\"@#@\"'", flame_red),
            Span::styled("~^~`", flame_deep),
            Span::styled(" '", tip_orange),
        ]),
        1 => Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("`", tip_orange),
            Span::styled(" ~^~`", flame_deep),
            Span::styled("\"'#@#'\"", flame_red),
            Span::styled("`~^~", flame_deep),
            Span::styled(" `", tip_orange),
        ]),
        2 => Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("'", tip_orange),
            Span::styled(" ^~`~", flame_deep),
            Span::styled("'@#@#@'", flame_red),
            Span::styled("~`~^", flame_deep),
            Span::styled(" '", tip_orange),
        ]),
        _ => Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("`", tip_orange),
            Span::styled(" ~`~^", flame_deep),
            Span::styled("\"#@#@#\"", flame_red),
            Span::styled("^~`~", flame_deep),
            Span::styled(" `", tip_orange),
        ]),
    };

    // Coals/embers with animated glow
    let coals = match frame {
        0 => Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("▄", coal_dark),
            Span::styled("░▓", coal_glow),
            Span::styled("█▄█", coal_dark),
            Span::styled("▓░▓", coal_glow),
            Span::styled("█▄█", coal_dark),
            Span::styled("▓░", coal_glow),
            Span::styled("▄", coal_dark),
        ]),
        1 => Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("▄", coal_dark),
            Span::styled("▓░", coal_glow),
            Span::styled("█▄█", coal_dark),
            Span::styled("░▓░", coal_glow),
            Span::styled("█▄█", coal_dark),
            Span::styled("░▓", coal_glow),
            Span::styled("▄", coal_dark),
        ]),
        2 => Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("▄", coal_dark),
            Span::styled("░▓", coal_glow),
            Span::styled("█▄█", coal_dark),
            Span::styled("▓░▓", coal_glow),
            Span::styled("█▄█", coal_dark),
            Span::styled("▓░", coal_glow),
            Span::styled("▄", coal_dark),
        ]),
        _ => Line::from(vec![
            Span::raw(pad.clone()),
            Span::styled("▄", coal_dark),
            Span::styled("▓░", coal_glow),
            Span::styled("█▄█", coal_dark),
            Span::styled("░▓░", coal_glow),
            Span::styled("█▄█", coal_dark),
            Span::styled("░▓", coal_glow),
            Span::styled("▄", coal_dark),
        ]),
    };

    // Log arrangement (crossing logs)
    let logs_top = Line::from(vec![
        Span::raw(pad.clone()),
        Span::styled("\\", wood_dark),
        Span::styled("▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄", wood_brown),
        Span::styled("/", wood_dark),
    ]);

    let logs_bottom = Line::from(vec![
        Span::raw(pad.clone()),
        Span::styled(" /", wood_dark),
        Span::styled("▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀", wood_brown),
        Span::styled("\\ ", wood_dark),
    ]);

    vec![
        spark_row,
        flame_tips,
        upper_flames,
        mid_flames,
        lower_flames,
        coals,
        logs_top,
        logs_bottom,
    ]
}
