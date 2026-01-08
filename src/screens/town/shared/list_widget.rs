use bevy::prelude::*;

/// State for tracking selection in a list.
#[derive(Default, Clone)]
pub struct ListState {
    /// Currently selected index.
    pub selected: usize,
    /// Total number of items.
    pub count: usize,
    /// Scroll offset for visible window.
    pub scroll_offset: usize,
    /// Number of visible items.
    pub visible_count: usize,
}

impl ListState {
    /// Create a new list state with the given item count.
    pub fn new(count: usize) -> Self {
        Self {
            selected: 0,
            count,
            scroll_offset: 0,
            visible_count: 10,
        }
    }

    /// Move selection up, wrapping at top.
    pub fn move_up(&mut self) {
        if self.count == 0 {
            return;
        }
        if self.selected == 0 {
            self.selected = self.count.saturating_sub(1);
        } else {
            self.selected -= 1;
        }
        self.update_scroll();
    }

    /// Move selection down, wrapping at bottom.
    pub fn move_down(&mut self) {
        if self.count == 0 {
            return;
        }
        self.selected = (self.selected + 1) % self.count;
        self.update_scroll();
    }

    /// Update scroll offset to keep selection visible.
    fn update_scroll(&mut self) {
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        } else if self.selected >= self.scroll_offset + self.visible_count {
            self.scroll_offset = self.selected - self.visible_count + 1;
        }
    }

    /// Reset selection to first item.
    pub fn reset(&mut self) {
        self.selected = 0;
        self.scroll_offset = 0;
    }

    /// Update the item count.
    pub fn set_count(&mut self, count: usize) {
        self.count = count;
        if self.selected >= count && count > 0 {
            self.selected = count - 1;
        }
        self.update_scroll();
    }
}

/// A scrollable list widget for Bevy UI.
pub struct ListWidget<T> {
    /// Items in the list.
    pub items: Vec<T>,
    /// Selection state.
    pub state: ListState,
}

impl<T> Default for ListWidget<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ListWidget<T> {
    /// Create a new empty list widget.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            state: ListState::default(),
        }
    }

    /// Set the items in the list.
    pub fn set_items(&mut self, items: Vec<T>) {
        self.state.set_count(items.len());
        self.items = items;
    }

    /// Get the currently selected item, if any.
    pub fn selected_item(&self) -> Option<&T> {
        self.items.get(self.state.selected)
    }

    /// Move selection up.
    pub fn move_up(&mut self) {
        self.state.move_up();
    }

    /// Move selection down.
    pub fn move_down(&mut self) {
        self.state.move_down();
    }

    /// Reset selection.
    pub fn reset(&mut self) {
        self.state.reset();
    }

    /// Get items in the visible window.
    pub fn visible_items(&self) -> impl Iterator<Item = (usize, &T)> {
        let start = self.state.scroll_offset;
        let end = (start + self.state.visible_count).min(self.items.len());
        self.items[start..end]
            .iter()
            .enumerate()
            .map(move |(i, item)| (start + i, item))
    }

    /// Check if an index is currently selected.
    pub fn is_selected(&self, index: usize) -> bool {
        index == self.state.selected
    }
}
