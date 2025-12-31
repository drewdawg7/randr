use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{List, ListItem, ListState, Paragraph},
    Frame,
};
use tuirealm::event::Key;

use ratatui::style::Color;

use crate::{
    inventory::{EquipmentSlot, HasInventory, InventoryItem},
    item::{Item, ItemType},
    system::game_state,
    ui::components::utilities::{item_display, lock_prefix, selection_prefix},
    ui::theme::{DARK_GRAY, DARK_WALNUT, WOOD_BROWN, OAK_BROWN, TAN_WOOD, LIGHT_BEIGE, CREAM_WOOD},
};

// Subtle parchment-tinted background (warm tan/beige tint)
const PARCHMENT_BG: Color = Color::Rgb(58, 52, 46);

use super::item_details::render_item_details;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum InventoryFilter {
    #[default]
    All,
    Equipment,
    Materials,
    Consumables,
}

impl InventoryFilter {
    fn next(&self) -> Self {
        match self {
            InventoryFilter::All => InventoryFilter::Equipment,
            InventoryFilter::Equipment => InventoryFilter::Materials,
            InventoryFilter::Materials => InventoryFilter::Consumables,
            InventoryFilter::Consumables => InventoryFilter::All,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            InventoryFilter::All => "All",
            InventoryFilter::Equipment => "Equipment",
            InventoryFilter::Materials => "Materials",
            InventoryFilter::Consumables => "Consumables",
        }
    }

    fn matches(&self, item_type: &ItemType) -> bool {
        match self {
            InventoryFilter::All => true,
            InventoryFilter::Equipment => item_type.is_equipment(),
            InventoryFilter::Materials => item_type.is_material(),
            InventoryFilter::Consumables => item_type.is_consumable(),
        }
    }
}

struct InventoryListItem {
    inv_item: InventoryItem,
    slot: Option<EquipmentSlot>,
}

pub struct InventoryModal {
    list_state: ListState,
    items: Vec<InventoryListItem>,
    scroll_offset: usize,
    visible_count: usize,
    show_details: bool,
    filter: InventoryFilter,
}

