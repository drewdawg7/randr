#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldId {
    VillageField,
}

#[derive(Debug)]
pub enum FieldError {
    MobSpawnError,
}
