use bevy::prelude::*;

#[derive(Component)]
pub struct FightScreenRoot;

#[derive(Component)]
pub struct ActionMenuItem {
    pub index: usize,
}

#[derive(Component)]
pub struct PostCombatMenuItem {
    pub index: usize,
}

#[derive(Component)]
pub struct PlayerHealthBar;

#[derive(Component)]
pub struct EnemyHealthBar;

#[derive(Component)]
pub struct CombatLogContainer;

#[derive(Component)]
pub struct PostCombatOverlay;

#[derive(Component)]
pub struct CombatResultText;

#[derive(Component)]
pub struct RewardsText;
