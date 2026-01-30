use bevy::prelude::*;

/// Navigation directions for menu/list traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub enum NavigationDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Game actions that can be triggered by input.
/// These map to the original keybinds from the terminal UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Event)]
pub enum GameAction {
    /// Navigation (Arrow keys)
    Navigate(NavigationDirection),

    /// Select/confirm (Enter)
    Select,

    /// Back/cancel (Backspace)
    Back,

    /// Tab switching (Tab)
    NextTab,

    /// Reverse tab switching (Shift+Tab)
    PrevTab,

    /// Mining action (Space)
    Mine,

    /// Open inventory modal (i)
    OpenInventory,

    /// Open profile modal (p)
    OpenProfile,

    /// Open keybinds modal (?)
    OpenKeybinds,

    /// Close current modal (Escape)
    CloseModal,

    /// Open monster compendium (b)
    OpenCompendium,

    /// Open skills modal (k)
    OpenSkills,
}

/// Tracks the currently-held navigation direction (if any).
/// Updated by InputPlugin each frame based on NavigationRepeatState.
#[derive(Resource, Default, Debug, Clone, PartialEq, Eq)]
pub struct HeldDirection(pub Option<NavigationDirection>);
