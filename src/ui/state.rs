//! UI-specific state management.
//!
//! This module contains UI state that is separate from game logic state.
//! Keeping UI state isolated makes it easier to swap out the UI framework
//! or to reason about what state affects the presentation layer vs. the game.

use crate::toast::ToastQueue;
use crate::ui::components::player::inventory_modal::InventoryModal;
use crate::ui::screen::{ScreenLifecycle, ScreenMetadata};
use crate::ui::Id;

/// Types of modal overlays that can be displayed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModalType {
    #[default]
    None,
    Keybinds,
    Inventory,
    Profile,
}

/// UI-specific state, separate from game logic state.
///
/// This struct contains all state related to the user interface,
/// such as which screen is active, modal states, and toast notifications.
/// Keeping this separate from game state (player, town, combat) makes
/// it easier to reason about UI behavior and potentially swap UI frameworks.
pub struct UIState {
    /// The currently active screen.
    pub current_screen: Id,

    /// Tracks screen transitions for lifecycle management.
    pub lifecycle: ScreenLifecycle,

    /// The currently active modal overlay, if any.
    pub active_modal: ModalType,

    /// The inventory modal state (kept separately as it has its own state).
    pub inventory_modal: InventoryModal,

    /// Whether to show detailed item information in lists.
    pub show_item_details: bool,

    /// Queue of toast notifications to display.
    pub toasts: ToastQueue,
}

impl UIState {
    /// Create a new UIState with default values.
    pub fn new() -> Self {
        Self {
            current_screen: Id::Menu,
            lifecycle: ScreenLifecycle::new(Id::Menu),
            active_modal: ModalType::None,
            inventory_modal: InventoryModal::new(),
            show_item_details: false,
            toasts: ToastQueue::default(),
        }
    }

    /// Navigate to a new screen.
    pub fn go_to_screen(&mut self, screen: Id) {
        self.current_screen = screen;
    }

    /// Check if the current screen just entered (for reset logic).
    pub fn just_entered_screen(&self) -> bool {
        self.lifecycle.just_entered()
    }

    /// Check if we just came from a specific screen.
    pub fn came_from(&self, screen: Id) -> bool {
        self.lifecycle.came_from(screen)
    }

    /// Get metadata about the current screen.
    pub fn current_screen_metadata(&self) -> ScreenMetadata {
        ScreenMetadata::for_screen(self.current_screen)
    }

    /// Open a modal.
    pub fn open_modal(&mut self, modal: ModalType) {
        self.active_modal = modal;
        if modal == ModalType::Inventory {
            self.inventory_modal.reset();
        }
    }

    /// Close the current modal.
    pub fn close_modal(&mut self) {
        self.active_modal = ModalType::None;
    }

    /// Toggle a modal (open if closed, close if open).
    pub fn toggle_modal(&mut self, modal: ModalType) {
        if self.active_modal == modal {
            self.close_modal();
        } else {
            self.open_modal(modal);
        }
    }

    /// Check if any modal is open.
    pub fn has_modal_open(&self) -> bool {
        self.active_modal != ModalType::None
    }

    /// Check if a specific modal is open.
    pub fn is_modal_open(&self, modal: ModalType) -> bool {
        self.active_modal == modal
    }

    /// Add an error toast.
    pub fn toast_error(&mut self, message: impl Into<String>) {
        self.toasts.error(message);
    }

    /// Add a success toast.
    pub fn toast_success(&mut self, message: impl Into<String>) {
        self.toasts.success(message);
    }

    /// Add an info toast.
    pub fn toast_info(&mut self, message: impl Into<String>) {
        self.toasts.info(message);
    }

    /// Update lifecycle for the current frame. Call once per frame.
    pub fn update_lifecycle(&mut self) {
        self.lifecycle.update(self.current_screen);
    }

    /// Clean up expired toasts. Call once per frame.
    pub fn cleanup_toasts(&mut self) {
        self.toasts.cleanup();
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Full UIState tests require game_state() to be initialized
    // (due to InventoryModal). These tests focus on isolated functionality.

    #[test]
    fn modal_type_default() {
        assert_eq!(ModalType::default(), ModalType::None);
    }

    #[test]
    fn modal_type_equality() {
        assert_eq!(ModalType::Inventory, ModalType::Inventory);
        assert_ne!(ModalType::Inventory, ModalType::Keybinds);
        assert_ne!(ModalType::None, ModalType::Inventory);
    }

    // Full UIState integration tests would go in tests/ directory
    // where game_state() can be properly initialized
}
