use bevy::prelude::*;

use crate::magic::effect::{ActiveEffect, PassiveEffect};
use crate::magic::page::Page;
use crate::magic::spell::ComputedSpell;
use crate::magic::tome::Tome;
use crate::magic::word::WordId;

/// Event fired when a player casts a spell
#[derive(Event, Debug, Clone)]
pub struct SpellCast {
    /// The spell that was cast
    pub spell: ComputedSpell,
    /// Index of the page the spell was cast from
    pub page_index: usize,
}

/// Event fired when a spell effect is applied
#[derive(Event, Debug, Clone)]
pub struct SpellEffectApplied {
    /// The active effect that was applied
    pub effect: ActiveEffect,
    /// Name of the spell that caused this effect
    pub spell_name: String,
}

/// Event fired when a tome is equipped
#[derive(Event, Debug, Clone)]
pub struct TomeEquipped {
    /// The tome that was equipped
    pub tome: Tome,
}

/// Event fired when a page is learned/inscribed
#[derive(Event, Debug, Clone)]
pub struct PageLearned {
    /// The page that was learned
    pub page: Page,
    /// Index in the tome where it was placed
    pub page_index: usize,
    /// The words that were inscribed
    pub words: Vec<WordId>,
}

/// Event fired when passive effects are activated
#[derive(Event, Debug, Clone)]
pub struct PassiveEffectActivated {
    /// The passive effect that was activated
    pub effect: PassiveEffect,
    /// Name of the spell that provides this effect
    pub spell_name: String,
}

/// Event fired when a spell backfires
#[derive(Event, Debug, Clone)]
pub struct SpellBackfired {
    /// Reason for the backfire
    pub reason: String,
    /// The page index where the backfire occurred
    pub page_index: usize,
}

/// Plugin that registers magic-related events
pub struct MagicPlugin;

impl Plugin for MagicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpellCast>()
            .add_event::<SpellEffectApplied>()
            .add_event::<TomeEquipped>()
            .add_event::<PageLearned>()
            .add_event::<PassiveEffectActivated>()
            .add_event::<SpellBackfired>();
    }
}
