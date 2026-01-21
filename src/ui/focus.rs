//! Unified focus and selection system.
//!
//! Provides traits and generic systems for managing selection state
//! across different UI screens and modals.

use bevy::prelude::*;

use crate::ui::MenuIndex;

/// Trait for resources that manage selection state.
///
/// Implement this trait for any resource that tracks a selected index
/// in a list or menu. The trait provides default implementations for
/// clamped navigation (up/down) as well as wrapping variants.
///
/// # Example
///
/// ```ignore
/// #[derive(Resource, Default)]
/// pub struct MyListState {
///     pub selected: usize,
///     pub count: usize,
/// }
///
/// impl SelectionState for MyListState {
///     fn selected(&self) -> usize { self.selected }
///     fn count(&self) -> usize { self.count }
///     fn set_selected(&mut self, index: usize) { self.selected = index; }
/// }
/// ```
pub trait SelectionState {
    /// Returns the currently selected index.
    fn selected(&self) -> usize;

    /// Returns the total number of items.
    fn count(&self) -> usize;

    /// Sets the selected index. Implementations should handle bounds.
    fn set_selected(&mut self, index: usize);

    /// Navigate up (clamped at 0).
    fn up(&mut self) {
        if self.selected() > 0 {
            self.set_selected(self.selected() - 1);
        }
    }

    /// Navigate down (clamped at count - 1).
    fn down(&mut self) {
        if self.selected() + 1 < self.count() {
            self.set_selected(self.selected() + 1);
        }
    }

    /// Navigate up with wrapping (wraps to last item at top).
    fn up_wrap(&mut self) {
        let new = if self.selected() == 0 {
            self.count().saturating_sub(1)
        } else {
            self.selected() - 1
        };
        self.set_selected(new);
    }

    /// Navigate down with wrapping (wraps to first item at bottom).
    fn down_wrap(&mut self) {
        if self.count() == 0 {
            return;
        }
        self.set_selected((self.selected() + 1) % self.count());
    }

    /// Reset selection to the first item.
    fn reset(&mut self) {
        self.set_selected(0);
    }

    /// Clamp selection to valid bounds after count changes.
    fn clamp_to_bounds(&mut self) {
        let count = self.count();
        if count == 0 {
            self.set_selected(0);
        } else if self.selected() >= count {
            self.set_selected(count - 1);
        }
    }
}

/// Creates a system that updates text color for menu items based on selection state.
///
/// Only runs when the state resource has changed.
///
/// # Type Parameters
/// * `M` - Marker component to filter which menu items to update
/// * `S` - Selection state resource
///
/// # Example
///
/// ```ignore
/// app.add_systems(Update, selection_text_color_system::<MyMenuItem, MyState>(my_color_fn));
/// ```
pub fn selection_text_color_system<M: Component, S: SelectionState + Resource>(
    color_fn: fn(bool) -> Color,
) -> impl FnMut(Res<S>, Query<(&MenuIndex, &mut TextColor), With<M>>) {
    move |state: Res<S>, mut query: Query<(&MenuIndex, &mut TextColor), With<M>>| {
        if !state.is_changed() {
            return;
        }
        for (index, mut color) in query.iter_mut() {
            let is_selected = index.0 == state.selected();
            *color = TextColor(color_fn(is_selected));
        }
    }
}

/// Creates a system that updates background color for selectable items.
///
/// Only runs when the state resource has changed.
///
/// # Example
///
/// ```ignore
/// app.add_systems(Update, selection_background_system::<MyItem, MyState>(SELECTED, NORMAL));
/// ```
pub fn selection_background_system<M: Component, S: SelectionState + Resource>(
    selected_color: Color,
    normal_color: Color,
) -> impl FnMut(Res<S>, Query<(&MenuIndex, &mut BackgroundColor), With<M>>) {
    move |state: Res<S>, mut query: Query<(&MenuIndex, &mut BackgroundColor), With<M>>| {
        if !state.is_changed() {
            return;
        }
        for (index, mut bg) in query.iter_mut() {
            let is_selected = index.0 == state.selected();
            *bg = if is_selected {
                selected_color.into()
            } else {
                normal_color.into()
            };
        }
    }
}
