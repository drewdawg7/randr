use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::dungeon::Dungeon;
use crate::entities::Progression;
use crate::inventory::{EquipmentSlot, Inventory, InventoryItem};
use crate::item::enums::{ItemQuality, ItemType};
use crate::item::{Item, ItemId};
use crate::magic::tome::Tome;
use crate::magic::word::WordId;
use crate::player::Player;
use crate::stats::{StatInstance, StatSheet, StatType};
use crate::storage::Storage;

/// Complete serializable game state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSaveState {
    pub version: u32,
    pub player: PlayerState,
    pub storage: StorageState,
    pub dungeon: Option<DungeonState>,
}

impl GameSaveState {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn from_game(player: &Player, storage: &Storage, dungeon: Option<&Dungeon>) -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            player: PlayerState::from_player(player),
            storage: StorageState::from_storage(storage),
            dungeon: dungeon.map(DungeonState::from_dungeon),
        }
    }

    pub fn to_player(&self) -> Player {
        self.player.to_player()
    }

    pub fn to_storage(&self) -> Storage {
        self.storage.to_storage()
    }

    pub fn to_dungeon(&self) -> Option<Dungeon> {
        self.dungeon.as_ref().map(|d| d.to_dungeon())
    }
}

// Player state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub name: String,
    pub gold: i32,
    pub level: i32,
    pub xp: i32,
    pub total_xp: i32,
    pub inventory: InventoryState,
    pub stats: StatSheetState,
}

impl PlayerState {
    fn from_player(player: &Player) -> Self {
        Self {
            name: player.name.to_string(),
            gold: player.gold,
            level: player.prog.level,
            xp: player.prog.xp,
            total_xp: player.prog.total_xp,
            inventory: InventoryState::from_inventory(&player.inventory),
            stats: StatSheetState::from_sheet(&player.stats),
        }
    }

    fn to_player(&self) -> Player {
        Player {
            name: Box::leak(self.name.clone().into_boxed_str()),
            gold: self.gold,
            prog: Progression {
                level: self.level,
                xp: self.xp,
                total_xp: self.total_xp,
            },
            inventory: self.inventory.to_inventory(),
            stats: self.stats.to_sheet(),
        }
    }
}

// Storage state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageState {
    pub inventory: InventoryState,
}

impl StorageState {
    fn from_storage(storage: &Storage) -> Self {
        Self {
            inventory: InventoryState::from_inventory(&storage.inventory),
        }
    }

    fn to_storage(&self) -> Storage {
        Storage {
            inventory: self.inventory.to_inventory(),
        }
    }
}

// Inventory state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryState {
    pub items: Vec<InventoryItemState>,
    pub equipment: HashMap<String, InventoryItemState>,
    pub max_slots: usize,
}

impl InventoryState {
    fn from_inventory(inv: &Inventory) -> Self {
        Self {
            items: inv.items.iter().map(InventoryItemState::from_item).collect(),
            equipment: inv
                .equipment()
                .iter()
                .map(|(slot, item)| (format!("{:?}", slot), InventoryItemState::from_item(item)))
                .collect(),
            max_slots: inv.max_slots(),
        }
    }

    fn to_inventory(&self) -> Inventory {
        let mut inv = if self.max_slots == usize::MAX {
            Inventory::new_unlimited()
        } else {
            let mut i = Inventory::new();
            i.items = Vec::new(); // Clear default items
            i
        };

        // Restore items
        inv.items = self.items.iter().map(|i| i.to_inventory_item()).collect();

        // Restore equipment
        for (slot_str, item) in &self.equipment {
            if let Some(slot) = parse_equipment_slot(slot_str) {
                inv.equipment_mut().insert(slot, item.to_inventory_item());
            }
        }

        inv
    }
}

// Inventory item state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItemState {
    pub item: ItemState,
    pub quantity: u32,
}

