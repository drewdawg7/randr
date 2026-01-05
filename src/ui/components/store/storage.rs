//! Storage screen - dual-panel item transfer interface.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};
use tuirealm::command::{Cmd, CmdResult, Direction as TuiDirection};
use tuirealm::event::Key;

use crate::{
    commands::{apply_result, execute, GameCommand},
    inventory::ManagesItems,
    system::game_state,
    ui::components::utilities::collect_player_items,
    ui::components::widgets::item_list::{
        DepositableItem, InventoryFilter, ItemList, ItemListConfig, StoredItem,
    },
    ui::theme as colors,
};

/// Which panel is currently focused.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StorageFocus {
    Player,
    Storage,
}

impl StorageFocus {
    pub fn toggle(&self) -> Self {
        match self {
            StorageFocus::Player => StorageFocus::Storage,
            StorageFocus::Storage => StorageFocus::Player,
        }
    }
}

pub struct StorageScreen {
    focus: StorageFocus,
    player_list: ItemList<DepositableItem, InventoryFilter>,
    storage_list: ItemList<StoredItem, InventoryFilter>,
}

#[allow(dead_code)]
impl StorageScreen {
    pub fn new() -> Self {
        let player_config = ItemListConfig {
            show_filter_button: true,
            show_scroll_indicators: true,
            visible_count: 10,
            show_back_button: false,
            back_label: "",
            background: None,
        };

        let storage_config = ItemListConfig {
            show_filter_button: true,
            show_scroll_indicators: true,
            visible_count: 10,
            show_back_button: false,
            back_label: "",
            background: None,
        };

        Self {
            focus: StorageFocus::Player,
            player_list: ItemList::new(player_config),
            storage_list: ItemList::new(storage_config),
        }
    }

    pub fn reset(&mut self) {
        self.focus = StorageFocus::Player;
        self.player_list.reset_selection();
        self.storage_list.reset_selection();
    }

    fn rebuild_player_items(&mut self) {
        let items: Vec<DepositableItem> = collect_player_items()
            .into_iter()
            .map(|inv_item| DepositableItem { inv_item })
            .collect();
        self.player_list.set_items(items);
    }

    fn rebuild_storage_items(&mut self) {
        let gs = game_state();
        let items: Vec<StoredItem> = gs
            .storage()
            .get_inventory_items()
            .iter()
            .cloned()
            .map(|inv_item| StoredItem { inv_item })
            .collect();
        self.storage_list.set_items(items);
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        self.rebuild_player_items();
        self.rebuild_storage_items();

        // Header area
        let header_height = 3;
        let header_area = Rect::new(area.x, area.y, area.width, header_height);
        self.render_header(frame, header_area);

        let content_area = Rect::new(
            area.x,
            area.y + header_height,
            area.width,
            area.height.saturating_sub(header_height),
        );

        // Split into two panels with separator
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(48),
                Constraint::Length(4), // Separator
                Constraint::Percentage(48),
            ])
            .split(content_area);

        let left_panel = chunks[0];
        let separator = chunks[1];
        let right_panel = chunks[2];

        // Render panels with focus indicator
        self.render_panel(frame, left_panel, "Inventory", StorageFocus::Player);
        self.render_separator(frame, separator);
        self.render_panel(frame, right_panel, "Storage", StorageFocus::Storage);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let buf = frame.buffer_mut();

        // Title
        let title = "Storage";
        let y = area.y;
        for (i, ch) in title.chars().enumerate() {
            if let Some(cell) = buf.cell_mut((area.x + i as u16 + 2, y)) {
                cell.set_char(ch);
                cell.set_fg(colors::CREAM_WOOD);
            }
        }

        // Instructions
        let hint = "[Tab] Switch Panel  [Enter] Transfer  [Backspace] Back";
        let y = area.y + 1;
        for (i, ch) in hint.chars().enumerate() {
            if let Some(cell) = buf.cell_mut((area.x + i as u16 + 2, y)) {
                cell.set_char(ch);
                cell.set_fg(colors::TAN_WOOD);
            }
        }

        // Separator line
        let y = area.y + 2;
        for x in 0..area.width {
            if let Some(cell) = buf.cell_mut((area.x + x, y)) {
                cell.set_char('─');
                cell.set_fg(colors::WOOD_BROWN);
            }
        }
    }

    fn render_panel(&mut self, frame: &mut Frame, area: Rect, title: &str, panel: StorageFocus) {
        let is_focused = self.focus == panel;

        let buf = frame.buffer_mut();

        // Panel title
        let prefix = if is_focused { "> " } else { "  " };
        let title_text = format!("{}{}", prefix, title);
        for (i, ch) in title_text.chars().enumerate() {
            if let Some(cell) = buf.cell_mut((area.x + i as u16, area.y)) {
                cell.set_char(ch);
                cell.set_fg(if is_focused {
                    colors::YELLOW
                } else {
                    colors::WHITE
                });
            }
        }

        // List area (below title)
        let list_area = Rect::new(
            area.x,
            area.y + 1,
            area.width,
            area.height.saturating_sub(1),
        );

        match panel {
            StorageFocus::Player => self.player_list.render(frame, list_area),
            StorageFocus::Storage => self.storage_list.render(frame, list_area),
        }
    }

    fn render_separator(&self, frame: &mut Frame, area: Rect) {
        let buf = frame.buffer_mut();
        for row in 0..area.height {
            if let Some(cell) = buf.cell_mut((area.x + 1, area.y + row)) {
                cell.set_char('│');
                cell.set_fg(colors::WOOD_BROWN);
            }
        }
    }

    /// Returns true if the back button is selected in player panel.
    pub fn is_back_selected(&self) -> bool {
        self.focus == StorageFocus::Player && self.player_list.is_back_selected()
    }

    pub fn handle_cmd(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(TuiDirection::Up) => {
                match self.focus {
                    StorageFocus::Player => self.player_list.move_up(),
                    StorageFocus::Storage => self.storage_list.move_up(),
                }
                CmdResult::Changed(tuirealm::State::None)
            }
            Cmd::Move(TuiDirection::Down) => {
                match self.focus {
                    StorageFocus::Player => self.player_list.move_down(),
                    StorageFocus::Storage => self.storage_list.move_down(),
                }
                CmdResult::Changed(tuirealm::State::None)
            }
            Cmd::Submit => {
                match self.focus {
                    StorageFocus::Player => {
                        // Back button check is done by caller
                        if let Some(item) = self.player_list.selected_item() {
                            let uuid = item.inv_item.item.item_uuid;
                            let result = execute(GameCommand::DepositItem { item_uuid: uuid });
                            apply_result(&result);
                        }
                    }
                    StorageFocus::Storage => {
                        if let Some(item) = self.storage_list.selected_item() {
                            let uuid = item.inv_item.item.item_uuid;
                            let result = execute(GameCommand::WithdrawItem { item_uuid: uuid });
                            apply_result(&result);
                        }
                    }
                }
                CmdResult::Submit(tuirealm::State::None)
            }
            _ => CmdResult::None,
        }
    }

    pub fn handle_key(&mut self, key: Key) -> bool {
        match key {
            Key::Tab => {
                self.focus = self.focus.toggle();
                true
            }
            Key::Char('f') | Key::Char('F') => {
                match self.focus {
                    StorageFocus::Player => self.player_list.cycle_filter(),
                    StorageFocus::Storage => self.storage_list.cycle_filter(),
                }
                true
            }
            _ => false,
        }
    }
}

impl Default for StorageScreen {
    fn default() -> Self {
        Self::new()
    }
}
