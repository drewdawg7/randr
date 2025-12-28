#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemKind {
    Sword,
    Dagger,
    BasicShield,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemType {
    Weapon,
    Shield
}

pub enum ItemError {
    MaxUpgradesReached,
}