impl InventoryItemState {
    fn from_item(item: &InventoryItem) -> Self {
        Self {
            item: ItemState::from_item(&item.item),
            quantity: item.quantity,
        }
    }

    fn to_inventory_item(&self) -> InventoryItem {
        InventoryItem {
            item: self.item.to_item(),
            quantity: self.quantity,
        }
    }
}

// Item state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemState {
    pub item_uuid: String,
    pub item_id: String,
    pub item_type: String,
    pub name: String,
    pub is_equipped: bool,
    pub is_locked: bool,
    pub num_upgrades: i32,
    pub max_upgrades: i32,
    pub max_stack_quantity: u32,
    pub base_stats: StatSheetState,
    pub stats: StatSheetState,
    pub gold_value: i32,
    pub quality: String,
    pub tome_data: Option<TomeState>,
}

impl ItemState {
    fn from_item(item: &Item) -> Self {
        Self {
            item_uuid: item.item_uuid.to_string(),
            item_id: format!("{:?}", item.item_id),
            item_type: format!("{:?}", item.item_type),
            name: item.name.clone(),
            is_equipped: item.is_equipped,
            is_locked: item.is_locked,
            num_upgrades: item.num_upgrades,
            max_upgrades: item.max_upgrades,
            max_stack_quantity: item.max_stack_quantity,
            base_stats: StatSheetState::from_sheet(&item.base_stats),
            stats: StatSheetState::from_sheet(&item.stats),
            gold_value: item.gold_value,
            quality: format!("{:?}", item.quality),
            tome_data: item.tome_data.as_ref().map(TomeState::from_tome),
        }
    }

    fn to_item(&self) -> Item {
        Item {
            item_uuid: Uuid::parse_str(&self.item_uuid).unwrap_or_else(|_| Uuid::new_v4()),
            item_id: parse_item_id(&self.item_id).unwrap_or(ItemId::Sword),
            item_type: parse_item_type(&self.item_type).unwrap_or(ItemType::QuestItem),
            name: self.name.clone(),
            is_equipped: self.is_equipped,
            is_locked: self.is_locked,
            num_upgrades: self.num_upgrades,
            max_upgrades: self.max_upgrades,
            max_stack_quantity: self.max_stack_quantity,
            base_stats: self.base_stats.to_sheet(),
            stats: self.stats.to_sheet(),
            gold_value: self.gold_value,
            quality: parse_item_quality(&self.quality).unwrap_or(ItemQuality::Normal),
            tome_data: self.tome_data.as_ref().map(|t| t.to_tome()),
        }
    }
}

// Tome state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomeState {
    pub pages: Vec<Option<PageState>>,
    pub active_page_index: usize,
    pub capacity: usize,
}

impl TomeState {
    fn from_tome(tome: &Tome) -> Self {
        Self {
            pages: tome
                .pages()
                .iter()
                .map(|p| p.as_ref().map(PageState::from_page))
                .collect(),
            active_page_index: tome.active_page_index(),
            capacity: tome.capacity(),
        }
    }

    fn to_tome(&self) -> Tome {
        let mut tome = Tome::new(self.capacity);
        // Restore pages
        for (i, page_state) in self.pages.iter().enumerate() {
            if let Some(page) = page_state {
                let p = page.to_page();
                if let Err(_) = tome.set_page(i, p) {
                    // Skip invalid pages
                }
            }
        }
        // Restore active page
        let _ = tome.set_active_page(self.active_page_index);
        tome
    }
}

// Page state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageState {
    pub words: Vec<String>,
}

impl PageState {
    fn from_page(page: &crate::magic::page::Page) -> Self {
        Self {
            words: page.words().iter().map(|w| format!("{:?}", w)).collect(),
        }
    }

    fn to_page(&self) -> crate::magic::page::Page {
        let mut page = crate::magic::page::Page::new();
        let words: Vec<WordId> = self
            .words
            .iter()
            .filter_map(|s| parse_word_id(s))
            .collect();
        if !words.is_empty() {
            page.inscribe(words);
        }
        page
    }
}

