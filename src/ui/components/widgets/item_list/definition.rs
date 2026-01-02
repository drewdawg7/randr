use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{List, ListItem as RatatuiListItem, ListState, Paragraph},
    Frame,
};

use crate::ui::components::utilities::{item_display, list_move_down, list_move_up, lock_prefix, selection_prefix, RETURN_ARROW};
use crate::ui::theme::{self as colors, DARK_GRAY};

use super::traits::{ItemFilter, ListItem, NoFilter};

/// Configuration for an ItemList widget.
#[derive(Clone)]
pub struct ItemListConfig {
    /// Whether to show the filter button at the top
    pub show_filter_button: bool,
    /// Whether to show scroll indicators ("... more above/below ...")
    pub show_scroll_indicators: bool,
    /// Maximum number of visible items before scrolling
    pub visible_count: usize,
    /// Whether to show a "Back" button at the end of the list
    pub show_back_button: bool,
    /// Label for the back button
    pub back_label: &'static str,
    /// Background color for the list
    pub background: Option<ratatui::style::Color>,
}

impl Default for ItemListConfig {
    fn default() -> Self {
        Self {
            show_filter_button: false,
            show_scroll_indicators: true,
            visible_count: 10,
            show_back_button: false,
            back_label: "Back",
            background: None,
        }
    }
}

/// A reusable item list widget with optional filtering and scrolling.
///
/// # Type Parameters
/// - `T`: The item type, must implement `ListItem`
/// - `F`: The filter type, must implement `ItemFilter<T>`. Defaults to `NoFilter`.
pub struct ItemList<T, F = NoFilter>
where
    T: ListItem,
    F: ItemFilter<T>,
{
    items: Vec<T>,
    filtered_indices: Vec<usize>,
    list_state: ListState,
    scroll_offset: usize,
    filter: F,
    config: ItemListConfig,
}

