//! Aseprite JSON export parser for sprite sheets.
//!
//! This module parses the JSON metadata exported by Aseprite when using
//! "Export Sprite Sheet" with the "JSON Data" option enabled.
//!
//! # Aseprite Export Settings
//! - Format: Hash (recommended) or Array
//! - Check "JSON Data"
//! - Save alongside the PNG file

use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

/// Aseprite JSON export format (Hash mode).
///
/// This represents the structure of the JSON file exported by Aseprite.
#[derive(Debug, Deserialize)]
pub struct AsepriteSheet {
    /// Map of frame names to frame data.
    /// In Hash mode, keys are the frame/slice names.
    pub frames: HashMap<String, AsepriteFrame>,
    /// Metadata about the sprite sheet.
    pub meta: AsepriteMeta,
}

/// A single frame/sprite in the sheet.
#[derive(Debug, Deserialize)]
pub struct AsepriteFrame {
    /// The rectangle defining this frame's position in the sheet.
    pub frame: AsepriteRect,
    /// Whether the frame was rotated during packing.
    #[serde(default)]
    pub rotated: bool,
    /// Whether the frame was trimmed (transparent pixels removed).
    #[serde(default)]
    pub trimmed: bool,
    /// Original sprite size before trimming.
    #[serde(rename = "sourceSize")]
    pub source_size: Option<AsepriteSize>,
    /// Sprite position within the original canvas.
    #[serde(rename = "spriteSourceSize")]
    pub sprite_source_size: Option<AsepriteRect>,
}

/// Rectangle coordinates in the sprite sheet.
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct AsepriteRect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Metadata about the sprite sheet.
#[derive(Debug, Deserialize)]
pub struct AsepriteMeta {
    /// Total size of the sprite sheet image.
    pub size: AsepriteSize,
    /// Original filename (optional).
    #[serde(default)]
    pub image: String,
    /// Application that created this (optional).
    #[serde(default)]
    pub app: String,
    /// Scale factor (optional).
    #[serde(default)]
    pub scale: String,
}

/// Size dimensions.
#[derive(Debug, Deserialize, Clone, Copy)]
pub struct AsepriteSize {
    pub w: u32,
    pub h: u32,
}

impl AsepriteSheet {
    /// Parse an Aseprite JSON export from a string.
    ///
    /// # Example
    /// ```ignore
    /// let json = std::fs::read_to_string("sprites/ui_icons.json")?;
    /// let sheet = AsepriteSheet::load(&json)?;
    /// ```
    pub fn load(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Convert this Aseprite export to a Bevy TextureAtlasLayout.
    ///
    /// Returns both the layout and a mapping from sprite names to atlas indices.
    ///
    /// # Example
    /// ```ignore
    /// let (layout, name_to_index) = sheet.to_layout();
    /// let heart_index = name_to_index.get("heart_full").unwrap();
    /// ```
    pub fn to_layout(&self) -> (TextureAtlasLayout, HashMap<String, usize>) {
        let mut layout =
            TextureAtlasLayout::new_empty(UVec2::new(self.meta.size.w, self.meta.size.h));
        let mut name_to_index = HashMap::new();

        // Sort frames by name for consistent ordering
        let mut frames: Vec<_> = self.frames.iter().collect();
        frames.sort_by_key(|(name, _)| *name);

        for (name, frame) in frames {
            let rect = URect::new(
                frame.frame.x,
                frame.frame.y,
                frame.frame.x + frame.frame.w,
                frame.frame.y + frame.frame.h,
            );
            let index = layout.add_texture(rect);
            name_to_index.insert(name.clone(), index);
        }

        (layout, name_to_index)
    }

    /// Get the names of all frames/sprites in this sheet.
    pub fn frame_names(&self) -> Vec<&str> {
        self.frames.keys().map(|s| s.as_str()).collect()
    }

    /// Get frame data by name.
    pub fn get_frame(&self, name: &str) -> Option<&AsepriteFrame> {
        self.frames.get(name)
    }
}

impl AsepriteRect {
    /// Convert to a Bevy URect.
    pub fn to_urect(&self) -> URect {
        URect::new(self.x, self.y, self.x + self.w, self.y + self.h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_aseprite_json() {
        let json = r#"{
            "frames": {
                "heart_full": {
                    "frame": {"x": 0, "y": 0, "w": 16, "h": 16},
                    "rotated": false,
                    "trimmed": false
                },
                "heart_empty": {
                    "frame": {"x": 16, "y": 0, "w": 16, "h": 16},
                    "rotated": false,
                    "trimmed": false
                }
            },
            "meta": {
                "size": {"w": 32, "h": 16}
            }
        }"#;

        let sheet = AsepriteSheet::load(json).unwrap();
        assert_eq!(sheet.frames.len(), 2);
        assert_eq!(sheet.meta.size.w, 32);
        assert_eq!(sheet.meta.size.h, 16);

        let (layout, name_to_index) = sheet.to_layout();
        assert_eq!(layout.len(), 2);
        assert!(name_to_index.contains_key("heart_full"));
        assert!(name_to_index.contains_key("heart_empty"));
    }
}