// StatSheet state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatSheetState {
    pub stats: HashMap<String, StatInstanceState>,
}

impl StatSheetState {
    fn from_sheet(sheet: &StatSheet) -> Self {
        Self {
            stats: sheet
                .stats()
                .iter()
                .map(|(t, i)| (format!("{:?}", t), StatInstanceState::from_instance(i)))
                .collect(),
        }
    }

    fn to_sheet(&self) -> StatSheet {
        let mut sheet = StatSheet::new();
        for (type_str, instance) in &self.stats {
            if let Some(stat_type) = parse_stat_type(type_str) {
                sheet.insert(instance.to_instance(stat_type));
            }
        }
        sheet
    }
}

// StatInstance state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatInstanceState {
    pub current_value: i32,
    pub max_value: i32,
}

impl StatInstanceState {
    fn from_instance(instance: &StatInstance) -> Self {
        Self {
            current_value: instance.current_value,
            max_value: instance.max_value,
        }
    }

    fn to_instance(&self, stat_type: StatType) -> StatInstance {
        StatInstance {
            stat_type,
            current_value: self.current_value,
            max_value: self.max_value,
        }
    }
}

// Dungeon state (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonState {
    pub name: String,
    pub player_position: (i32, i32),
    pub is_generated: bool,
    // Rooms are complex with non-serializable types, so we'll skip for now
    // The dungeon can be regenerated on load
}

impl DungeonState {
    fn from_dungeon(dungeon: &Dungeon) -> Self {
        Self {
            name: dungeon.name.clone(),
            player_position: dungeon.player_position,
            is_generated: dungeon.is_generated,
        }
    }

    fn to_dungeon(&self) -> Dungeon {
        // Return a default dungeon - proper restoration would require
        // full serialization of dungeon state
        Dungeon::default()
    }
}

// Helper parsing functions
fn parse_equipment_slot(s: &str) -> Option<EquipmentSlot> {
    match s {
        "Weapon" => Some(EquipmentSlot::Weapon),
        "OffHand" => Some(EquipmentSlot::OffHand),
        "Ring" => Some(EquipmentSlot::Ring),
        "Tool" => Some(EquipmentSlot::Tool),
        "Head" => Some(EquipmentSlot::Head),
        "Chest" => Some(EquipmentSlot::Chest),
        "Hands" => Some(EquipmentSlot::Hands),
        "Feet" => Some(EquipmentSlot::Feet),
        "Legs" => Some(EquipmentSlot::Legs),
        _ => None,
    }
}

fn parse_stat_type(s: &str) -> Option<StatType> {
    match s {
        "Health" => Some(StatType::Health),
        "Attack" => Some(StatType::Attack),
        "Defense" => Some(StatType::Defense),
        "GoldFind" => Some(StatType::GoldFind),
        "Mining" => Some(StatType::Mining),
        "MagicFind" => Some(StatType::MagicFind),
        _ => None,
    }
}

fn parse_item_quality(s: &str) -> Option<ItemQuality> {
    match s {
        "Poor" => Some(ItemQuality::Poor),
        "Normal" => Some(ItemQuality::Normal),
        "Improved" => Some(ItemQuality::Improved),
        "WellForged" => Some(ItemQuality::WellForged),
        "Masterworked" => Some(ItemQuality::Masterworked),
        "Mythic" => Some(ItemQuality::Mythic),
        _ => None,
    }
}

// Note: These parse functions need to handle all ItemId and WordId variants
// For now, providing basic implementations that return defaults
fn parse_item_id(_s: &str) -> Option<ItemId> {
    // This would need to match all ItemId variants
    // For now, return None to use default
    None
}

fn parse_item_type(_s: &str) -> Option<ItemType> {
    // This would need proper parsing of ItemType enum
    None
}

fn parse_word_id(_s: &str) -> Option<WordId> {
    // This would need to match all WordId variants
    None
}
