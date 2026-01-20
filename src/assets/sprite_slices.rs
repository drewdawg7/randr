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
