use bevy::prelude::*;

use super::state::TownTab;

#[derive(Component)]
pub struct TownUiRoot;

#[derive(Component)]
pub(super) struct TabHeaderItem {
    pub tab: TownTab,
}

#[derive(Component)]
pub struct ContentArea;

#[derive(Component)]
pub struct TabContent;
