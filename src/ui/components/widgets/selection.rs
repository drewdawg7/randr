//! Selection abstractions for UI navigation.
//!
//! This module provides reusable selection patterns that abstract common
//! navigation patterns across the UI. Use these instead of manually managing
//! selection indices to avoid bugs and ensure consistent behavior.
//!
//! # Available Types
//!
//! - [`ListSelection`] - Vertical list navigation with wrapping
//! - [`BinaryToggle`] - Toggle between exactly two options
//! - [`BoundedSelection`] - Bounded numeric selection (no wrapping)
//! - [`GridSelection`] - 2D grid selection (row, column)
//! - [`DirectionalSelection`] - Compass-style selection with availability constraints

use ratatui::widgets::ListState;
use std::collections::HashSet;

/// Direction for navigation commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NavDirection {
    Up,
    Down,
    Left,
    Right,
}

impl NavDirection {
    /// Returns the opposite direction.
    pub fn opposite(self) -> Self {
        match self {
            NavDirection::Up => NavDirection::Down,
            NavDirection::Down => NavDirection::Up,
            NavDirection::Left => NavDirection::Right,
            NavDirection::Right => NavDirection::Left,
        }
    }
}

/// Vertical list selection with wrapping navigation.
///
/// Wraps around when navigating past the top or bottom of the list.
///
/// # Example
/// ```
/// use game::ui::components::widgets::selection::ListSelection;
/// let mut sel = ListSelection::new(5); // 5 items
/// sel.move_down(); // 0 -> 1
/// sel.move_up();   // 1 -> 0
/// sel.move_up();   // 0 -> 4 (wraps)
/// ```
#[derive(Debug, Clone)]
pub struct ListSelection {
    count: usize,
    selected: usize,
    list_state: ListState,
}

impl ListSelection {
    /// Create a new selection for a list with the given item count.
    /// Starts with item 0 selected.
    pub fn new(count: usize) -> Self {
        let mut list_state = ListState::default();
        if count > 0 {
            list_state.select(Some(0));
        }
        Self {
            count,
            selected: 0,
            list_state,
        }
    }

    /// Update the item count, clamping selection if needed.
    pub fn set_count(&mut self, count: usize) {
        self.count = count;
        if count == 0 {
            self.selected = 0;
            self.list_state.select(None);
        } else if self.selected >= count {
            self.selected = count - 1;
            self.list_state.select(Some(self.selected));
        }
    }

    /// Get the current item count.
    pub fn count(&self) -> usize {
        self.count
    }

    /// Move selection up with wrapping.
    pub fn move_up(&mut self) {
        if self.count == 0 {
            return;
        }
        self.selected = if self.selected == 0 {
            self.count - 1
        } else {
            self.selected - 1
        };
        self.list_state.select(Some(self.selected));
    }

    /// Move selection down with wrapping.
    pub fn move_down(&mut self) {
        if self.count == 0 {
            return;
        }
        self.selected = if self.selected >= self.count - 1 {
            0
        } else {
            self.selected + 1
        };
        self.list_state.select(Some(self.selected));
    }

    /// Handle a navigation direction (only Up/Down are meaningful).
    pub fn navigate(&mut self, dir: NavDirection) {
        match dir {
            NavDirection::Up => self.move_up(),
            NavDirection::Down => self.move_down(),
            _ => {}
        }
    }

    /// Get the currently selected index.
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// Set the selected index directly.
    pub fn select(&mut self, idx: usize) {
        if self.count > 0 && idx < self.count {
            self.selected = idx;
            self.list_state.select(Some(idx));
        }
    }

    /// Reset selection to the first item.
    pub fn reset(&mut self) {
        self.selected = 0;
        if self.count > 0 {
            self.list_state.select(Some(0));
        } else {
            self.list_state.select(None);
        }
    }

    /// Get a mutable reference to the underlying ListState for rendering.
    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.list_state
    }

    /// Get the underlying ListState for rendering.
    pub fn list_state(&self) -> &ListState {
        &self.list_state
    }
}

