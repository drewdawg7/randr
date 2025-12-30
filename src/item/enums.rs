use rand::{thread_rng, Rng};

use crate::stats::StatSheet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemKind {
    Sword,
    Dagger,
    BasicShield,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemType {
    Weapon,
    Shield
}

pub enum ItemError {
    MaxUpgradesReached,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemQuality {
    Poor,
    Normal,
    Improved,
    WellForged,
    Masterworked,
    Mythic
}
impl ItemQuality {
    /// Returns the human-readable display name for this quality level
    pub fn display_name(&self) -> &'static str {
        match self {
            ItemQuality::Poor => "Poor",
            ItemQuality::Normal => "Normal",
            ItemQuality::Improved => "Improved",
            ItemQuality::WellForged => "Well-Forged",
            ItemQuality::Masterworked => "Masterworked",
            ItemQuality::Mythic => "Mythic",
        }
    }

    pub fn roll() -> Self {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..100);

        match roll {
            0..=9   => ItemQuality::Poor,         // 10%
            10..=69 => ItemQuality::Normal,       // 60%
            70..=84 => ItemQuality::Improved,     // 15%
            85..=94 => ItemQuality::WellForged,   // 10%
            95..=97 => ItemQuality::Masterworked, // 3%
            _       => ItemQuality::Mythic,       // 2%
        }
    }

    pub fn multiplier(&self) -> f64 {
        match self {
            ItemQuality::Poor         => 0.80,
            ItemQuality::Normal       => 1.0,
            ItemQuality::Improved     => 1.2,
            ItemQuality::WellForged   => 1.4,
            ItemQuality::Masterworked => 1.6,
            ItemQuality::Mythic       => 1.8,
        }
    }

    pub fn multiply_stats(&self, mut sheet: StatSheet) -> StatSheet {
        let multiplier = self.multiplier();
        for stat in sheet.stats_mut().values_mut() {
            stat.current_value = ((stat.current_value as f64) * multiplier).round() as i32;
            stat.max_value     = ((stat.max_value     as f64) * multiplier).round() as i32;
        }
        sheet
    } 
}
