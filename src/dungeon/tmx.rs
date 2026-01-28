//! TMX/TSX map parser for loading Tiled maps into DungeonLayout.
//!
//! Parses Tiled Map Editor files (.tmx) and tilesets (.tsx) to create
//! dungeon layouts with tile properties determining walkability and spawn rules.

use super::grid::GridPosition;
use super::layout::DungeonLayout;
use super::tile::{Tile, TileType};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Properties that can be defined on tiles in the TSX tileset.
///
/// Default: solid, no entity spawning, no player spawning.
/// Only tiles explicitly marked with properties in TSX can be walkable.
#[derive(Debug, Clone)]
pub struct TileProperties {
    pub is_solid: bool,
    pub can_have_entity: bool,
    pub can_spawn_player: bool,
}

impl Default for TileProperties {
    fn default() -> Self {
        Self {
            is_solid: true, // Tiles without properties are walls by default
            can_have_entity: false,
            can_spawn_player: false,
        }
    }
}

/// Parsed tileset data from a TSX file.
#[derive(Debug)]
pub struct Tileset {
    pub name: String,
    pub tile_width: u32,
    pub tile_height: u32,
    pub tile_count: u32,
    pub columns: u32,
    pub image_source: String,
    /// Tile properties indexed by tile ID (0-based, local to tileset).
    pub tile_properties: HashMap<u32, TileProperties>,
}

/// Parsed map data from a TMX file.
#[derive(Debug)]
pub struct TmxMap {
    pub width: u32,
    pub height: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub tileset: Tileset,
    pub tileset_first_gid: u32,
    /// Tile data as global IDs (0 = empty, firstgid+ = tileset tiles).
    pub tile_data: Vec<u32>,
}

/// Errors that can occur during TMX/TSX parsing.
#[derive(Debug)]
pub enum TmxError {
    IoError(std::io::Error),
    ParseError(String),
    MissingAttribute(String),
    MissingElement(String),
}

impl From<std::io::Error> for TmxError {
    fn from(e: std::io::Error) -> Self {
        TmxError::IoError(e)
    }
}

impl std::fmt::Display for TmxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TmxError::IoError(e) => write!(f, "IO error: {}", e),
            TmxError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            TmxError::MissingAttribute(attr) => write!(f, "Missing attribute: {}", attr),
            TmxError::MissingElement(elem) => write!(f, "Missing element: {}", elem),
        }
    }
}

impl std::error::Error for TmxError {}

/// Parse a TSX tileset file.
pub fn parse_tsx(path: &Path) -> Result<Tileset, TmxError> {
    let content = fs::read_to_string(path)?;
    parse_tsx_content(&content)
}

