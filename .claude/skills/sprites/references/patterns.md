# Sprite Patterns

## Marker + System Pattern for UI Widgets

When adding sprites to UI widgets built with `ChildBuilder`, you can't access `Res<GameSprites>`. Use markers:

```rust
/// Marker component for sprite placeholder
#[derive(Component)]
struct HeartIconPlaceholder;

/// Widget function - spawns placeholder (no GameSprites needed)
pub fn spawn_health_display(parent: &mut ChildBuilder, hp: i32, max_hp: i32) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(4.0),
        ..default()
    }).with_children(|row| {
        // Placeholder - will be populated by system
        row.spawn((
            HeartIconPlaceholder,
            Node { width: Val::Px(16.0), height: Val::Px(16.0), ..default() },
        ));
        row.spawn(Text::new(format!("{}/{}", hp, max_hp)));
    });
}

/// System that populates placeholders with actual sprites
fn populate_heart_icons(
    mut commands: Commands,
    query: Query<Entity, With<HeartIconPlaceholder>>,
    game_sprites: Res<GameSprites>,
) {
    let Some(sheet) = &game_sprites.ui_all else { return };
    let Some(index) = sheet.get("heart_icon") else { return };

    for entity in &query {
        commands.entity(entity)
            .remove::<HeartIconPlaceholder>()
            .insert(ImageNode::from_atlas_image(
                sheet.texture.clone(),
                TextureAtlas { layout: sheet.layout.clone(), index },
            ));
    }
}
```

**Key points:**
- Widget functions using `ChildBuilder` can't access Bevy resources
- The marker is removed after population (one-time setup)
- Register the system in your plugin

## Animating Sprites

```rust
#[derive(Component)]
struct Animation {
    frames: Vec<String>,  // ["walk_1", "walk_2", "walk_3"]
    current: usize,
    timer: Timer,
}

fn animate_sprites(
    time: Res<Time>,
    game_sprites: Res<GameSprites>,
    mut query: Query<(&mut Animation, &mut Sprite)>,
) {
    let Some(sheet) = &game_sprites.ui_icons else { return };

    for (mut anim, mut sprite) in &mut query {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            anim.current = (anim.current + 1) % anim.frames.len();
            if let Some(index) = sheet.get(&anim.frames[anim.current]) {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = index;
                }
            }
        }
    }
}
```

## Changing Sprite at Runtime

```rust
fn update_health_icon(
    game_sprites: Res<GameSprites>,
    health: Res<PlayerHealth>,
    mut query: Query<&mut Sprite, With<HealthIcon>>,
) {
    let Some(icons) = &game_sprites.ui_icons else { return };

    for mut sprite in &mut query {
        let name = match health.percent() {
            p if p > 0.5 => "heart_full",
            p if p > 0.0 => "heart_half",
            _ => "heart_empty",
        };

        if let Some(index) = icons.get(name) {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = index;
            }
        }
    }
}
```

## Stateful Sprite Buttons

For toggles that change appearance based on state:

```rust
#[derive(Component)]
struct SpriteMenuItem {
    index: usize,
    unselected_slice: &'static str,
    selected_slice: &'static str,
}

fn update_sprite_menu_items(
    mut commands: Commands,
    menu_selection: Res<MenuSelection>,
    game_sprites: Res<GameSprites>,
    mut query: Query<(Entity, &SpriteMenuItem, Option<&mut ImageNode>)>,
) {
    let Some(ui_all) = &game_sprites.ui_all else { return };

    for (entity, sprite_item, image_node) in &mut query {
        let slice_name = if sprite_item.index == menu_selection.index {
            sprite_item.selected_slice
        } else {
            sprite_item.unselected_slice
        };

        let Some(index) = ui_all.get(slice_name) else { continue };

        match image_node {
            Some(mut node) => {
                if let Some(atlas) = &mut node.texture_atlas {
                    atlas.index = index;
                }
            }
            None => {
                commands.entity(entity).insert(ImageNode::from_atlas_image(
                    ui_all.texture.clone(),
                    TextureAtlas { layout: ui_all.layout.clone(), index },
                ));
            }
        }
    }
}
```

**Key difference from marker pattern:** The component persists for ongoing state changes.

## Scaling Sprites to Grid

```rust
const TILE_SIZE: f32 = 40.0;

// 16x16 sprite scaled to 40x40 tile grid
if let Some(sprite) = sheet.sprite_sized("floor", Vec2::splat(TILE_SIZE)) {
    commands.spawn((sprite, Transform::from_xyz(x, y, 0.0)));
}
```