/// Binary toggle between exactly two options.
///
/// # Example
/// ```
/// use game::ui::components::widgets::selection::BinaryToggle;
/// let mut toggle = BinaryToggle::new("Attack", "Run");
/// assert_eq!(toggle.selected(), &"Attack");
/// toggle.toggle();
/// assert_eq!(toggle.selected(), &"Run");
/// ```
#[derive(Debug, Clone)]
pub struct BinaryToggle<T> {
    first: T,
    second: T,
    is_first_selected: bool,
}

impl<T> BinaryToggle<T> {
    /// Create a new toggle with two options. First option is selected by default.
    pub fn new(first: T, second: T) -> Self {
        Self {
            first,
            second,
            is_first_selected: true,
        }
    }

    /// Toggle to the other option.
    pub fn toggle(&mut self) {
        self.is_first_selected = !self.is_first_selected;
    }

    /// Handle any navigation direction (all toggle).
    pub fn navigate(&mut self, _dir: NavDirection) {
        self.toggle();
    }

    /// Get a reference to the currently selected option.
    pub fn selected(&self) -> &T {
        if self.is_first_selected {
            &self.first
        } else {
            &self.second
        }
    }

    /// Check if the first option is selected.
    pub fn is_first(&self) -> bool {
        self.is_first_selected
    }

    /// Check if the second option is selected.
    pub fn is_second(&self) -> bool {
        !self.is_first_selected
    }

    /// Select the first option.
    pub fn select_first(&mut self) {
        self.is_first_selected = true;
    }

    /// Select the second option.
    pub fn select_second(&mut self) {
        self.is_first_selected = false;
    }

    /// Reset to the first option (default).
    pub fn reset(&mut self) {
        self.is_first_selected = true;
    }

    /// Get references to both options.
    pub fn options(&self) -> (&T, &T) {
        (&self.first, &self.second)
    }
}

impl<T: PartialEq> BinaryToggle<T> {
    /// Check if a given value matches the current selection.
    pub fn is_selected(&self, value: &T) -> bool {
        self.selected() == value
    }
}

/// Bounded selection that doesn't wrap.
///
/// Useful for selecting between a fixed number of options where
/// you don't want wrap-around behavior.
///
/// # Example
/// ```
/// use game::ui::components::widgets::selection::BoundedSelection;
/// let mut sel = BoundedSelection::new(3); // 0, 1, 2
/// sel.move_right(); // 0 -> 1
/// sel.move_right(); // 1 -> 2
/// sel.move_right(); // 2 -> 2 (stays at max)
/// ```
#[derive(Debug, Clone)]
pub struct BoundedSelection {
    current: usize,
    max: usize, // Inclusive maximum
}

impl BoundedSelection {
    /// Create a new bounded selection with the given maximum (inclusive).
    /// Starts at 0.
    pub fn new(max: usize) -> Self {
        Self { current: 0, max }
    }

    /// Create a new bounded selection with explicit bounds.
    pub fn with_bounds(min: usize, max: usize, start: usize) -> Self {
        let start = start.clamp(min, max);
        Self { current: start - min, max: max - min }
    }

