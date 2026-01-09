use bevy::prelude::*;

/// Resource tracking the fight screen UI state.
#[derive(Resource, Default)]
pub struct FightScreenState {
    /// Current action menu selection (0=Attack, 1=Run)
    pub action_selection: usize,
    /// Post-combat menu selection (0=Fight Again, 1=Continue)
    pub post_combat_selection: usize,
    /// Frames since entering fight screen (used to skip input on first frame)
    pub frames_since_entry: u32,
}

impl FightScreenState {
    const ACTION_ITEMS: usize = 2; // Attack, Run
    const POST_COMBAT_ITEMS: usize = 2; // Fight Again, Continue

    pub fn action_up(&mut self) {
        if self.action_selection > 0 {
            self.action_selection -= 1;
        }
    }

    pub fn action_down(&mut self) {
        if self.action_selection + 1 < Self::ACTION_ITEMS {
            self.action_selection += 1;
        }
    }

    pub fn post_combat_up(&mut self) {
        if self.post_combat_selection > 0 {
            self.post_combat_selection -= 1;
        }
    }

    pub fn post_combat_down(&mut self) {
        if self.post_combat_selection + 1 < Self::POST_COMBAT_ITEMS {
            self.post_combat_selection += 1;
        }
    }

    pub fn reset(&mut self) {
        self.action_selection = 0;
        self.post_combat_selection = 0;
        self.frames_since_entry = 0;
    }
}

/// Combat origin tracking - determines where to return after combat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombatOrigin {
    Field,
    DungeonRoom,
    DungeonBoss,
}

/// Resource tracking where the current combat originated from.
#[derive(Resource, Default)]
pub struct CombatSource {
    pub origin: Option<CombatOrigin>,
}

impl CombatSource {
    pub fn new(origin: CombatOrigin) -> Self {
        Self {
            origin: Some(origin),
        }
    }

    pub fn clear(&mut self) {
        self.origin = None;
    }
}
