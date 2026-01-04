use rand::Rng;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::loot::LootTable;
use crate::ui::theme as colors;
use crate::location::mine::rock::spec::specs::{COPPER_ROCK, COAL_ROCK, TIN_ROCK};

const CAVE_WIDTH: usize = 60;
const CAVE_HEIGHT: usize = 20;

/// Rock symbol (Nerd Font)
const ROCK_SYMBOL: char = '\u{e88a}';

/// Player symbol (Nerd Font)
const PLAYER_SYMBOL: char = '\u{f183}';

/// Pickaxe symbol (Nerd Font) - shown when adjacent to rock
const PICKAXE_SYMBOL: char = '\u{F08B7}';

/// Exit ladder symbol (Nerd Font)
const LADDER_SYMBOL: char = '\u{F15A2}';

/// Exit indicator arrow (Nerd Font) - shown when on ladder
const EXIT_ARROW_SYMBOL: char = '\u{F062}';

/// Rock types with spawn weights (Copper: 50, Coal: 30, Tin: 20)
#[derive(Clone, Copy)]
pub enum RockType {
    Copper,
    Coal,
    Tin,
}

impl RockType {
    /// Get the color for this rock type
    fn color(&self) -> Color {
        match self {
            RockType::Copper => colors::COPPER_ORE,
            RockType::Coal => colors::COAL_ORE,
            RockType::Tin => colors::TIN_ORE,
        }
    }

    /// Get the loot table for this rock type
    pub fn loot_table(&self) -> &LootTable {
        match self {
            RockType::Copper => &COPPER_ROCK.loot,
            RockType::Coal => &COAL_ROCK.loot,
            RockType::Tin => &TIN_ROCK.loot,
        }
    }

    /// Randomly select a rock type based on spawn weights
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..100);
        if roll < 50 {
            RockType::Copper
        } else if roll < 80 {
            RockType::Coal
        } else {
            RockType::Tin
        }
    }
}

/// A rock placed in the cave
#[derive(Clone, Copy)]
pub struct CaveRock {
    pub x: usize,
    pub y: usize,
    pub rock_type: RockType,
}

/// Cell types in the cave
#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Floor,
}

/// Generated cave layout
pub struct CaveLayout {
    cells: [[Cell; CAVE_WIDTH]; CAVE_HEIGHT],
    rocks: Vec<CaveRock>,
    player_x: usize,
    player_y: usize,
    exit_x: usize,
    exit_y: usize,
}

impl CaveLayout {
    /// Generate a new procedural cave using cellular automata
    pub fn generate() -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = [[Cell::Wall; CAVE_WIDTH]; CAVE_HEIGHT];

        // Step 1: Seed the center with guaranteed floor (large ellipse)
        let center_x = CAVE_WIDTH / 2;
        let center_y = CAVE_HEIGHT / 2;
        let radius_x = CAVE_WIDTH / 3;  // Horizontal radius
        let radius_y = CAVE_HEIGHT / 3; // Vertical radius

        for y in 3..CAVE_HEIGHT - 3 {
            for x in 3..CAVE_WIDTH - 3 {
                // Check if inside ellipse (with some randomness at edges)
                let dx = (x as f32 - center_x as f32) / radius_x as f32;
                let dy = (y as f32 - center_y as f32) / radius_y as f32;
                let dist = dx * dx + dy * dy;

                if dist < 0.7 {
                    // Inside core - always floor
                    cells[y][x] = Cell::Floor;
                } else if dist < 1.2 && rng.gen_bool(0.65) {
                    // Edge zone - probabilistic floor
                    cells[y][x] = Cell::Floor;
                }
            }
        }

        // Step 2: Run cellular automata iterations to smooth edges
        for _ in 0..4 {
            cells = Self::automata_step(&cells);
        }

        // Step 3: Ensure edges are always walls (3 cells thick)
        for y in 0..CAVE_HEIGHT {
            for x in 0..CAVE_WIDTH {
                if x < 3 || x >= CAVE_WIDTH - 3 || y < 3 || y >= CAVE_HEIGHT - 3 {
                    cells[y][x] = Cell::Wall;
                }
            }
        }

        // Step 4: Keep only the largest connected floor region
        cells = Self::keep_largest_region(cells);

        // Step 5: Find exit position (as centered as possible)
        let (exit_x, exit_y) = Self::find_center_floor(&cells);

        // Step 6: Place 6-8 rocks randomly on floor tiles (avoiding exit)
        let rocks = Self::place_rocks(&cells, exit_x, exit_y);

        // Step 7: Find player spawn point (center of floor area, avoiding exit and rocks)
        let (player_x, player_y) = Self::find_spawn_point(&cells, &rocks, exit_x, exit_y);

