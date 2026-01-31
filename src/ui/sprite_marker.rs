//! Generic sprite marker trait and population system.
//!
//! This module provides a trait-based approach to sprite population,
//! eliminating repeated marker-to-sprite conversion code across the codebase.

use bevy::ecs::system::{StaticSystemParam, SystemParam};
use bevy::prelude::*;
use tracing::instrument;

use super::animation::{AnimationConfig, SpriteAnimation};

/// Data needed to populate a sprite entity.
pub struct SpriteData {
    /// Texture handle for the sprite
    pub texture: Handle<Image>,
    /// Texture atlas layout handle
    pub layout: Handle<TextureAtlasLayout>,
    /// Animation configuration
    pub animation: AnimationConfig,
    /// Whether to flip the sprite horizontally
    pub flip_x: bool,
}

/// Trait for marker components that request sprite population.
///
/// Implementors define how to resolve sprite data from world resources.
/// The marker is removed after population.
///
/// # Example
///
/// ```ignore
/// #[derive(Component)]
/// pub struct MySprite;
///
/// impl SpriteMarker for MySprite {
///     type Resources = Res<'static, MySpriteSheet>;
///
///     fn resolve(&self, sheet: &Res<MySpriteSheet>) -> Option<SpriteData> {
///         Some(SpriteData {
///             texture: sheet.texture.clone(),
///             layout: sheet.layout.clone(),
///             animation: sheet.animation.clone(),
///             flip_x: false,
///         })
///     }
/// }
/// ```
pub trait SpriteMarker: Component + Sized {
    /// The resource type(s) needed for sprite lookup.
    type Resources: SystemParam;

    /// Resolve the sprite data for this marker given the resources.
    ///
    /// Returns `None` if the sprite sheet isn't loaded yet or the marker
    /// refers to an unknown sprite.
    fn resolve(
        &self,
        resources: &<Self::Resources as SystemParam>::Item<'_, '_>,
    ) -> Option<SpriteData>;
}

/// Generic system to populate sprite markers.
///
/// This system detects entities with newly added marker components,
/// resolves their sprite data using the trait implementation,
/// removes the marker, and inserts the sprite components.
#[instrument(level = "debug", skip_all, fields(marker_count = query.iter().count()))]
pub fn populate_sprite_markers<M: SpriteMarker>(
    mut commands: Commands,
    query: Query<(Entity, &M), Added<M>>,
    resources: StaticSystemParam<M::Resources>,
) {
    for (entity, marker) in &query {
        if let Some(data) = marker.resolve(&resources) {
            let mut image = ImageNode::from_atlas_image(
                data.texture,
                TextureAtlas {
                    layout: data.layout,
                    index: data.animation.first_frame,
                },
            );
            image.flip_x = data.flip_x;

            commands
                .entity(entity)
                .remove::<M>()
                .insert((image, SpriteAnimation::new(&data.animation)));
        }
    }
}

/// Extension trait for registering sprite markers with the app.
pub trait SpriteMarkerAppExt {
    /// Register a sprite marker type with the app.
    ///
    /// This adds the generic populate system for the marker type.
    fn register_sprite_marker<M: SpriteMarker>(&mut self) -> &mut Self;
}

impl SpriteMarkerAppExt for App {
    fn register_sprite_marker<M: SpriteMarker>(&mut self) -> &mut Self {
        self.add_systems(
            Update,
            populate_sprite_markers::<M>.run_if(any_with_component::<M>),
        );
        self
    }
}
