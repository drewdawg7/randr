use crate::assets::SpriteSheetKey;

#[derive(Debug, Clone)]
pub struct SpriteInfo {
    pub name: String,
    pub sheet_key: SpriteSheetKey,
}
