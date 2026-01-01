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
