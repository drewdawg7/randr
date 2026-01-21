#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiAllSlice {
    CellBackground,
    HeartIcon,
    GoldIcon,
    TitleBanner,
    InfoPanelBg,
    Book,
    ButtonTown,
    ButtonTownSelected,
    ButtonProfile,
    ButtonProfileSelected,
    ButtonQuit,
    ButtonQuitSelected,
}

impl UiAllSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CellBackground => "Slice_10",
            Self::HeartIcon => "Slice_3013",
            Self::GoldIcon => "Slice_3019",
            Self::TitleBanner => "Slice_3353",
            Self::InfoPanelBg => "Slice_2",
            Self::Book => "Slice_4891",
            Self::ButtonTown => "Slice_295",
            Self::ButtonTownSelected => "Slice_329",
            Self::ButtonProfile => "Slice_193",
            Self::ButtonProfileSelected => "Slice_227",
            Self::ButtonQuit => "Slice_397",
            Self::ButtonQuitSelected => "Slice_431",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiSelectorsSlice {
    SelectorFrame1,
    SelectorFrame2,
}

impl UiSelectorsSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectorFrame1 => "Slice_61",
            Self::SelectorFrame2 => "Slice_91",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HealthBarSlice {
    Health0,
    Health10,
    Health20,
    Health30,
    Health40,
    Health50,
    Health60,
    Health70,
    Health80,
    Health90,
    Health100,
}

impl HealthBarSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Health0 => "Slice_2938",
            Self::Health10 => "Slice_2944",
            Self::Health20 => "Slice_2943",
            Self::Health30 => "Slice_2942",
            Self::Health40 => "Slice_2941",
            Self::Health50 => "Slice_2940",
            Self::Health60 => "Slice_2937",
            Self::Health70 => "Slice_2936",
            Self::Health80 => "Slice_2935",
            Self::Health90 => "Slice_2934",
            Self::Health100 => "Slice_2933",
        }
    }

    pub fn for_percent(percent: f32) -> Self {
        let index = ((percent / 100.0) * 10.0).round() as usize;
        Self::ALL[index.min(10)]
    }

    pub const ALL: [Self; 11] = [
        Self::Health0,
        Self::Health10,
        Self::Health20,
        Self::Health30,
        Self::Health40,
        Self::Health50,
        Self::Health60,
        Self::Health70,
        Self::Health80,
        Self::Health90,
        Self::Health100,
    ];
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TravelBookSlice {
    Banner,
}

impl TravelBookSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Banner => "banner",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BookSlotSlice {
    Slot,
}

impl BookSlotSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Slot => "slot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridSlotSlice {
    Slot,
}

impl GridSlotSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Slot => "slot",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShopBgSlice {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    Center,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl ShopBgSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopLeft => "BGbox_01A 0.aseprite",
            Self::TopCenter => "BGbox_01A 1.aseprite",
            Self::TopRight => "BGbox_01A 2.aseprite",
            Self::MiddleLeft => "BGbox_01A 3.aseprite",
            Self::Center => "BGbox_01A 4.aseprite",
            Self::MiddleRight => "BGbox_01A 5.aseprite",
            Self::BottomLeft => "BGbox_01A 6.aseprite",
            Self::BottomCenter => "BGbox_01A 7.aseprite",
            Self::BottomRight => "BGbox_01A 8.aseprite",
        }
    }

    pub const ALL: [Self; 9] = [
        Self::TopLeft,
        Self::TopCenter,
        Self::TopRight,
        Self::MiddleLeft,
        Self::Center,
        Self::MiddleRight,
        Self::BottomLeft,
        Self::BottomCenter,
        Self::BottomRight,
    ];
}

impl NineSlice for ShopBgSlice {
    const ALL: [Self; 9] = Self::ALL;
    const SLICE_SIZE: f32 = 48.0;
    const SHEET_KEY: SpriteSheetKey = SpriteSheetKey::ShopBgSlices;

    fn as_str(self) -> &'static str {
        Self::as_str(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DetailPanelSlice {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    Center,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl DetailPanelSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopLeft => "BGbox_08A 0.aseprite",
            Self::TopCenter => "BGbox_08A 1.aseprite",
            Self::TopRight => "BGbox_08A 2.aseprite",
            Self::MiddleLeft => "BGbox_08A 3.aseprite",
            Self::Center => "BGbox_08A 4.aseprite",
            Self::MiddleRight => "BGbox_08A 5.aseprite",
            Self::BottomLeft => "BGbox_08A 6.aseprite",
            Self::BottomCenter => "BGbox_08A 7.aseprite",
            Self::BottomRight => "BGbox_08A 8.aseprite",
        }
    }

    pub const ALL: [Self; 9] = [
        Self::TopLeft,
        Self::TopCenter,
        Self::TopRight,
        Self::MiddleLeft,
        Self::Center,
        Self::MiddleRight,
        Self::BottomLeft,
        Self::BottomCenter,
        Self::BottomRight,
    ];
}

impl NineSlice for DetailPanelSlice {
    const ALL: [Self; 9] = Self::ALL;
    const SLICE_SIZE: f32 = 48.0;
    const SHEET_KEY: SpriteSheetKey = SpriteSheetKey::DetailPanelBg;

