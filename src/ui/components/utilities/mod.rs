//! UI utility functions organized by purpose.
//!
//! This module provides cohesive groups of utilities:
//! - `styling`: Pure functions for creating styled Spans (selection prefixes, item display)
//! - `headers`: Location header creation and rendering
//! - `navigation`: List navigation with wrapping
//! - `queries`: Game state accessors for UI (isolated game_state access)
//!
//! Icon constants are in `crate::ui::theme::icons` but re-exported here for convenience.

mod styling;
mod headers;
mod navigation;
mod queries;

// Re-export all public items for backward compatibility
pub use styling::{selection_prefix, lock_prefix, equip_prefix, item_display};
pub use headers::{blacksmith_header, store_header, render_location_header};
pub use navigation::{list_move_up, list_move_down};
pub use queries::{collect_player_items, collect_player_equipment};

// Re-export icons from theme for backward compatibility
pub use crate::ui::theme::icons::*;
