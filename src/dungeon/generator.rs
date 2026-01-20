use super::DungeonLayout;

pub trait LayoutGenerator {
    fn generate(&self) -> DungeonLayout;
}