    fn as_str(self) -> &'static str {
        Self::as_str(self)
    }
}

use crate::stats::StatType;

use super::SpriteSheetKey;

/// Position category for a nine-slice panel cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlicePosition {
    /// Corner cells (fixed size in both dimensions)
    Corner,
    /// Top/bottom edge cells (stretch horizontally)
    TopBottom,
    /// Left/right edge cells (stretch vertically)
    LeftRight,
    /// Center cell (stretch in both dimensions)
    Center,
}

/// Trait for nine-slice panel sprite slices.
///
/// Implementors provide the 9 slices (in row-major order: TL, TC, TR, ML, C, MR, BL, BC, BR),
/// the slice size, and the sprite sheet key to use.
pub trait NineSlice: Copy {
    /// All 9 slices in row-major order.
    const ALL: [Self; 9];
    /// Size of corner slices (edges use this for their fixed dimension).
    const SLICE_SIZE: f32;
    /// The sprite sheet key for this nine-slice set.
    const SHEET_KEY: SpriteSheetKey;

    /// Returns the slice name for sprite lookup.
    fn as_str(self) -> &'static str;

    /// Returns the position category for this slice based on its index in ALL.
    fn position(self) -> SlicePosition {
        let index = Self::ALL.iter().position(|&s| std::mem::discriminant(&s) == std::mem::discriminant(&self)).unwrap_or(0);
        match index {
            0 | 2 | 6 | 8 => SlicePosition::Corner,
            1 | 7 => SlicePosition::TopBottom,
            3 | 5 => SlicePosition::LeftRight,
            4 => SlicePosition::Center,
            _ => SlicePosition::Corner,
        }
    }

    /// Computes the dimensions for this slice given the stretch dimensions.
    fn dimensions(self, stretch_width: f32, stretch_height: f32) -> (f32, f32) {
        match self.position() {
            SlicePosition::Corner => (Self::SLICE_SIZE, Self::SLICE_SIZE),
            SlicePosition::TopBottom => (stretch_width, Self::SLICE_SIZE),
            SlicePosition::LeftRight => (Self::SLICE_SIZE, stretch_height),
            SlicePosition::Center => (stretch_width, stretch_height),
        }
    }
}

/// Position category for a three-slice banner cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThreeSlicePosition {
    /// Left edge (fixed width)
    Left,
    /// Center (stretchable)
    Center,
    /// Right edge (fixed width)
    Right,
}

/// Trait for horizontal 3-slice sprites (left edge, stretchable center, right edge).
///
/// Implementors provide the 3 slices (left, center, right),
/// the edge width, height, and sprite sheet key.
pub trait ThreeSlice: Copy {
    /// All 3 slices in order: left, center, right.
    const ALL: [Self; 3];
    /// Width of left/right edge slices (fixed).
    const EDGE_WIDTH: f32;
    /// Height of all slices (fixed).
    const HEIGHT: f32;
    /// The sprite sheet key for this three-slice set.
    const SHEET_KEY: SpriteSheetKey;

    /// Returns the slice name for sprite lookup.
    fn as_str(self) -> &'static str;

    /// Returns the position category for this slice.
    fn position(self) -> ThreeSlicePosition {
        let index = Self::ALL
            .iter()
            .position(|&s| std::mem::discriminant(&s) == std::mem::discriminant(&self))
            .unwrap_or(0);
        match index {
            0 => ThreeSlicePosition::Left,
            1 => ThreeSlicePosition::Center,
            2 => ThreeSlicePosition::Right,
            _ => ThreeSlicePosition::Left,
        }
    }

    /// Computes the width for this slice given the stretch width.
    fn width(self, stretch_width: f32) -> f32 {
        match self.position() {
            ThreeSlicePosition::Left | ThreeSlicePosition::Right => Self::EDGE_WIDTH,
            ThreeSlicePosition::Center => stretch_width,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FightBannerSlice {
    Left,
    Center,
    Right,
}

impl FightBannerSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Left => "LEFT",
            Self::Center => "CENTER",
            Self::Right => "RIGHT",
        }
    }

    pub const ALL: [Self; 3] = [Self::Left, Self::Center, Self::Right];
}

impl ThreeSlice for FightBannerSlice {
    const ALL: [Self; 3] = Self::ALL;
    const EDGE_WIDTH: f32 = 32.0;
    const HEIGHT: f32 = 39.0;
    const SHEET_KEY: SpriteSheetKey = SpriteSheetKey::FightBannerSlices;

