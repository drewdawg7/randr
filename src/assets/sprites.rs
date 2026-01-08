use bevy::prelude::*;

/// Plugin that loads and manages game assets.
pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
            .add_systems(PreStartup, load_assets);
    }
}

/// Container for all loaded game assets.
#[derive(Resource, Default)]
pub struct GameAssets {
    pub sprites: SpriteAssets,
}

/// Sprite asset handles for functional game elements.
/// Note: Decorative ASCII art is being removed per the rewrite spec.
#[derive(Default)]
pub struct SpriteAssets {
    // Mine screen sprites
    pub mine_wall: Option<Handle<Image>>,
    pub mine_floor: Option<Handle<Image>>,
    pub mine_rock: Option<Handle<Image>>,
    pub mine_ore: Option<Handle<Image>>,
    pub mine_player: Option<Handle<Image>>,
    pub mine_pickaxe: Option<Handle<Image>>,
    pub mine_ladder: Option<Handle<Image>>,

    // Fight screen sprites
    pub fight_player: Option<Handle<Image>>,
    pub fight_health_bar: Option<Handle<Image>>,
    // Enemy sprites will be loaded dynamically based on mob type

    // Dungeon minimap sprites
    pub dungeon_unexplored: Option<Handle<Image>>,
    pub dungeon_current: Option<Handle<Image>>,
    pub dungeon_cleared: Option<Handle<Image>>,
    pub dungeon_boss: Option<Handle<Image>>,
}

/// System to load assets at startup.
fn load_assets(asset_server: Res<AssetServer>, mut game_assets: ResMut<GameAssets>) {
    // Mine sprites
    game_assets.sprites.mine_wall = try_load(&asset_server, "sprites/mine/wall.png");
    game_assets.sprites.mine_floor = try_load(&asset_server, "sprites/mine/floor.png");
    game_assets.sprites.mine_rock = try_load(&asset_server, "sprites/mine/rock.png");
    game_assets.sprites.mine_ore = try_load(&asset_server, "sprites/mine/ore.png");
    game_assets.sprites.mine_player = try_load(&asset_server, "sprites/mine/player.png");
    game_assets.sprites.mine_pickaxe = try_load(&asset_server, "sprites/mine/pickaxe.png");
    game_assets.sprites.mine_ladder = try_load(&asset_server, "sprites/mine/ladder.png");

    // Fight sprites
    game_assets.sprites.fight_player = try_load(&asset_server, "sprites/fight/player.png");
    game_assets.sprites.fight_health_bar = try_load(&asset_server, "sprites/fight/health_bar.png");

    // Dungeon minimap sprites
    game_assets.sprites.dungeon_unexplored =
        try_load(&asset_server, "sprites/dungeon/unexplored.png");
    game_assets.sprites.dungeon_current = try_load(&asset_server, "sprites/dungeon/current.png");
    game_assets.sprites.dungeon_cleared = try_load(&asset_server, "sprites/dungeon/cleared.png");
    game_assets.sprites.dungeon_boss = try_load(&asset_server, "sprites/dungeon/boss.png");

    info!("Asset loading initiated");
}

/// Try to load an asset, returning None if file doesn't exist.
/// This allows the game to run with placeholder/missing assets during development.
fn try_load(asset_server: &AssetServer, path: &str) -> Option<Handle<Image>> {
    Some(asset_server.load(path))
}

/// Helper to get a sprite handle or a default placeholder.
impl SpriteAssets {
    /// Check if mine sprites are loaded.
    pub fn mine_ready(&self) -> bool {
        self.mine_wall.is_some()
            && self.mine_floor.is_some()
            && self.mine_rock.is_some()
            && self.mine_player.is_some()
    }

    /// Check if fight sprites are loaded.
    pub fn fight_ready(&self) -> bool {
        self.fight_player.is_some() && self.fight_health_bar.is_some()
    }

    /// Check if dungeon minimap sprites are loaded.
    pub fn dungeon_ready(&self) -> bool {
        self.dungeon_unexplored.is_some()
            && self.dungeon_current.is_some()
            && self.dungeon_cleared.is_some()
            && self.dungeon_boss.is_some()
    }
}
