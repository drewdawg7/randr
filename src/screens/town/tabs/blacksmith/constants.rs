use crate::screens::town::shared::MenuOption;

pub const MENU_OPTIONS: &[MenuOption] = &[
    MenuOption {
        label: "Upgrade",
        description: Some("Upgrade your equipment"),
    },
    MenuOption {
        label: "Quality",
        description: Some("Improve item quality"),
    },
    MenuOption {
        label: "Smelt",
        description: Some("Break down items for materials"),
    },
    MenuOption {
        label: "Forge",
        description: Some("Craft new items"),
    },
];
