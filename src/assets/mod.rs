mod sprite_slices;
mod sprites;

pub use sprite_slices::{
    BookSlotSlice, DetailPanelSlice, GridSlotSlice, HealthBarSlice, ShopBgSlice, TravelBookSlice,
    UiAllSlice, UiSelectorsSlice,
};
pub use sprites::{AssetPlugin, GameFonts, GameSprites, SpriteSheet, SpriteSheetKey};
