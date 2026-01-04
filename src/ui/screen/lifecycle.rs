//! Screen lifecycle management.
//!
//! This module provides utilities for detecting screen transitions and
//! managing screen state across the application. Since screens are stored
//! as boxed trait objects in tuirealm's Application, we use a transition
//! detection pattern rather than direct method calls.
//!
//! # Usage
//!
//! Screens can check for transitions in their `view()` or `on()` methods:
//!
//! ```ignore
//! fn view(&mut self, frame: &mut Frame, area: Rect) {
//!     let gs = game_state();
//!     if gs.screen_lifecycle().just_entered() {
//!         self.reset_state();
//!     }
//!     // ... rest of rendering
//! }
//! ```

use super::common::Id;

/// Tracks screen transitions and provides lifecycle utilities.
#[derive(Debug, Clone)]
pub struct ScreenLifecycle {
    /// The previous screen before the current one.
    previous_screen: Option<Id>,
    /// The current screen.
    current_screen: Id,
    /// Whether we just transitioned to the current screen.
    just_entered: bool,
    /// Counter incremented on each transition (for change detection).
    transition_count: u64,
}

impl ScreenLifecycle {
    /// Create a new lifecycle tracker starting at the given screen.
    pub fn new(initial_screen: Id) -> Self {
        Self {
            previous_screen: None,
            current_screen: initial_screen,
            just_entered: true, // Initial screen counts as "just entered"
            transition_count: 0,
        }
    }

    /// Update the lifecycle for a new frame. Call this once per frame
    /// BEFORE rendering, passing the current screen from GameState.
    ///
    /// Returns `true` if a transition occurred.
    pub fn update(&mut self, new_screen: Id) -> bool {
        if new_screen != self.current_screen {
            self.previous_screen = Some(self.current_screen);
            self.current_screen = new_screen;
            self.just_entered = true;
            self.transition_count += 1;
            true
        } else {
            self.just_entered = false;
            false
        }
    }

    /// Check if we just entered the current screen this frame.
    ///
    /// This is `true` only on the first frame after a transition.
    /// Use this to reset screen state when entering.
    pub fn just_entered(&self) -> bool {
        self.just_entered
    }

    /// Get the previous screen we came from, if any.
    pub fn previous_screen(&self) -> Option<Id> {
        self.previous_screen
    }

    /// Check if we just came from a specific screen.
    pub fn came_from(&self, screen: Id) -> bool {
        self.just_entered && self.previous_screen == Some(screen)
    }

    /// Get the current screen.
    pub fn current_screen(&self) -> Id {
        self.current_screen
    }

    /// Get the total number of transitions that have occurred.
    pub fn transition_count(&self) -> u64 {
        self.transition_count
    }

    /// Clear the "just entered" flag.
    ///
    /// Call this after handling the transition to prevent re-triggering
    /// reset logic on subsequent checks within the same frame.
    pub fn acknowledge_entry(&mut self) {
        self.just_entered = false;
    }
}

impl Default for ScreenLifecycle {
    fn default() -> Self {
        Self::new(Id::Menu)
    }
}

/// Metadata about a screen for validation and documentation.
#[derive(Debug, Clone)]
pub struct ScreenMetadata {
    /// The screen identifier.
    pub id: Id,
    /// Human-readable name.
    pub name: &'static str,
    /// Whether this screen requires active combat to be valid.
    pub requires_combat: bool,
    /// Whether this screen requires an active dungeon.
    pub requires_dungeon: bool,
    /// The "parent" screen to return to (if any).
    pub parent: Option<Id>,
}

impl ScreenMetadata {
    /// Get metadata for a screen.
    pub fn for_screen(id: Id) -> Self {
        match id {
            Id::Menu => Self {
                id,
                name: "Main Menu",
                requires_combat: false,
                requires_dungeon: false,
                parent: None,
            },
            Id::Town => Self {
                id,
                name: "Town",
                requires_combat: false,
                requires_dungeon: false,
                parent: Some(Id::Menu),
            },
            Id::Fight => Self {
                id,
                name: "Combat",
                requires_combat: true,
                requires_dungeon: false,
                parent: None, // Dynamic based on CombatSource
            },
            Id::Profile => Self {
                id,
                name: "Player Profile",
                requires_combat: false,
                requires_dungeon: false,
                parent: Some(Id::Town),
            },
            Id::Mine => Self {
                id,
                name: "Mine",
                requires_combat: false,
                requires_dungeon: false,
                parent: Some(Id::Town),
            },
            Id::Dungeon => Self {
                id,
                name: "Dungeon",
                requires_combat: false,
                requires_dungeon: true,
                parent: Some(Id::Town),
            },
            Id::Quit => Self {
                id,
                name: "Quit",
                requires_combat: false,
                requires_dungeon: false,
                parent: None,
            },
        }
    }

    /// Get all valid screens that can be navigated to from this screen.
    pub fn valid_destinations(&self) -> Vec<Id> {
        match self.id {
            Id::Menu => vec![Id::Town, Id::Quit],
            Id::Town => vec![Id::Menu, Id::Fight, Id::Mine, Id::Dungeon, Id::Profile],
            Id::Fight => vec![Id::Town, Id::Dungeon], // Based on CombatSource
            Id::Profile => vec![Id::Town],
            Id::Mine => vec![Id::Town],
            Id::Dungeon => vec![Id::Town, Id::Fight],
            Id::Quit => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_tracks_transitions() {
        let mut lifecycle = ScreenLifecycle::new(Id::Menu);
        assert!(lifecycle.just_entered());
        assert_eq!(lifecycle.current_screen(), Id::Menu);
        assert_eq!(lifecycle.previous_screen(), None);

        // Same screen - no transition
        lifecycle.update(Id::Menu);
        assert!(!lifecycle.just_entered());

        // Transition to Town
        lifecycle.update(Id::Town);
        assert!(lifecycle.just_entered());
        assert_eq!(lifecycle.current_screen(), Id::Town);
        assert_eq!(lifecycle.previous_screen(), Some(Id::Menu));
        assert!(lifecycle.came_from(Id::Menu));
        assert!(!lifecycle.came_from(Id::Fight));

        // Next frame - no longer "just entered"
        lifecycle.update(Id::Town);
        assert!(!lifecycle.just_entered());
        assert!(!lifecycle.came_from(Id::Menu));
    }

    #[test]
    fn screen_metadata() {
        let fight_meta = ScreenMetadata::for_screen(Id::Fight);
        assert!(fight_meta.requires_combat);
        assert!(!fight_meta.requires_dungeon);

        let dungeon_meta = ScreenMetadata::for_screen(Id::Dungeon);
        assert!(!dungeon_meta.requires_combat);
        assert!(dungeon_meta.requires_dungeon);
    }
}