/// Parse TSX content from a string.
fn parse_tsx_content(content: &str) -> Result<Tileset, TmxError> {
    // Find tileset element
    let tileset_start = content
        .find("<tileset")
        .ok_or_else(|| TmxError::MissingElement("tileset".to_string()))?;
    let tileset_end = content[tileset_start..]
        .find('>')
        .ok_or_else(|| TmxError::ParseError("Unclosed tileset tag".to_string()))?;
    let tileset_tag = &content[tileset_start..tileset_start + tileset_end + 1];

    let name = extract_attr(tileset_tag, "name").unwrap_or_default();
    let tile_width = extract_attr(tileset_tag, "tilewidth")
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| TmxError::MissingAttribute("tilewidth".to_string()))?;
    let tile_height = extract_attr(tileset_tag, "tileheight")
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| TmxError::MissingAttribute("tileheight".to_string()))?;
    let tile_count = extract_attr(tileset_tag, "tilecount")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let columns = extract_attr(tileset_tag, "columns")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    // Find image source
    let image_source = if let Some(img_start) = content.find("<image") {
        let img_end = content[img_start..]
            .find("/>")
            .or_else(|| content[img_start..].find('>'))
            .unwrap_or(0);
        let img_tag = &content[img_start..img_start + img_end + 2];
        extract_attr(img_tag, "source").unwrap_or_default()
    } else {
        String::new()
    };

    // Parse tile properties
    let mut tile_properties = HashMap::new();
    let mut search_pos = 0;

    while let Some(tile_start) = content[search_pos..].find("<tile ") {
        let abs_start = search_pos + tile_start;
        let tile_end = content[abs_start..]
            .find("</tile>")
            .or_else(|| content[abs_start..].find("/>"))
            .unwrap_or(content.len() - abs_start);
        let tile_section = &content[abs_start..abs_start + tile_end + 7.min(content.len() - abs_start - tile_end)];

        // Get tile ID
        if let Some(id) = extract_attr(tile_section, "id").and_then(|s| s.parse::<u32>().ok()) {
            let mut props = TileProperties::default();

            // Parse properties within this tile
            let mut prop_pos = 0;
            while let Some(prop_start) = tile_section[prop_pos..].find("<property ") {
                let abs_prop_start = prop_pos + prop_start;
                let prop_end = tile_section[abs_prop_start..]
                    .find("/>")
                    .unwrap_or(tile_section.len() - abs_prop_start);
                let prop_tag = &tile_section[abs_prop_start..abs_prop_start + prop_end + 2];

                if let (Some(name), Some(value)) =
                    (extract_attr(prop_tag, "name"), extract_attr(prop_tag, "value"))
                {
                    match name.as_str() {
                        "is_solid" => props.is_solid = value == "true",
                        "can_have_entity" => props.can_have_entity = value == "true",
                        "can_spawn_player" => props.can_spawn_player = value == "true",
                        _ => {}
                    }
                }
                prop_pos = abs_prop_start + prop_end + 2;
            }

            tile_properties.insert(id, props);
        }

        search_pos = abs_start + tile_end + 1;
    }

    Ok(Tileset {
        name,
        tile_width,
        tile_height,
        tile_count,
        columns,
        image_source,
        tile_properties,
    })
}

/// Parse a TMX map file, loading its referenced tileset.
pub fn parse_tmx(path: &Path) -> Result<TmxMap, TmxError> {
    let content = fs::read_to_string(path)?;
    parse_tmx_content(&content, path.parent())
}

/// Parse TMX content from a string.
fn parse_tmx_content(content: &str, base_path: Option<&Path>) -> Result<TmxMap, TmxError> {
    // Find map element
    let map_start = content
        .find("<map")
        .ok_or_else(|| TmxError::MissingElement("map".to_string()))?;
    let map_end = content[map_start..]
        .find('>')
        .ok_or_else(|| TmxError::ParseError("Unclosed map tag".to_string()))?;
    let map_tag = &content[map_start..map_start + map_end + 1];

    let width = extract_attr(map_tag, "width")
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| TmxError::MissingAttribute("width".to_string()))?;
    let height = extract_attr(map_tag, "height")
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| TmxError::MissingAttribute("height".to_string()))?;
    let tile_width = extract_attr(map_tag, "tilewidth")
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| TmxError::MissingAttribute("tilewidth".to_string()))?;
    let tile_height = extract_attr(map_tag, "tileheight")
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| TmxError::MissingAttribute("tileheight".to_string()))?;

    // Find tileset reference
    let tileset_start = content
        .find("<tileset")
        .ok_or_else(|| TmxError::MissingElement("tileset".to_string()))?;
    let tileset_end = content[tileset_start..]
        .find("/>")
        .or_else(|| content[tileset_start..].find('>'))
        .ok_or_else(|| TmxError::ParseError("Unclosed tileset tag".to_string()))?;
    let tileset_tag = &content[tileset_start..tileset_start + tileset_end + 2];

    let first_gid = extract_attr(tileset_tag, "firstgid")
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| TmxError::MissingAttribute("firstgid".to_string()))?;

    // Load external tileset
    let tileset = if let Some(tsx_source) = extract_attr(tileset_tag, "source") {
        let tsx_path = if let Some(base) = base_path {
            base.join(&tsx_source)
        } else {
            Path::new(&tsx_source).to_path_buf()
        };
        parse_tsx(&tsx_path)?
    } else {
        // Embedded tileset (parse from same content)
        parse_tsx_content(content)?
    };

    // Parse tile layer data
    let data_start = content
        .find("<data")
        .ok_or_else(|| TmxError::MissingElement("data".to_string()))?;
    let data_content_start = content[data_start..]
        .find('>')
        .ok_or_else(|| TmxError::ParseError("Unclosed data tag".to_string()))?;
    let data_content_end = content[data_start..]
        .find("</data>")
        .ok_or_else(|| TmxError::MissingElement("</data>".to_string()))?;

    let csv_data = &content[data_start + data_content_start + 1..data_start + data_content_end];
    let tile_data: Vec<u32> = csv_data
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    Ok(TmxMap {
        width,
        height,
        tile_width,
        tile_height,
        tileset,
        tileset_first_gid: first_gid,
        tile_data,
    })
}

