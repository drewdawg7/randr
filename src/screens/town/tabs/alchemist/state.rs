//! Alchemist tab state resources.

use bevy::prelude::*;

use crate::item::recipe::RecipeId;

use super::super::super::shared::SelectionState;

/// The kind of mode for the Alchemist tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlchemistModeKind {
    #[default]
    Menu,
    Brew,
}

/// Alchemist mode resource.
#[derive(Resource)]
pub struct AlchemistMode {
    pub mode: AlchemistModeKind,
    pub available_recipes: Vec<RecipeId>,
}

impl Default for AlchemistMode {
    fn default() -> Self {
        Self {
            mode: AlchemistModeKind::Menu,
            available_recipes: RecipeId::all_alchemy_recipes(),
        }
    }
}

/// Alchemist selections resource.
#[derive(Resource)]
pub struct AlchemistSelections {
    pub menu: usize,
    pub recipe: SelectionState,
}

impl Default for AlchemistSelections {
    fn default() -> Self {
        let recipe_count = RecipeId::all_alchemy_recipes().len();
        Self {
            menu: 0,
            recipe: SelectionState {
                selected: 0,
                count: recipe_count,
                scroll_offset: 0,
                visible_count: 10,
            },
        }
    }
}
