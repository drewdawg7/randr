use rand::Rng;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::item::ItemId;
use crate::loot::LootTable;

/// Base rock shape - spaces are transparent, other chars are rock pixels
const ROCK_SHAPE: &[&str] = &[
    "        ######        ",
    "    ############    ",
    "  ################  ",
    "  ################  ",
    "####################",
    "####################",
    "####################",
    "  ##################  ",
];

/// Block characters for different "densities" of rock
const BLOCK_CHARS: [char; 4] = ['█', '▓', '▒', '░'];

/// Grey color palette for base rock
const GREY_COLORS: [Color; 4] = [
    Color::Rgb(50, 50, 55),   // Dark
    Color::Rgb(70, 70, 75),   // Medium-dark
    Color::Rgb(90, 90, 95),   // Medium
    Color::Rgb(110, 110, 115), // Light
];

/// Maps an ItemId to its ore color, if it's an ore
pub fn ore_color(item_id: ItemId) -> Option<Color> {
    match item_id {
        ItemId::CopperOre => Some(Color::Rgb(184, 115, 51)),  // Copper orange
        ItemId::TinOre => Some(Color::Rgb(180, 180, 190)),    // Silver-ish
        ItemId::Coal => Some(Color::Rgb(25, 25, 25)),         // Dark coal
        _ => None,
    }
}

/// A single cell in the rock art
#[derive(Clone, Debug)]
pub struct RockCell {
    pub ch: char,
    pub color: Color,
}

/// Generated rock art pattern - stored per Rock instance
#[derive(Clone, Debug)]
pub struct RockArt {
    pub cells: Vec<Vec<RockCell>>,
    pub width: usize,
    pub height: usize,
}

impl RockArt {
    /// Generate a new randomized rock art based on loot table proportions
    pub fn generate(loot: &LootTable) -> Self {
        let mut rng = rand::thread_rng();

        // Collect ore colors and their proportions
        let ore_entries: Vec<(Color, f32)> = loot
            .ore_proportions()
            .filter_map(|(item_id, proportion)| {
                ore_color(item_id).map(|color| (color, proportion))
            })
            .collect();

        // Calculate total ore proportion (for normalizing)
        let total_ore: f32 = ore_entries.iter().map(|(_, p)| p).sum();

        // Ore density: how much of the rock should be ore vs grey
        // Scale based on total ore proportion, but cap it so rocks are mostly grey
        let ore_density = (total_ore * 0.25).min(0.35);

        let height = ROCK_SHAPE.len();
        let width = ROCK_SHAPE.iter().map(|s| s.len()).max().unwrap_or(0);

        let mut cells: Vec<Vec<RockCell>> = Vec::with_capacity(height);

        for row_str in ROCK_SHAPE {
            let mut row: Vec<RockCell> = Vec::with_capacity(width);

            for ch in row_str.chars() {
                let cell = if ch == ' ' {
                    // Transparent/empty space
                    RockCell { ch: ' ', color: Color::Reset }
                } else {
                    // Decide if this pixel is ore or grey
                    let is_ore = rng.gen_range(0.0..1.0_f32) < ore_density && !ore_entries.is_empty();

                    if is_ore {
                        // Pick which ore based on proportions
                        let roll = rng.gen_range(0.0..total_ore);
                        let mut cumulative = 0.0;
                        let mut chosen_color = GREY_COLORS[0];

                        for (color, proportion) in &ore_entries {
                            cumulative += proportion;
                            if roll < cumulative {
                                chosen_color = *color;
                                break;
                            }
                        }

                        // Pick a random block character
                        let block_idx = rng.gen_range(0..BLOCK_CHARS.len());
                        RockCell {
                            ch: BLOCK_CHARS[block_idx],
                            color: chosen_color
                        }
                    } else {
                        // Grey rock pixel
                        let grey_idx = rng.gen_range(0..GREY_COLORS.len());
                        let block_idx = rng.gen_range(0..BLOCK_CHARS.len());
                        RockCell {
                            ch: BLOCK_CHARS[block_idx],
                            color: GREY_COLORS[grey_idx]
                        }
                    }
                };
                row.push(cell);
            }

            // Pad row to full width if needed
            while row.len() < width {
                row.push(RockCell { ch: ' ', color: Color::Reset });
            }

            cells.push(row);
        }

        RockArt { cells, width, height }
    }

    /// Generate a simple single-color rock (for rocks with no ore drops)
    pub fn generate_simple(color: Color) -> Self {
        let mut rng = rand::thread_rng();

        let height = ROCK_SHAPE.len();
        let width = ROCK_SHAPE.iter().map(|s| s.len()).max().unwrap_or(0);

        let mut cells: Vec<Vec<RockCell>> = Vec::with_capacity(height);

        for row_str in ROCK_SHAPE {
            let mut row: Vec<RockCell> = Vec::with_capacity(width);

            for ch in row_str.chars() {
                let cell = if ch == ' ' {
                    RockCell { ch: ' ', color: Color::Reset }
                } else {
                    let block_idx = rng.gen_range(0..BLOCK_CHARS.len());
                    RockCell { ch: BLOCK_CHARS[block_idx], color }
                };
                row.push(cell);
            }

            while row.len() < width {
                row.push(RockCell { ch: ' ', color: Color::Reset });
            }

            cells.push(row);
        }

        RockArt { cells, width, height }
    }

    /// Convert to ratatui Lines for rendering
    pub fn to_lines(&self) -> Vec<Line<'static>> {
        self.cells
            .iter()
            .map(|row| {
                let spans: Vec<Span> = row
                    .iter()
                    .map(|cell| {
                        Span::styled(cell.ch.to_string(), Style::default().fg(cell.color))
                    })
                    .collect();
                Line::from(spans)
            })
            .collect()
    }
}
