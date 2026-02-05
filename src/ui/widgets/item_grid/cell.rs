use bevy::prelude::*;

use super::CELL_SIZE;

#[derive(Component)]
pub struct GridContainer;

#[derive(Component)]
pub struct GridCell {
    pub index: usize,
}

#[derive(Bundle)]
pub struct GridCellBundle {
    pub cell: GridCell,
    pub node: Node,
}

impl GridCellBundle {
    pub fn new(index: usize) -> Self {
        Self {
            cell: GridCell { index },
            node: Node {
                width: Val::Px(CELL_SIZE),
                height: Val::Px(CELL_SIZE),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        }
    }
}
