#[derive(Debug, Clone)]
pub enum MobQuality {
    Normal,
    Boss,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MobId {
    Slime,
    Goblin,
    Cow,
    Dragon,
}
