use bevy::prelude::*;

#[derive(Component)]
pub struct FightScreenRoot;

/// Marker component for action menu items (Attack, Run).
/// Use with `MenuIndex` for selection tracking.
#[derive(Component)]
pub struct ActionMenuItem;

/// Marker component for post-combat menu items (Fight Again, Continue).
/// Use with `MenuIndex` for selection tracking.
#[derive(Component)]
pub struct PostCombatMenuItem;

#[derive(Component)]
pub struct PlayerHealthBar;

#[derive(Component)]
pub struct EnemyHealthBar;

/// Marker component for the enemy name label, used to update it when combat initializes.
#[derive(Component)]
pub struct EnemyNameLabel;

#[derive(Component)]
pub struct PostCombatOverlay;

#[derive(Component)]
pub struct CombatResultText;

#[derive(Component)]
pub struct RewardsText;

/// Marker component indicating the fight background needs to be populated.
#[derive(Component)]
pub struct NeedsFightBackground;

/// Marker component indicating the mob sprite needs to be populated.
#[derive(Component)]
pub struct NeedsMobSprite;
