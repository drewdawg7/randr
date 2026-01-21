use crate::ui::SelectionState;

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

impl SelectionState for ListState {
    fn selected(&self) -> usize {
        self.selected
    }

    fn count(&self) -> usize {
        self.count
    }

    fn set_selected(&mut self, index: usize) {
        self.selected = index;
    }

    /// Uses wrapping navigation (overrides default clamped behavior).
    fn up(&mut self) {
        self.up_wrap();
        self.update_scroll();
    }

    /// Uses wrapping navigation (overrides default clamped behavior).
    fn down(&mut self) {
        self.down_wrap();
        self.update_scroll();
    }

    fn reset(&mut self) {
        self.selected = 0;
        self.scroll_offset = 0;
    }
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

    /// Update scroll offset to keep selection visible.
    fn update_scroll(&mut self) {
        if self.selected < self.scroll_offset {
            self.scroll_offset = self.selected;
        } else if self.selected >= self.scroll_offset + self.visible_count {
            self.scroll_offset = self.selected - self.visible_count + 1;
        }
    }

    /// Update the item count.
    pub fn set_count(&mut self, count: usize) {
        self.count = count;
        self.clamp_to_bounds();
        self.update_scroll();
    }
}