        Self { cells, rocks, player_x, player_y, exit_x, exit_y }
    }

    /// Find the most centered floor tile
    fn find_center_floor(cells: &[[Cell; CAVE_WIDTH]; CAVE_HEIGHT]) -> (usize, usize) {
        let center_x = CAVE_WIDTH / 2;
        let center_y = CAVE_HEIGHT / 2;

        // Spiral outward from center to find a floor tile
        for radius in 0..CAVE_WIDTH.max(CAVE_HEIGHT) {
            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    let x = (center_x as i32 + dx) as usize;
                    let y = (center_y as i32 + dy) as usize;

                    if x < CAVE_WIDTH && y < CAVE_HEIGHT && cells[y][x] == Cell::Floor {
                        return (x, y);
                    }
                }
            }
        }

        (center_x, center_y)
    }

    /// Find a spawn point for the player near the center (avoiding exit and rocks)
    fn find_spawn_point(cells: &[[Cell; CAVE_WIDTH]; CAVE_HEIGHT], rocks: &[CaveRock], exit_x: usize, exit_y: usize) -> (usize, usize) {
        let center_x = CAVE_WIDTH / 2;
        let center_y = CAVE_HEIGHT / 2;

        // Spiral outward from center to find a valid floor tile
        for radius in 1..CAVE_WIDTH.max(CAVE_HEIGHT) {
            for dy in -(radius as i32)..=(radius as i32) {
                for dx in -(radius as i32)..=(radius as i32) {
                    let x = (center_x as i32 + dx) as usize;
                    let y = (center_y as i32 + dy) as usize;

                    if x < CAVE_WIDTH && y < CAVE_HEIGHT {
                        if cells[y][x] == Cell::Floor {
                            // Skip exit position
                            if x == exit_x && y == exit_y {
                                continue;
                            }
                            // Check no rock at this position
                            let has_rock = rocks.iter().any(|r| r.x == x && r.y == y);
                            if !has_rock {
                                return (x, y);
                            }
                        }
                    }
                }
            }
        }

        // Fallback to center (shouldn't happen)
        (center_x, center_y)
    }

    /// Try to move the player in a direction. Returns true if moved.
    pub fn move_player(&mut self, dx: i32, dy: i32) -> bool {
        let new_x = (self.player_x as i32 + dx) as usize;
        let new_y = (self.player_y as i32 + dy) as usize;

        // Check bounds
        if new_x >= CAVE_WIDTH || new_y >= CAVE_HEIGHT {
            return false;
        }

        // Check wall collision
        if self.cells[new_y][new_x] == Cell::Wall {
            return false;
        }

        // Check rock collision
        let has_rock = self.rocks.iter().any(|r| r.x == new_x && r.y == new_y);
        if has_rock {
            return false;
        }

        // Move player
        self.player_x = new_x;
        self.player_y = new_y;
        true
    }

    /// Check if player is adjacent to any rock. Returns the index if so.
    pub fn adjacent_rock_index(&self) -> Option<usize> {
        for (i, rock) in self.rocks.iter().enumerate() {
            let dx = (rock.x as i32 - self.player_x as i32).abs();
            let dy = (rock.y as i32 - self.player_y as i32).abs();
            // Adjacent means within 1 cell (including diagonals)
            if dx <= 1 && dy <= 1 && !(dx == 0 && dy == 0) {
                return Some(i);
            }
        }
        None
    }

    /// Check if player is adjacent to any rock
    pub fn is_adjacent_to_rock(&self) -> bool {
        self.adjacent_rock_index().is_some()
    }

    /// Mine the adjacent rock, removing it and returning its type
    pub fn mine_adjacent_rock(&mut self) -> Option<RockType> {
        if let Some(idx) = self.adjacent_rock_index() {
            let rock = self.rocks.remove(idx);
            Some(rock.rock_type)
        } else {
            None
        }
    }

    /// Check if player is standing on the exit ladder
    pub fn is_on_exit(&self) -> bool {
        self.player_x == self.exit_x && self.player_y == self.exit_y
    }

    /// Place 6-8 rocks uniformly spread across the cave floor (avoiding exit)
    fn place_rocks(cells: &[[Cell; CAVE_WIDTH]; CAVE_HEIGHT], exit_x: usize, exit_y: usize) -> Vec<CaveRock> {
        let mut rng = rand::thread_rng();
        let rock_count = rng.gen_range(6..=8);

        // Divide cave into a 3x3 grid of zones for uniform distribution
        let zone_width = CAVE_WIDTH / 3;
        let zone_height = CAVE_HEIGHT / 3;

        // Collect floor positions grouped by zone (excluding exit)
        let mut zones: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 9];
        for y in 0..CAVE_HEIGHT {
            for x in 0..CAVE_WIDTH {
                if cells[y][x] == Cell::Floor && !(x == exit_x && y == exit_y) {
                    let zone_x = (x / zone_width).min(2);
                    let zone_y = (y / zone_height).min(2);
                    let zone_idx = zone_y * 3 + zone_x;
                    zones[zone_idx].push((x, y));
                }
            }
        }

        // Get zones that have floor tiles, shuffle them
        let mut available_zones: Vec<usize> = zones
            .iter()
            .enumerate()
            .filter(|(_, z)| !z.is_empty())
            .map(|(i, _)| i)
            .collect();

        let mut rocks = Vec::new();
        let mut zone_idx = 0;

        for _ in 0..rock_count {
            if available_zones.is_empty() {
                break;
            }

            // Cycle through zones to spread rocks
            let zone = available_zones[zone_idx % available_zones.len()];
            zone_idx += 1;

            if !zones[zone].is_empty() {
                let pos_idx = rng.gen_range(0..zones[zone].len());
                let (x, y) = zones[zone].remove(pos_idx);
                rocks.push(CaveRock {
                    x,
                    y,
                    rock_type: RockType::random(),
                });
            }

            // Remove empty zones
            available_zones.retain(|&z| !zones[z].is_empty());
        }

        rocks
    }

    /// Find all connected floor regions and keep only the largest one
    fn keep_largest_region(mut cells: [[Cell; CAVE_WIDTH]; CAVE_HEIGHT]) -> [[Cell; CAVE_WIDTH]; CAVE_HEIGHT] {
        let mut visited = [[false; CAVE_WIDTH]; CAVE_HEIGHT];
        let mut regions: Vec<Vec<(usize, usize)>> = Vec::new();

        // Find all connected floor regions using flood fill
        for y in 0..CAVE_HEIGHT {
            for x in 0..CAVE_WIDTH {
                if cells[y][x] == Cell::Floor && !visited[y][x] {
                    let region = Self::flood_fill(&cells, &mut visited, x, y);
                    regions.push(region);
                }
            }
        }

        // Find the largest region
        if let Some(largest) = regions.iter().max_by_key(|r| r.len()) {
            // Convert largest region to a set for fast lookup
            let largest_set: std::collections::HashSet<(usize, usize)> =
                largest.iter().copied().collect();

            // Fill all non-largest floor cells as walls
            for y in 0..CAVE_HEIGHT {
                for x in 0..CAVE_WIDTH {
                    if cells[y][x] == Cell::Floor && !largest_set.contains(&(x, y)) {
                        cells[y][x] = Cell::Wall;
                    }
                }
            }
        }

        cells
    }

    /// Flood fill to find all connected floor cells starting from (start_x, start_y)
    fn flood_fill(
        cells: &[[Cell; CAVE_WIDTH]; CAVE_HEIGHT],
        visited: &mut [[bool; CAVE_WIDTH]; CAVE_HEIGHT],
        start_x: usize,
        start_y: usize,
    ) -> Vec<(usize, usize)> {
        let mut region = Vec::new();
        let mut stack = vec![(start_x, start_y)];

        while let Some((x, y)) = stack.pop() {
            if visited[y][x] {
                continue;
            }
            if cells[y][x] != Cell::Floor {
                continue;
            }

            visited[y][x] = true;
            region.push((x, y));

            // Add 4-directional neighbors
            if x > 0 { stack.push((x - 1, y)); }
            if x < CAVE_WIDTH - 1 { stack.push((x + 1, y)); }
            if y > 0 { stack.push((x, y - 1)); }
            if y < CAVE_HEIGHT - 1 { stack.push((x, y + 1)); }
        }

        region
    }

    /// Single cellular automata step - walls grow if they have many wall neighbors
    fn automata_step(cells: &[[Cell; CAVE_WIDTH]; CAVE_HEIGHT]) -> [[Cell; CAVE_WIDTH]; CAVE_HEIGHT] {
        let mut new_cells = *cells;

        for y in 1..CAVE_HEIGHT - 1 {
            for x in 1..CAVE_WIDTH - 1 {
                let wall_count = Self::count_wall_neighbors(cells, x, y);

                // Rule: become wall if 5+ neighbors are walls, become floor if <4 neighbors are walls
                if wall_count >= 5 {
                    new_cells[y][x] = Cell::Wall;
                } else if wall_count < 4 {
                    new_cells[y][x] = Cell::Floor;
                }
            }
        }

        new_cells
    }

    /// Count wall neighbors in 3x3 area around cell
    fn count_wall_neighbors(cells: &[[Cell; CAVE_WIDTH]; CAVE_HEIGHT], x: usize, y: usize) -> usize {
        let mut count = 0;
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if cells[ny][nx] == Cell::Wall {
                    count += 1;
                }
            }
        }
        count
    }

    /// Convert cell to display character based on neighbors
    fn cell_to_char(&self, x: usize, y: usize) -> char {
        if self.cells[y][x] == Cell::Floor {
            return ' ';
        }

        // Count floor neighbors to determine wall edge character
        let mut floor_neighbors = 0;
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < CAVE_WIDTH as i32 && ny >= 0 && ny < CAVE_HEIGHT as i32 {
                    if self.cells[ny as usize][nx as usize] == Cell::Floor {
                        floor_neighbors += 1;
                    }
                }
            }
        }

        // More floor neighbors = lighter wall character (edge)
        match floor_neighbors {
            0 => '#',      // Deep wall
            1 => '@',      // Mostly wall
            2 => '%',      // Wall edge
            _ => ';',      // Floor edge
        }
    }

    /// Get style for character
    fn char_style(ch: char) -> Style {
        match ch {
            '#' => Style::default().fg(colors::CAVE_WALL_DARK),
            '@' => Style::default().fg(colors::CAVE_WALL_MID),
            '%' => Style::default().fg(colors::CAVE_WALL_LIGHT),
            ';' => Style::default().fg(colors::CAVE_FLOOR_EDGE),
            _ => Style::default(),
        }
    }

    /// Render the cave to lines
    pub fn to_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        for y in 0..CAVE_HEIGHT {
            let mut spans = Vec::new();
            let mut current_char = self.cell_to_char(0, y);
            let mut current_style = Self::char_style(current_char);
            let mut current_text = String::new();
            current_text.push(current_char);

            for x in 1..CAVE_WIDTH {
                let ch = self.cell_to_char(x, y);
                let style = Self::char_style(ch);

                if style != current_style {
                    spans.push(Span::styled(current_text.clone(), current_style));
                    current_text.clear();
                    current_style = style;
                }
                current_text.push(ch);
            }

            if !current_text.is_empty() {
                spans.push(Span::styled(current_text, current_style));
            }

            lines.push(Line::from(spans));
        }

        lines
    }
}

