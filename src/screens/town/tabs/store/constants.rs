use crate::item::ItemId;
use crate::screens::town::shared::MenuOption;

pub const STORE_MENU_OPTIONS: &[MenuOption] = &[
    MenuOption {
        label: "Buy",
        description: Some("Purchase items"),
    },
    MenuOption {
        label: "Sell",
        description: Some("Sell your items"),
    },
    MenuOption {
        label: "Storage",
        description: Some("Access your storage"),
    },
];

pub const STORAGE_MENU_OPTIONS: &[MenuOption] = &[
    MenuOption {
        label: "View Storage",
        description: Some("View and withdraw stored items"),
    },
    MenuOption {
        label: "Deposit Items",
        description: Some("Store items from your inventory"),
    },
];

/// Item available for purchase in the store.
#[derive(Clone, Copy)]
pub struct BuyableItem {
    pub item_id: ItemId,
    pub name: &'static str,
    pub price: i32,
    pub description: &'static str,
}

pub const BUYABLE_ITEMS: &[BuyableItem] = &[
    BuyableItem {
        item_id: ItemId::BasicHPPotion,
        name: "Health Potion",
        price: 50,
        description: "Restores 50 HP",
    },
    BuyableItem {
        item_id: ItemId::Sword,
        name: "Sword",
        price: 100,
        description: "A basic sword (+10 ATK)",
    },
    BuyableItem {
        item_id: ItemId::BasicShield,
        name: "Basic Shield",
        price: 80,
        description: "Basic protection (+4 DEF)",
    },
    BuyableItem {
        item_id: ItemId::CopperHelmet,
        name: "Copper Helmet",
        price: 200,
        description: "Copper armor (+36 DEF)",
    },
];
