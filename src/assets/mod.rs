mod sprite_slices;
mod sprites;

pub use sprite_slices::{
    BookSlotSlice, HealthBarSlice, TravelBookSlice, UiAllSlice, UiSelectorsSlice,
};
pub use sprites::{
    AssetPlugin, GameAssets, GameFonts, GameSprites, SpriteAssets, SpriteSheet, SpriteSheetKey,
};
