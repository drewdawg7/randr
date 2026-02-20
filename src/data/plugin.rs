use bevy::prelude::*;

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_data);
    }
}

fn load_data() {
    crate::item::data::init();
}