/// Extract an attribute value from an XML tag string.
fn extract_attr(tag: &str, attr_name: &str) -> Option<String> {
    let pattern = format!("{}=\"", attr_name);
    if let Some(start) = tag.find(&pattern) {
        let value_start = start + pattern.len();
        if let Some(end) = tag[value_start..].find('"') {
            return Some(tag[value_start..value_start + end].to_string());
        }
    }
    None
}

/// Info needed to render tiles directly from tileset.
#[derive(Debug, Clone)]
pub struct TilesetRenderInfo {
    pub columns: u32,
    pub first_gid: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub image_path: String,
}

impl TmxMap {
    /// Get tileset rendering info for direct sprite lookup.
    pub fn tileset_render_info(&self) -> TilesetRenderInfo {
        TilesetRenderInfo {
            columns: self.tileset.columns,
            first_gid: self.tileset_first_gid,
            tile_width: self.tileset.tile_width,
            tile_height: self.tileset.tile_height,
            image_path: self.tileset.image_source.clone(),
        }
    }

    /// Convert the TMX map to a DungeonLayout.
    ///
    /// Maps TMX 1:1 without any modifications:
    /// - Exact TMX dimensions are used (no trimming)
    /// - Tile ID stored in each tile for direct sprite lookup
    /// - Tile ID 0 → TileType::Empty (not rendered)
    /// - `is_solid=true` → TileType::Wall
    /// - `is_solid=false` → TileType::Floor
    /// - `can_spawn_player=true` → candidate for player spawn
    pub fn to_layout(&self) -> DungeonLayout {
        let width = self.width as usize;
        let height = self.height as usize;

        let mut layout = DungeonLayout::new(width, height);
        let mut spawn_candidates: Vec<(usize, usize)> = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let gid = self.tile_data.get(idx).copied().unwrap_or(0);

                let tile_type = if gid == 0 {
                    // Tile ID 0 = empty, not rendered
                    TileType::Empty
                } else {
                    let local_id = gid - self.tileset_first_gid;
                    let props = self
                        .tileset
                        .tile_properties
                        .get(&local_id)
                        .cloned()
                        .unwrap_or_default();

                    if props.can_spawn_player {
                        spawn_candidates.push((x, y));
                    }

                    if props.is_solid {
                        TileType::Wall
                    } else {
                        TileType::Floor
                    }
                };

                // Store the exact tileset ID for direct rendering
                let tile = Tile::new(tile_type).with_tileset_id(gid);
                layout.set_tile(x, y, tile);
            }
        }

        // Set entrance to first spawn candidate or center
        if let Some(&(x, y)) = spawn_candidates.first() {
            layout.entrance = (x, y);
            // Keep the tileset_id when setting spawn point
            if let Some(existing) = layout.tile_at(x, y) {
                let tileset_id = existing.tileset_id;
                let mut tile = Tile::new(TileType::SpawnPoint);
                if let Some(id) = tileset_id {
                    tile = tile.with_tileset_id(id);
                }
                layout.set_tile(x, y, tile);
            }
        } else {
            layout.entrance = (width / 2, height / 2);
        }

        layout
    }

    /// Get all tiles marked as valid for entity spawning.
    pub fn entity_spawn_positions(&self) -> Vec<GridPosition> {
        let width = self.width as usize;
        let height = self.height as usize;
        let mut positions = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let gid = self.tile_data.get(idx).copied().unwrap_or(0);

                if gid > 0 {
                    let local_id = gid - self.tileset_first_gid;
                    if let Some(props) = self.tileset.tile_properties.get(&local_id) {
                        if props.can_have_entity && !props.is_solid {
                            positions.push(GridPosition::new(x, y));
                        }
                    }
                }
            }
        }

        positions
    }

    /// Get all tiles marked as valid for player spawning.
    pub fn player_spawn_positions(&self) -> Vec<GridPosition> {
        let width = self.width as usize;
        let height = self.height as usize;
        let mut positions = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let gid = self.tile_data.get(idx).copied().unwrap_or(0);

                if gid > 0 {
                    let local_id = gid - self.tileset_first_gid;
                    if let Some(props) = self.tileset.tile_properties.get(&local_id) {
                        if props.can_spawn_player && !props.is_solid {
                            positions.push(GridPosition::new(x, y));
                        }
                    }
                }
            }
        }

        positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tsx_content() {
        let tsx = r#"<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" name="cave" tilewidth="32" tileheight="32" tilecount="280" columns="14">
 <image source="Cave Tileset.png" width="448" height="640"/>
 <tile id="0">
  <properties>
   <property name="is_solid" type="bool" value="true"/>
  </properties>
 </tile>
 <tile id="70">
  <properties>
   <property name="can_have_entity" type="bool" value="true"/>
   <property name="can_spawn_player" type="bool" value="true"/>
   <property name="is_solid" type="bool" value="false"/>
  </properties>
 </tile>
</tileset>"#;

        let tileset = parse_tsx_content(tsx).unwrap();
        assert_eq!(tileset.name, "cave");
        assert_eq!(tileset.tile_width, 32);
        assert_eq!(tileset.tile_height, 32);
        assert_eq!(tileset.tile_count, 280);
        assert_eq!(tileset.columns, 14);

        let props_0 = tileset.tile_properties.get(&0).unwrap();
        assert!(props_0.is_solid);
        assert!(!props_0.can_have_entity);

        let props_70 = tileset.tile_properties.get(&70).unwrap();
        assert!(!props_70.is_solid);
        assert!(props_70.can_have_entity);
        assert!(props_70.can_spawn_player);
    }

    #[test]
    fn test_parse_tmx_to_layout() {
        let tsx = r#"<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" name="test" tilewidth="32" tileheight="32" tilecount="4" columns="2">
 <tile id="0"><properties><property name="is_solid" type="bool" value="true"/></properties></tile>
 <tile id="1"><properties><property name="is_solid" type="bool" value="false"/><property name="can_spawn_player" type="bool" value="true"/></properties></tile>
</tileset>"#;

        // TMX content shown for reference (not parsed in this unit test)
        let _tmx = r#"<?xml version="1.0" encoding="UTF-8"?>
<map version="1.10" width="3" height="3" tilewidth="32" tileheight="32">
 <tileset firstgid="1" source="test.tsx"/>
 <layer id="1" name="Tile Layer 1" width="3" height="3">
  <data encoding="csv">
1,1,1,
1,2,1,
1,1,1
</data>
 </layer>
</map>"#;

        // Parse tileset directly for this test
        let tileset = parse_tsx_content(tsx).unwrap();

        // Create map with embedded tileset for test
        let map = TmxMap {
            width: 3,
            height: 3,
            tile_width: 32,
            tile_height: 32,
            tileset,
            tileset_first_gid: 1,
            tile_data: vec![1, 1, 1, 1, 2, 1, 1, 1, 1],
        };

        let layout = map.to_layout();
        assert_eq!(layout.width(), 3);
        assert_eq!(layout.height(), 3);

        // Center tile (1,1) should be walkable floor
        assert!(layout.is_walkable(1, 1));
        // Edge tiles should be walls
        assert!(!layout.is_walkable(0, 0));
    }
}
