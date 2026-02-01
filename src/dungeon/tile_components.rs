use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[allow(non_camel_case_types)]
pub struct is_solid;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[allow(non_camel_case_types)]
pub struct can_have_entity;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[allow(non_camel_case_types)]
pub struct can_spawn_player;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[allow(non_camel_case_types)]
pub struct is_door;
