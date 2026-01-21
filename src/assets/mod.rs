mod sprite_slices;
mod sprites;

pub use sprite_slices::{
    BookSlotSlice, DetailPanelSlice, DungeonTileSlice, FightBannerSlice, GridSlotSlice,
    HealthBarSlice, ItemDetailIconsSlice, NineSlice, ShopBgSlice, ThreeSlice, TravelBookSlice,
    UiAllSlice, UiSelectorsSlice,
};
pub use sprites::{AssetPlugin, GameFonts, GameSprites, SpriteSheet, SpriteSheetKey};