impl InventoryModal {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            list_state,
            items: Vec::new(),
            scroll_offset: 0,
            visible_count: 10,
            show_details: false,
            filter: InventoryFilter::default(),
        }
    }

    fn rebuild_items(&mut self) {
        self.items.clear();

        // Add equipped items first (if filter matches)
        for slot in EquipmentSlot::all() {
            if let Some(inv_item) = game_state().player.get_equipped_item(*slot) {
                if self.filter.matches(&inv_item.item.item_type) {
                    self.items.push(InventoryListItem {
                        inv_item: inv_item.clone(),
                        slot: Some(*slot),
                    });
                }
            }
        }

        // Add backpack items (if filter matches)
        for inv_item in game_state().player.get_inventory_items().iter() {
            if self.filter.matches(&inv_item.item.item_type) {
                self.items.push(InventoryListItem {
                    inv_item: inv_item.clone(),
                    slot: inv_item.item.item_type.equipment_slot(),
                });
            }
        }
    }

    pub fn reset(&mut self) {
        self.list_state.select(Some(0));
        self.scroll_offset = 0;
        self.filter = InventoryFilter::default();
        self.rebuild_items();
    }

    fn cycle_filter(&mut self) {
        self.filter = self.filter.next();
        self.list_state.select(Some(0));
        self.scroll_offset = 0;
    }

    fn selected_index(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }

    fn adjust_scroll(&mut self) {
        let selected = self.selected_index();
        if selected < self.scroll_offset {
            self.scroll_offset = selected;
        } else if selected >= self.scroll_offset + self.visible_count {
            self.scroll_offset = selected - self.visible_count + 1;
        }
    }

    fn move_up(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let current = self.selected_index();
        let new_idx = if current == 0 {
            self.items.len() - 1
        } else {
            current - 1
        };
        self.list_state.select(Some(new_idx));
        self.adjust_scroll();
    }

    fn move_down(&mut self) {
        if self.items.is_empty() {
            return;
        }
        let current = self.selected_index();
        let new_idx = (current + 1) % self.items.len();
        self.list_state.select(Some(new_idx));
        self.adjust_scroll();
    }

    fn toggle_equip(&mut self) {
        let selected = self.selected_index();
        if selected >= self.items.len() {
            return;
        }

        let list_item = &self.items[selected];
        let item = &list_item.inv_item.item;
        let item_uuid = item.item_uuid;

        if let Some(slot) = list_item.slot {
            if item.is_equipped {
                let _ = game_state().player.unequip_item(slot);
            } else {
                game_state().player.equip_from_inventory(item_uuid, slot);
            }
        }
    }

    fn toggle_lock(&mut self) {
        let selected = self.selected_index();
        if selected >= self.items.len() {
            return;
        }

        let item_uuid = self.items[selected].inv_item.item.item_uuid;
        if let Some(inv_item) = game_state().player.find_item_by_uuid_mut(item_uuid) {
            inv_item.item.toggle_lock();
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.rebuild_items();

        let frame_area = frame.area();
        let gap = 2u16; // Gap between the two modals

        // Calculate sizes for two separate modals
        let list_width = (frame_area.width * 30 / 100).min(35).max(25);
        let details_width = (frame_area.width * 35 / 100).min(40).max(30);
        let modal_height = (frame_area.height * 60 / 100).min(20).max(12);

        let total_width = list_width + gap + details_width;
        let start_x = (frame_area.width.saturating_sub(total_width)) / 2;
        let y = (frame_area.height.saturating_sub(modal_height)) / 2;

        // Left modal: Inventory list with ASCII border
        let border_area = Rect::new(start_x, y, list_width, modal_height);
        self.render_ascii_border(frame, border_area);

        // Inner area (inside the border - 1 char border on each side)
        let inner_x = start_x + 1;
        let inner_y = y + 1;
        let inner_width = list_width.saturating_sub(2);
        let inner_height = modal_height.saturating_sub(2);

        // Render filter button at the top
        let filter_area = Rect::new(inner_x, inner_y, inner_width, 1);
        self.render_filter_button(frame, filter_area);

        // List area below the filter button
        let list_area = Rect::new(
            inner_x,
            inner_y + 1,
            inner_width,
            inner_height.saturating_sub(1),
        );
        self.render_item_list(frame, list_area);

        // Right modal: Item details (only if toggled on)
        if self.show_details {
            let details_area = Rect::new(start_x + list_width + gap, y, details_width, modal_height);

            let selected_item: Option<&Item> = if self.selected_index() < self.items.len() {
                Some(&self.items[self.selected_index()].inv_item.item)
            } else {
                None
            };
            render_item_details(frame, details_area, selected_item);
        }
    }

    fn render_ascii_border(&self, frame: &mut Frame, area: Rect) {
        let width = area.width as usize;
        let height = area.height;

        // Parchment colors cycling for border characters
        let parchment_colors = [
            DARK_WALNUT,
            WOOD_BROWN,
            OAK_BROWN,
            TAN_WOOD,
            LIGHT_BEIGE,
            CREAM_WOOD,
            LIGHT_BEIGE,
            TAN_WOOD,
            OAK_BROWN,
        ];

        // Border patterns (fuller block characters for top/bottom)
        const TOP_PATTERN: &str = "▄█▓▒░▒▓█";
        const BOTTOM_PATTERN: &str = "▀█▓▒░▒▓█";
        const LEFT_PATTERN: &[char] = &['║', '┃', '│', '┆', '┊'];
        const RIGHT_PATTERN: &[char] = &['║', '┃', '│', '┆', '┊'];

        let mut lines: Vec<Line> = Vec::new();

        // Top border with background
        let top_spans: Vec<Span> = TOP_PATTERN
            .chars()
            .cycle()
            .take(width)
            .enumerate()
            .map(|(i, ch)| {
                let color = parchment_colors[i % parchment_colors.len()];
                Span::styled(ch.to_string(), Style::default().fg(color).bg(PARCHMENT_BG))
            })
            .collect();
        lines.push(Line::from(top_spans));

        // Middle rows with side borders and background
        for row in 1..height.saturating_sub(1) {
            let left_ch = LEFT_PATTERN[row as usize % LEFT_PATTERN.len()];
            let right_ch = RIGHT_PATTERN[row as usize % RIGHT_PATTERN.len()];
            let left_color = parchment_colors[row as usize % parchment_colors.len()];
            let right_color = parchment_colors[(row as usize + 3) % parchment_colors.len()];

            let left = Span::styled(left_ch.to_string(), Style::default().fg(left_color).bg(PARCHMENT_BG));
            let right = Span::styled(right_ch.to_string(), Style::default().fg(right_color).bg(PARCHMENT_BG));
            let middle = Span::styled(
                " ".repeat(width.saturating_sub(2)),
                Style::default().bg(PARCHMENT_BG),
            );

            lines.push(Line::from(vec![left, middle, right]));
        }

        // Bottom border with background
        let bottom_spans: Vec<Span> = BOTTOM_PATTERN
            .chars()
            .cycle()
            .take(width)
            .enumerate()
            .map(|(i, ch)| {
                let color = parchment_colors[i % parchment_colors.len()];
                Span::styled(ch.to_string(), Style::default().fg(color).bg(PARCHMENT_BG))
            })
            .collect();
        lines.push(Line::from(bottom_spans));

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);
    }

    fn render_filter_button(&self, frame: &mut Frame, area: Rect) {
        // Button style matching parchment theme
        let button_text = format!("[ {} ]", self.filter.label());
        let button_style = Style::default()
            .fg(CREAM_WOOD)
            .bg(WOOD_BROWN);

        let button = Paragraph::new(Line::from(Span::styled(button_text, button_style)))
            .style(Style::default().bg(PARCHMENT_BG));
        frame.render_widget(button, area);
    }

    fn render_item_list(&mut self, frame: &mut Frame, area: Rect) {
        let selected = self.selected_index();

        // Get visible slice of items
        let end_idx = (self.scroll_offset + self.visible_count).min(self.items.len());
        let visible_items = if self.items.is_empty() {
            &[]
        } else {
            &self.items[self.scroll_offset..end_idx]
        };

        let mut list_items: Vec<ListItem> = Vec::new();

        for (offset, list_item) in visible_items.iter().enumerate() {
            let global_idx = self.scroll_offset + offset;
            let is_selected = global_idx == selected;

            let item = &list_item.inv_item.item;
            let quantity = if item.item_type.is_equipment() {
                None
            } else {
                Some(list_item.inv_item.quantity)
            };

            list_items.push(ListItem::new(Line::from(vec![
                selection_prefix(is_selected),
                lock_prefix(item),
                item_display(item, quantity),
            ])));
        }

        // Show scroll indicators if needed
        if self.scroll_offset > 0 {
            list_items.insert(
                0,
                ListItem::new(Line::from(Span::styled(
                    "  ... more above ...",
                    Style::default().fg(DARK_GRAY),
                ))),
            );
        }
        if end_idx < self.items.len() {
            list_items.push(ListItem::new(Line::from(Span::styled(
                "  ... more below ...",
                Style::default().fg(DARK_GRAY),
            ))));
        }

        // Show empty message if no items
        if self.items.is_empty() {
            list_items.push(ListItem::new(Line::from(Span::styled(
                "  No items",
                Style::default().fg(DARK_GRAY),
            ))));
        }

        let list = List::new(list_items).style(Style::default().bg(PARCHMENT_BG));
        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    /// Handle keyboard input, returns true if modal should close
    pub fn handle_input(&mut self, key: Key) -> bool {
        match key {
            Key::Up => {
                self.move_up();
                false
            }
            Key::Down => {
                self.move_down();
                false
            }
            Key::Enter => {
                self.cycle_filter();
                false
            }
            Key::Char('e') | Key::Char('E') => {
                self.toggle_equip();
                false
            }
            Key::Char('d') => {
                self.show_details = !self.show_details;
                false
            }
            Key::Char('L') => {
                self.toggle_lock();
                false
            }
            Key::Esc | Key::Char('i') => true, // Close modal
            _ => false,
        }
    }
}