    /// Move selection left (decrement).
    pub fn move_left(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    /// Move selection right (increment).
    pub fn move_right(&mut self) {
        if self.current < self.max {
            self.current += 1;
        }
    }

    /// Move selection up (decrement).
    pub fn move_up(&mut self) {
        self.move_left();
    }

    /// Move selection down (increment).
    pub fn move_down(&mut self) {
        self.move_right();
    }

    /// Handle a navigation direction.
    pub fn navigate(&mut self, dir: NavDirection) {
        match dir {
            NavDirection::Up => self.move_up(),
            NavDirection::Down => self.move_down(),
            NavDirection::Left => self.move_left(),
            NavDirection::Right => self.move_right(),
        }
    }

    /// Get the current selection value.
    pub fn selected(&self) -> usize {
        self.current
    }

    /// Set the selection directly, clamping to bounds.
    pub fn select(&mut self, value: usize) {
        self.current = value.min(self.max);
    }

    /// Check if at the minimum value.
    pub fn at_min(&self) -> bool {
        self.current == 0
    }

    /// Check if at the maximum value.
    pub fn at_max(&self) -> bool {
        self.current == self.max
    }

    /// Reset to 0.
    pub fn reset(&mut self) {
        self.current = 0;
    }
}

/// 2D grid selection with bounded movement.
///
/// Useful for layouts like the mine screen where you have
/// rows and columns of options.
///
/// # Example
/// ```
/// use game::ui::components::widgets::selection::GridSelection;
/// let mut grid = GridSelection::new(2, 3); // 2 rows, 3 columns
/// grid.move_right(); // (0,0) -> (0,1)
/// grid.move_down();  // (0,1) -> (1,1)
/// ```
#[derive(Debug, Clone)]
pub struct GridSelection {
    row: usize,
    col: usize,
    max_row: usize,   // Inclusive
    max_col: usize,   // Inclusive
}

impl GridSelection {
    /// Create a new grid selection with the given dimensions.
    /// Starts at (0, 0).
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            row: 0,
            col: 0,
            max_row: rows.saturating_sub(1),
            max_col: cols.saturating_sub(1),
        }
    }

    /// Move selection up.
    pub fn move_up(&mut self) {
        if self.row > 0 {
            self.row -= 1;
        }
    }

    /// Move selection down.
    pub fn move_down(&mut self) {
        if self.row < self.max_row {
            self.row += 1;
        }
    }

    /// Move selection left.
    pub fn move_left(&mut self) {
        if self.col > 0 {
            self.col -= 1;
        }
    }

    /// Move selection right.
    pub fn move_right(&mut self) {
        if self.col < self.max_col {
            self.col += 1;
        }
    }

    /// Handle a navigation direction.
    pub fn navigate(&mut self, dir: NavDirection) {
        match dir {
            NavDirection::Up => self.move_up(),
            NavDirection::Down => self.move_down(),
            NavDirection::Left => self.move_left(),
            NavDirection::Right => self.move_right(),
        }
    }

    /// Get the current position as (row, column).
    pub fn position(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    /// Get the current row.
    pub fn row(&self) -> usize {
        self.row
    }

    /// Get the current column.
    pub fn col(&self) -> usize {
        self.col
    }

    /// Set position directly, clamping to bounds.
    pub fn select(&mut self, row: usize, col: usize) {
        self.row = row.min(self.max_row);
        self.col = col.min(self.max_col);
    }

    /// Reset to (0, 0).
    pub fn reset(&mut self) {
        self.row = 0;
        self.col = 0;
    }
}

/// Compass-style directional selection with availability constraints.
///
/// Allows selection of cardinal directions (N/S/E/W) plus a center position,
/// with the ability to mark directions as available or unavailable.
///
/// # Example
/// ```
/// use game::ui::components::widgets::selection::{DirectionalSelection, NavDirection};
/// let mut compass = DirectionalSelection::new();
/// compass.set_available(NavDirection::Up, true);   // North available
/// compass.set_available(NavDirection::Right, true); // East available
/// compass.navigate(NavDirection::Up); // Center -> North
/// compass.navigate(NavDirection::Right); // North -> East
/// ```
#[derive(Debug, Clone)]
pub struct DirectionalSelection {
    position: Option<NavDirection>, // None = center
    available: HashSet<NavDirection>,
}

impl DirectionalSelection {
    /// Create a new directional selection starting at center.
    pub fn new() -> Self {
        Self {
            position: None,
            available: HashSet::new(),
        }
    }

    /// Create with specific available directions.
    pub fn with_available(available: impl IntoIterator<Item = NavDirection>) -> Self {
        Self {
            position: None,
            available: available.into_iter().collect(),
        }
    }

    /// Set whether a direction is available.
    pub fn set_available(&mut self, dir: NavDirection, is_available: bool) {
        if is_available {
            self.available.insert(dir);
        } else {
            self.available.remove(&dir);
            // If we were on this direction, return to center
            if self.position == Some(dir) {
                self.position = None;
            }
        }
    }

