use avian2d::prelude::*;

#[derive(PhysicsLayer, Clone, Copy, Debug, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Mob,
    StaticEntity,
    Trigger,
    AttackHitbox,
}

/// Collision layers for static entities (chests, rocks, crafting stations)
pub fn static_entity_layers() -> CollisionLayers {
    CollisionLayers::new(GameLayer::StaticEntity, [GameLayer::Player])
}

/// Collision layers for trigger entities (stairs, doors)
pub fn trigger_layers() -> CollisionLayers {
    CollisionLayers::new(GameLayer::Trigger, [GameLayer::Player])
}

/// Collision layers for mobs and NPCs
pub fn mob_layers() -> CollisionLayers {
    CollisionLayers::new(GameLayer::Mob, [GameLayer::Player, GameLayer::AttackHitbox])
}

pub fn attack_hitbox_layers() -> CollisionLayers {
    CollisionLayers::new(GameLayer::AttackHitbox, [GameLayer::Mob])
}
