use bevy::prelude::*;

use crate::screens::town::shared::SelectionState;

use super::constants::MENU_OPTIONS;

/// The different modes/screens in the Blacksmith tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlacksmithModeKind {
    #[default]
    Menu,
    Upgrade,
    Quality,
    Smelt,
    Forge,
}

/// Blacksmith mode - tracks navigation state within the tab.
#[derive(Resource, Default)]
pub struct BlacksmithMode {
    pub mode: BlacksmithModeKind,
}

/// Blacksmith selections - tracks cursor positions in each mode.
#[derive(Resource)]
pub struct BlacksmithSelections {
    pub menu: SelectionState,
    pub upgrade: SelectionState,
    pub quality: SelectionState,
    pub smelt: SelectionState,
    pub forge: SelectionState,
}

impl Default for BlacksmithSelections {
    fn default() -> Self {
        Self {
            menu: SelectionState {
                selected: 0,
                count: MENU_OPTIONS.len(),
                scroll_offset: 0,
                visible_count: 10,
            },
            upgrade: SelectionState::new(0),
            quality: SelectionState::new(0),
            smelt: SelectionState::new(0),
            forge: SelectionState::new(0),
        }
    }
}