    fn as_str(self) -> &'static str {
        Self::as_str(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemDetailIconsSlice {
    AttackIcon,
    HealthIcon,
    DefenseIcon,
    GoldIcon,
    DefaultStatIcon,
}

impl ItemDetailIconsSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AttackIcon => "attack_icon",
            Self::HealthIcon => "health_icon",
            Self::DefenseIcon => "defense_icon",
            Self::GoldIcon => "gold_icon",
            Self::DefaultStatIcon => "default_stat_icon",
        }
    }

    /// Returns the SpriteSheetKey for this icon slice.
    pub const fn sprite_sheet_key(self) -> SpriteSheetKey {
        match self {
            Self::AttackIcon => SpriteSheetKey::ItemDetailIcons,
            Self::HealthIcon => SpriteSheetKey::HealthIcon,
            Self::DefenseIcon => SpriteSheetKey::DefenseIcon,
            Self::GoldIcon => SpriteSheetKey::GoldIcon,
            Self::DefaultStatIcon => SpriteSheetKey::DefaultStatIcon,
        }
    }

    /// Maps a StatType to its display icon. Reusable across any UI that displays stats.
    pub fn for_stat(stat_type: StatType) -> Self {
        match stat_type {
            StatType::Health => Self::HealthIcon,
            StatType::Attack => Self::AttackIcon,
            StatType::Defense => Self::DefenseIcon,
            _ => Self::DefaultStatIcon, // GoldFind, Mining, MagicFind
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DungeonTileSlice {
    // Floor tiles
    FloorTile1,
    FloorTile2,
    FloorTile3,
    FloorTile4,
    FloorTile5,
    FloorTile6,
    FloorTile7,
    FloorTile8,
    // Floor edges
    FloorEdgeTopLeft,
    FloorEdgeTop1,
    FloorEdgeTop2,
    FloorEdgeTopRight,
    FloorEdgeLeft,
    FloorEdgeLeft2,
    FloorEdgeRight1,
    FloorEdgeRight2,
    // Top walls
    TopWall1,
    TopWall2,
    TopWall3,
    TopWall4,
    // Bottom walls
    BottomWall1,
    BottomWall2,
    BottomWall3,
    BottomWall4,
    // Side walls
    SideWall2,
    SideWall3,
    SideWall4,
    SideWall5,
    SideWall6,
    SideWall7,
    SideWall8,
    // Wall corners
    BottomLeftWall,
    BottomRightWall,
    WallCornerTopLeft,
    // Columns
    WallColumn,
    WallColumnRed1,
    WallColumnRed2,
    WallColumnBlue1,
    WallColumnBlue2,
    // Torches
    TorchWall1,
    TorchWall2,
    TorchWall3,
    TorchWall4,
    // Special
    Gate,
    GateFloor,
    Stairs,
}

impl DungeonTileSlice {
    pub const fn as_str(self) -> &'static str {
        match self {
            // Floor tiles
            Self::FloorTile1 => "floor_tile1",
            Self::FloorTile2 => "floor_tile2",
            Self::FloorTile3 => "floor_tile3",
            Self::FloorTile4 => "floor_tile4",
            Self::FloorTile5 => "floor_tile5",
            Self::FloorTile6 => "floor_tile6",
            Self::FloorTile7 => "floor_tile7",
            Self::FloorTile8 => "floor_tile8",
            // Floor edges
            Self::FloorEdgeTopLeft => "floor_edge_topleft",
            Self::FloorEdgeTop1 => "floor_edge_top1",
            Self::FloorEdgeTop2 => "floor_edge_top2",
            Self::FloorEdgeTopRight => "floor_edge_topright",
            Self::FloorEdgeLeft => "floor_edge_left",
            Self::FloorEdgeLeft2 => "floor_edge_left2",
            Self::FloorEdgeRight1 => "floor_edge_right1",
            Self::FloorEdgeRight2 => "floor_edge_right2",
            // Top walls
            Self::TopWall1 => "top_wall_1",
            Self::TopWall2 => "top_wall_2",
            Self::TopWall3 => "top_wall_3",
            Self::TopWall4 => "top_wall_4",
            // Bottom walls
            Self::BottomWall1 => "bottom_wall_1",
            Self::BottomWall2 => "bottom_wall_2",
            Self::BottomWall3 => "bottom_wall_3",
            Self::BottomWall4 => "bottom_wall_4",
            // Side walls
            Self::SideWall2 => "side_wall_2",
            Self::SideWall3 => "side_wall_3",
            Self::SideWall4 => "side_wall_4",
            Self::SideWall5 => "side_wall_5",
            Self::SideWall6 => "side_wall_6",
            Self::SideWall7 => "side_wall_7",
            Self::SideWall8 => "side_wall_8",
            // Wall corners
            Self::BottomLeftWall => "bottom_left_wall",
            Self::BottomRightWall => "bottom_right_wall",
            Self::WallCornerTopLeft => "wall_corner_topleft",
            // Columns
            Self::WallColumn => "wall_column",
            Self::WallColumnRed1 => "wall_column_red1",
            Self::WallColumnRed2 => "wall_column_red2",
            Self::WallColumnBlue1 => "wall_column_blue1",
            Self::WallColumnBlue2 => "wall_column_blue2",
            // Torches
            Self::TorchWall1 => "torch_wall_1",
            Self::TorchWall2 => "torch_wall_2",
            Self::TorchWall3 => "torch_wall_3",
            Self::TorchWall4 => "torch_wall_4",
            // Special
            Self::Gate => "gate",
            Self::GateFloor => "gate_floor",
            Self::Stairs => "stairs",
        }
    }
}