/// Renders a cave layout centered in the given area
pub fn render_cave(frame: &mut Frame, area: Rect, cave: &CaveLayout) {
    // Calculate centering offsets
    let x_offset = area.x + area.width.saturating_sub(CAVE_WIDTH as u16) / 2;
    let y_offset = area.y + area.height.saturating_sub(CAVE_HEIGHT as u16) / 2;

    let cave_area = Rect {
        x: x_offset,
        y: y_offset,
        width: CAVE_WIDTH as u16,
        height: CAVE_HEIGHT as u16,
    };

    let bg_style = Style::default().bg(colors::CAVE_FLOOR_BG);
    let lines = cave.to_lines();

    frame.render_widget(
        Paragraph::new(lines).style(bg_style),
        cave_area,
    );

    // Render exit ladder
    let buf = frame.buffer_mut();
    let exit_screen_x = x_offset + cave.exit_x as u16;
    let exit_screen_y = y_offset + cave.exit_y as u16;
    if let Some(cell) = buf.cell_mut((exit_screen_x, exit_screen_y)) {
        cell.set_char(LADDER_SYMBOL);
        cell.set_fg(colors::WOOD_BROWN);
    }

    // Render rocks on top
    for rock in &cave.rocks {
        let screen_x = x_offset + rock.x as u16;
        let screen_y = y_offset + rock.y as u16;

        if let Some(cell) = buf.cell_mut((screen_x, screen_y)) {
            cell.set_char(ROCK_SYMBOL);
            cell.set_fg(rock.rock_type.color());
        }
    }

    // Render player
    let player_screen_x = x_offset + cave.player_x as u16;
    let player_screen_y = y_offset + cave.player_y as u16;
    if let Some(cell) = buf.cell_mut((player_screen_x, player_screen_y)) {
        cell.set_char(PLAYER_SYMBOL);
        cell.set_fg(colors::WHITE);
    }

    // Render indicator above player
    if player_screen_y > 0 {
        let indicator_y = player_screen_y - 1;
        if cave.is_on_exit() {
            // Exit arrow when on ladder
            if let Some(cell) = buf.cell_mut((player_screen_x, indicator_y)) {
                cell.set_char(EXIT_ARROW_SYMBOL);
                cell.set_fg(colors::YELLOW);
            }
        } else if cave.is_adjacent_to_rock() {
            // Pickaxe when adjacent to rock
            if let Some(cell) = buf.cell_mut((player_screen_x, indicator_y)) {
                cell.set_char(PICKAXE_SYMBOL);
                cell.set_fg(colors::YELLOW);
            }
        }
    }
}