    /// Update all available directions at once.
    pub fn set_all_available(&mut self, available: impl IntoIterator<Item = NavDirection>) {
        self.available = available.into_iter().collect();
        // If current position is no longer available, return to center
        if let Some(pos) = self.position {
            if !self.available.contains(&pos) {
                self.position = None;
            }
        }
    }

    /// Check if a direction is available.
    pub fn is_available(&self, dir: NavDirection) -> bool {
        self.available.contains(&dir)
    }

    /// Navigate in a direction.
    pub fn navigate(&mut self, dir: NavDirection) {
        match self.position {
            None => {
                // From center, try to move to the requested direction
                if self.available.contains(&dir) {
                    self.position = Some(dir);
                }
            }
            Some(current) => {
                // From a direction position:
                // - Opposite direction returns to center
                // - Same axis movement tries to go to the requested direction
                // - Perpendicular movement tries to go to the requested direction
                if dir == current.opposite() {
                    // Going back towards center
                    self.position = None;
                } else if dir != current && self.available.contains(&dir) {
                    // Moving to a different available direction
                    self.position = Some(dir);
                }
                // Moving in the same direction we're already on does nothing
            }
        }
    }

    /// Get the current position.
    /// Returns None for center, Some(direction) for cardinal positions.
    pub fn position(&self) -> Option<NavDirection> {
        self.position
    }

    /// Check if at center position.
    pub fn is_center(&self) -> bool {
        self.position.is_none()
    }

    /// Check if at a specific direction.
    pub fn is_at(&self, dir: NavDirection) -> bool {
        self.position == Some(dir)
    }

    /// Reset to center position.
    pub fn reset(&mut self) {
        self.position = None;
    }

    /// Get all available directions.
    pub fn available_directions(&self) -> &HashSet<NavDirection> {
        &self.available
    }
}

impl Default for DirectionalSelection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_selection_wrapping() {
        let mut sel = ListSelection::new(3);
        assert_eq!(sel.selected(), 0);

        sel.move_up(); // 0 -> 2 (wrap)
        assert_eq!(sel.selected(), 2);

        sel.move_down(); // 2 -> 0 (wrap)
        assert_eq!(sel.selected(), 0);

        sel.move_down(); // 0 -> 1
        assert_eq!(sel.selected(), 1);
    }

    #[test]
    fn binary_toggle() {
        let mut toggle = BinaryToggle::new("A", "B");
        assert!(toggle.is_first());
        assert_eq!(toggle.selected(), &"A");

        toggle.toggle();
        assert!(toggle.is_second());
        assert_eq!(toggle.selected(), &"B");

        toggle.reset();
        assert!(toggle.is_first());
    }

    #[test]
    fn bounded_selection_no_wrap() {
        let mut sel = BoundedSelection::new(2);
        assert_eq!(sel.selected(), 0);

        sel.move_left(); // stays at 0
        assert_eq!(sel.selected(), 0);

        sel.move_right(); // 0 -> 1
        sel.move_right(); // 1 -> 2
        sel.move_right(); // stays at 2
        assert_eq!(sel.selected(), 2);
    }

    #[test]
    fn grid_selection() {
        let mut grid = GridSelection::new(2, 3);
        assert_eq!(grid.position(), (0, 0));

        grid.move_right();
        assert_eq!(grid.position(), (0, 1));

        grid.move_down();
        assert_eq!(grid.position(), (1, 1));

        grid.move_up();
        grid.move_up(); // stays at row 0
        assert_eq!(grid.position(), (0, 1));
    }

    #[test]
    fn directional_selection() {
        let mut compass = DirectionalSelection::new();
        compass.set_available(NavDirection::Up, true);
        compass.set_available(NavDirection::Right, true);

        assert!(compass.is_center());

        compass.navigate(NavDirection::Up);
        assert!(compass.is_at(NavDirection::Up));

        compass.navigate(NavDirection::Right);
        assert!(compass.is_at(NavDirection::Right));

        compass.navigate(NavDirection::Left); // Opposite, goes to center
        assert!(compass.is_center());
    }
}