impl<T: ListItem, F: ItemFilter<T>> ItemList<T, F> {
    /// Create a new ItemList with the given configuration.
    pub fn new(config: ItemListConfig) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            items: Vec::new(),
            filtered_indices: Vec::new(),
            list_state,
            scroll_offset: 0,
            filter: F::default(),
            config,
        }
    }

    /// Create a new ItemList with default configuration.
    pub fn new_default() -> Self {
        Self::new(ItemListConfig::default())
    }

    /// Set the items to display, rebuilding the filtered list.
    pub fn set_items(&mut self, items: Vec<T>) {
        self.items = items;
        self.rebuild_filtered();
    }

    /// Rebuild the filtered indices based on the current filter.
    pub fn rebuild_filtered(&mut self) {
        self.filtered_indices = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| self.filter.matches(item))
            .map(|(i, _)| i)
            .collect();
    }

    /// Get the total count of items (filtered + back button if shown).
    fn total_count(&self) -> usize {
        let base = self.filtered_indices.len();
        if self.config.show_back_button {
            base + 1
        } else {
            base
        }
    }

    /// Get the currently selected index.
    pub fn selected_index(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }

    /// Check if the "Back" button is currently selected.
    pub fn is_back_selected(&self) -> bool {
        self.config.show_back_button && self.selected_index() == self.filtered_indices.len()
    }

    /// Get the currently selected item, if any.
    pub fn selected_item(&self) -> Option<&T> {
        let idx = self.selected_index();
        if idx < self.filtered_indices.len() {
            self.filtered_indices.get(idx).map(|&i| &self.items[i])
        } else {
            None
        }
    }

    /// Get the currently selected item mutably, if any.
    pub fn selected_item_mut(&mut self) -> Option<&mut T> {
        let idx = self.selected_index();
        if idx < self.filtered_indices.len() {
            self.filtered_indices.get(idx).copied().map(|i| &mut self.items[i])
        } else {
            None
        }
    }

    /// Get all items (unfiltered).
    pub fn items(&self) -> &[T] {
        &self.items
    }

    /// Get the current filter.
    pub fn filter(&self) -> &F {
        &self.filter
    }

    /// Move selection up with wrapping.
    pub fn move_up(&mut self) {
        let count = self.total_count();
        list_move_up(&mut self.list_state, count);
        self.adjust_scroll();
    }

    /// Move selection down with wrapping.
    pub fn move_down(&mut self) {
        let count = self.total_count();
        list_move_down(&mut self.list_state, count);
        self.adjust_scroll();
    }

    /// Cycle to the next filter.
    pub fn cycle_filter(&mut self) {
        self.filter = self.filter.next();
        self.list_state.select(Some(0));
        self.scroll_offset = 0;
        self.rebuild_filtered();
    }

    /// Reset selection to the first item.
    pub fn reset_selection(&mut self) {
        self.list_state.select(Some(0));
        self.scroll_offset = 0;
    }

    /// Reset everything including filter.
    pub fn reset(&mut self) {
        self.filter = F::default();
        self.reset_selection();
        self.rebuild_filtered();
    }

    /// Adjust scroll offset to keep selection visible.
    fn adjust_scroll(&mut self) {
        let selected = self.selected_index();
        if selected < self.scroll_offset {
            self.scroll_offset = selected;
        } else if selected >= self.scroll_offset + self.config.visible_count {
            self.scroll_offset = selected - self.config.visible_count + 1;
        }
    }

    /// Render the item list widget.
    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        // If showing filter button, render it at the top
        let list_area = if self.config.show_filter_button && area.height > 1 {
            let filter_area = Rect::new(area.x, area.y, area.width, 1);
            self.render_filter_button(frame, filter_area);
            Rect::new(area.x, area.y + 1, area.width, area.height.saturating_sub(1))
        } else {
            area
        };

        self.render_items(frame, list_area);
    }

    /// Render the filter button.
    fn render_filter_button(&self, frame: &mut Frame, area: Rect) {
        let button_text = format!("[ {} ]", self.filter.label());
        let button_style = Style::default()
            .fg(colors::CREAM_WOOD)
            .bg(colors::WOOD_BROWN);

        let bg_style = if let Some(bg) = self.config.background {
            Style::default().bg(bg)
        } else {
            Style::default()
        };

        let button = Paragraph::new(Line::from(Span::styled(button_text, button_style)))
            .style(bg_style);
        frame.render_widget(button, area);
    }

    /// Render the list items with scroll indicators.
    fn render_items(&mut self, frame: &mut Frame, area: Rect) {
        let selected = self.selected_index();

        // Calculate visible range
        let visible_count = self.config.visible_count.min(area.height as usize);
        let end_idx = (self.scroll_offset + visible_count).min(self.filtered_indices.len());

        let mut list_items: Vec<RatatuiListItem> = Vec::new();

        // Show scroll indicator if scrolled down
        let show_more_above = self.config.show_scroll_indicators && self.scroll_offset > 0;
        if show_more_above {
            list_items.push(RatatuiListItem::new(Line::from(Span::styled(
                "  ... more above ...",
                Style::default().fg(DARK_GRAY),
            ))));
        }

        // Render visible items
        for (offset, &item_idx) in self.filtered_indices[self.scroll_offset..end_idx]
            .iter()
            .enumerate()
        {
            let global_idx = self.scroll_offset + offset;
            let is_selected = global_idx == selected;
            let item = &self.items[item_idx];

            list_items.push(self.render_item(item, is_selected));
        }

        // Render back button if configured
        if self.config.show_back_button {
            let back_idx = self.filtered_indices.len();
            let is_back_in_view = back_idx >= self.scroll_offset
                && back_idx < self.scroll_offset + visible_count;

            if is_back_in_view || self.filtered_indices.is_empty() {
                let is_selected = selected == back_idx;
                list_items.push(RatatuiListItem::new(Line::from(vec![
                    selection_prefix(is_selected),
                    Span::styled(
                        format!("{} {}", RETURN_ARROW, self.config.back_label),
                        Style::default().fg(colors::WHITE),
                    ),
                ])));
            }
        }

        // Show scroll indicator if more items below
        let has_more_below = self.config.show_scroll_indicators
            && end_idx < self.filtered_indices.len();
        // Account for back button not being shown when checking "more below"
        let back_not_visible = self.config.show_back_button
            && self.filtered_indices.len() >= self.scroll_offset + visible_count;
        if has_more_below || back_not_visible {
            list_items.push(RatatuiListItem::new(Line::from(Span::styled(
                "  ... more below ...",
                Style::default().fg(DARK_GRAY),
            ))));
        }

        // Show empty message if no items
        if self.filtered_indices.is_empty() && !self.config.show_back_button {
            list_items.push(RatatuiListItem::new(Line::from(Span::styled(
                "  No items",
                Style::default().fg(DARK_GRAY),
            ))));
        }

        let list_style = if let Some(bg) = self.config.background {
            Style::default().bg(bg)
        } else {
            Style::default()
        };

        let list = List::new(list_items).style(list_style);
        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    /// Render a single item as a ListItem.
    fn render_item(&self, item: &T, is_selected: bool) -> RatatuiListItem<'static> {
        let mut spans = vec![selection_prefix(is_selected)];

        // Add lock prefix if applicable
        if item.show_lock() {
            if let Some(i) = item.item() {
                spans.push(lock_prefix(i));
            }
        }

        // Add item display (name with quality color and quantity/upgrades)
        if let Some(i) = item.item() {
            let quantity = item.quantity();
            spans.push(item_display(i, quantity));
        } else {
            // Non-item entries (like recipes)
            let color = colors::WHITE;
            spans.push(Span::styled(
                item.display_name().into_owned(),
                Style::default().fg(color),
            ));
        }

        // Add suffix spans (price, cost, etc.)
        spans.extend(item.suffix_spans());

        RatatuiListItem::new(Line::from(spans))
    }
}

impl<T: ListItem> ItemList<T, NoFilter> {
    /// Create a new ItemList without filtering.
    pub fn new_without_filter(config: ItemListConfig) -> Self {
        Self::new(config)
    }
}
