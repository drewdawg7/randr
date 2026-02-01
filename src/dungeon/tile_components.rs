use bevy::prelude::*;
use bevy::reflect::TypePath;

macro_rules! tile_property {
    ($name:ident) => {
        #[derive(Component, Reflect, Default)]
        #[reflect(Component, Default, type_path = false)]
        #[allow(non_camel_case_types)]
        pub struct $name(pub bool);

        impl TypePath for $name {
            fn type_path() -> &'static str {
                stringify!($name)
            }

            fn short_type_path() -> &'static str {
                stringify!($name)
            }
        }
    };
}

tile_property!(is_solid);
tile_property!(can_have_entity);
tile_property!(can_spawn_player);
tile_property!(is_door);
