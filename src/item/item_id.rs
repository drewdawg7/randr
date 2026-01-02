#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemId {
    // Weapons
    Sword,
    Dagger,
    TinSword,
    CopperSword,
    BronzeSword,

    // Shields
    BasicShield,

    // Armor - Copper
    CopperHelmet,
    CopperChestplate,
    CopperGauntlets,
    CopperGreaves,
    CopperLeggings,

    // Armor - Tin
    TinHelmet,
    TinChestplate,
    TinGauntlets,
    TinGreaves,
    TinLeggings,

    // Armor - Bronze
    BronzeHelmet,
    BronzeChestplate,
    BronzeGauntlets,
    BronzeGreaves,
    BronzeLeggings,

    // Tools
    BronzePickaxe,

    // Accessories
    GoldRing,

    // Ores
    Coal,
    CopperOre,
    TinOre,

    // Ingots
    TinIngot,
    CopperIngot,
    BronzeIngot,

    // Materials
    Cowhide,
    SlimeGel,

    // Consumables
    BasicHPPotion,

    // Upgrade Materials
    QualityUpgradeStone,
}
