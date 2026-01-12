//! Typed sprite slice enums for each sprite sheet.
//!
//! These enums provide compile-time safety and semantic naming for sprite slices.
//! Each enum corresponds to a specific `SpriteSheetKey` and contains variants for
//! commonly-used slices from that sheet.
//!
//! # Usage
//! ```rust
//! let cell = sheet.image_node(UiAllSlice::CellBackground.as_str());
//! let idx = selectors.get(UiSelectorsSlice::SelectorFrame1.as_str())?;
//! ```

/// Slices from the UiAll sprite sheet (Cute Fantasy UI pack).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiAllSlice {
    /// Cell background for grids (Slice_10)
    CellBackground,
    /// Heart icon for health display (Slice_3013)
    HeartIcon,
    /// Gold coin icon (Slice_3019)
    GoldIcon,
    /// "randr" title banner (Slice_3353)
    TitleBanner,
    /// Info panel background (Slice_2)
    InfoPanelBg,
    /// Book sprite for compendium (Slice_4891)
    Book,
    /// Menu button: Town unselected (Slice_295)
    ButtonTown,
    /// Menu button: Town selected (Slice_329)
    ButtonTownSelected,
    /// Menu button: Profile unselected (Slice_193)
    ButtonProfile,
    /// Menu button: Profile selected (Slice_227)
    ButtonProfileSelected,
    /// Menu button: Quit unselected (Slice_397)
    ButtonQuit,
    /// Menu button: Quit selected (Slice_431)
    ButtonQuitSelected,
}

impl UiAllSlice {
    /// Returns the string slice name used in sprite sheet lookups.
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

/// Slices from the UiSelectors sprite sheet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiSelectorsSlice {
    /// Selector animation frame 1 (Slice_61)
    SelectorFrame1,
    /// Selector animation frame 2 (Slice_91)
    SelectorFrame2,
}

impl UiSelectorsSlice {
    /// Returns the string slice name used in sprite sheet lookups.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectorFrame1 => "Slice_61",
            Self::SelectorFrame2 => "Slice_91",
        }
    }
}

/// Health bar slices from UiAll sprite sheet.
/// Ordered from empty (0%) to full (100%).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HealthBarSlice {
    /// 0% health - empty bar (Slice_2938)
    Health0,
    /// ~9% health (Slice_2944)
    Health10,
    /// ~18% health (Slice_2943)
    Health20,
    /// ~27% health (Slice_2942)
    Health30,
    /// ~36% health (Slice_2941)
    Health40,
    /// ~45% health (Slice_2940)
    Health50,
    /// ~55% health (Slice_2937)
    Health60,
    /// ~64% health (Slice_2936)
    Health70,
    /// ~73% health (Slice_2935)
    Health80,
    /// ~82% health (Slice_2934)
    Health90,
    /// 91-100% health - full bar (Slice_2933)
    Health100,
}

impl HealthBarSlice {
    /// Returns the string slice name used in sprite sheet lookups.
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

    /// Get the appropriate health bar slice for a percentage (0.0 to 100.0).
    pub fn for_percent(percent: f32) -> Self {
        let index = ((percent / 100.0) * 10.0).round() as usize;
        Self::ALL[index.min(10)]
    }

    /// All health bar slices in order from empty to full.
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

/// Slices from the TravelBook sprite sheet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TravelBookSlice {
    /// Banner background for player stats
    Banner,
}

impl TravelBookSlice {
    /// Returns the string slice name used in sprite sheet lookups.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Banner => "banner",
        }
    }
}

/// Slices from the BookSlot sprite sheet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BookSlotSlice {
    /// Slot sprite for mob display
    Slot,
}

impl BookSlotSlice {
    /// Returns the string slice name used in sprite sheet lookups.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Slot => "slot",
        }
    }
}
