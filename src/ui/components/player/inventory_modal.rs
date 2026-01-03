use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use tuirealm::event::Key;

use crate::{
    inventory::{EquipmentSlot, HasInventory},
    item::{
        consumable::{use_consumable, ConsumableError},
        Item,
    },
    system::game_state,
    ui::components::widgets::item_list::{InventoryFilter, InventoryListItem, ItemList, ItemListConfig},
    ui::theme::{CREAM_WOOD, DARK_WALNUT, LIGHT_BEIGE, OAK_BROWN, TAN_WOOD, WOOD_BROWN},
};

// Subtle parchment-tinted background (warm tan/beige tint)
const PARCHMENT_BG: Color = Color::Rgb(58, 52, 46);

use super::item_details::render_item_details_for_modal;

pub struct InventoryModal {
    item_list: ItemList<InventoryListItem, InventoryFilter>,
}

impl InventoryModal {
    pub fn new() -> Self {
        let config = ItemListConfig {
            show_filter_button: true,
            show_scroll_indicators: true,
            visible_count: 10,
            show_back_button: false,
            back_label: "Back",
            background: Some(PARCHMENT_BG),
        };
        Self {
            item_list: ItemList::new(config),
        }
    }

    fn rebuild_items(&mut self) {
        let mut items = Vec::new();

        // Add equipped items first
        for slot in EquipmentSlot::all() {
            if let Some(inv_item) = game_state().player.get_equipped_item(*slot) {
                items.push(InventoryListItem {
                    inv_item: inv_item.clone(),
                    slot: Some(*slot),
                });
            }
        }

        // Add backpack items
        for inv_item in game_state().player.get_inventory_items().iter() {
            items.push(InventoryListItem {
                inv_item: inv_item.clone(),
                slot: inv_item.item.item_type.equipment_slot(),
            });
        }

        self.item_list.set_items(items);
    }

    pub fn reset(&mut self) {
        self.item_list.reset();
        self.rebuild_items();
    }

    fn toggle_equip(&mut self) {
        if let Some(list_item) = self.item_list.selected_item() {
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
    }

    fn toggle_lock(&mut self) {
        if let Some(list_item) = self.item_list.selected_item() {
            let item_uuid = list_item.inv_item.item.item_uuid;
            if let Some(inv_item) = game_state().player.find_item_by_uuid_mut(item_uuid) {
                inv_item.item.toggle_lock();
            }
        }
    }

    fn use_selected_consumable(&mut self) -> Option<String> {
        let list_item = self.item_list.selected_item()?;
        let inv_item = &list_item.inv_item;

        // Only allow using consumables
        if !inv_item.item.item_type.is_consumable() {
            return Some("Cannot use this item".to_string());
        }

        let gs = game_state();

        // Attempt to use the consumable
        match use_consumable(&mut gs.player, inv_item) {
            Ok(result) => {
                let description = result.describe();
                // Decrease quantity after successful use
                let item_uuid = inv_item.item.item_uuid;
                if let Some(inv_item) = gs.player.find_item_by_uuid_mut(item_uuid) {
                    if inv_item.quantity > 1 {
                        inv_item.quantity -= 1;
                    } else {
                        gs.player.remove_item(item_uuid);
                    }
                }
                Some(format!("Used {}! {}", result.item_name, description))
            }
            Err(ConsumableError::AlreadyAtFullHealth) => Some("Already at full health!".to_string()),
            Err(ConsumableError::NoEffectRegistered) => Some("This item has no effect".to_string()),
            Err(_) => Some("Cannot use this item".to_string()),
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.rebuild_items();

        let frame_area = frame.area();

        // Calculate sizes for inventory list modal
        let list_width = (frame_area.width * 30 / 100).min(35).max(25);
        let modal_height = (frame_area.height * 60 / 100).min(20).max(12);

        let start_x = (frame_area.width.saturating_sub(list_width)) / 2;
        let y = (frame_area.height.saturating_sub(modal_height)) / 2;

        // Left modal: Inventory list with ASCII border
        let border_area = Rect::new(start_x, y, list_width, modal_height);
        self.render_ascii_border(frame, border_area);

        // Inner area (inside the border - 1 char border on each side)
        let inner_x = start_x + 1;
        let inner_y = y + 1;
        let inner_width = list_width.saturating_sub(2);
        let inner_height = modal_height.saturating_sub(2);

        // Render the item list (with filter button at top)
        let list_area = Rect::new(inner_x, inner_y, inner_width, inner_height);
        self.item_list.render(frame, list_area);

        // Right modal: Item details (rendered beside list if toggled)
        let selected_item: Option<&Item> = self
            .item_list
            .selected_item()
            .map(|list_item| &list_item.inv_item.item);
        render_item_details_for_modal(frame, border_area, selected_item);
    }

    fn render_ascii_border(&self, frame: &mut Frame, area: Rect) {
        let width = area.width as usize;
        let height = area.height;

        // Parchment colors cycling for border characters
        let parchment_colors = [
            DARK_WALNUT, WOOD_BROWN, OAK_BROWN, TAN_WOOD, LIGHT_BEIGE, CREAM_WOOD, LIGHT_BEIGE,
            TAN_WOOD, OAK_BROWN,
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

            let left = Span::styled(
                left_ch.to_string(),
                Style::default().fg(left_color).bg(PARCHMENT_BG),
            );
            let right = Span::styled(
                right_ch.to_string(),
                Style::default().fg(right_color).bg(PARCHMENT_BG),
            );
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

    /// Handle keyboard input, returns true if modal should close
    pub fn handle_input(&mut self, key: Key) -> bool {
        match key {
            Key::Up => {
                self.item_list.move_up();
                false
            }
            Key::Down => {
                self.item_list.move_down();
                false
            }
            Key::Char('f') | Key::Char('F') => {
                self.item_list.cycle_filter();
                false
            }
            Key::Char('e') | Key::Char('E') => {
                self.toggle_equip();
                false
            }
            Key::Char('d') => {
                let gs = game_state();
                gs.show_item_details = !gs.show_item_details;
                false
            }
            Key::Char('l') => {
                self.toggle_lock();
                false
            }
            Key::Char('u') | Key::Char('U') => {
                if let Some(message) = self.use_selected_consumable() {
                    let gs = game_state();
                    // Check if the message indicates an error
                    if message.contains("Cannot") || message.contains("Already") || message.contains("no effect") {
                        gs.toasts.error(message);
                    } else {
                        gs.toasts.success(message);
                    }
                }
                false
            }
            Key::Esc | Key::Char('i') => true, // Close modal
            _ => false,
        }
    }
}
