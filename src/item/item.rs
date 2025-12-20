
use crate::registry::{Registry, RegistryDefaults, SpawnFromSpec};


#[derive(Debug, Clone)]
pub struct Item {
    pub kind: ItemKind,
    pub name: &'static str,
}


#[derive(Debug, Clone)]
pub struct ItemSpec {
    pub name: &'static str,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ItemKind {
    Sword,
    Dagger
}

pub type ItemRegistry = Registry<ItemKind, ItemSpec>;


impl SpawnFromSpec<ItemKind> for ItemSpec {
    type Output = Item;

    fn spawn_from_spec(kind: ItemKind, spec: &Self) -> Self::Output {
        Item {
            kind,
            name: spec.name,
        }
    }
}

impl RegistryDefaults<ItemKind> for ItemSpec {
    fn defaults() -> impl IntoIterator<Item = (ItemKind, Self)> {
        [
            (
                ItemKind::Sword,
                ItemSpec {
                    name: "Sword",
                }
            ),
            (
                ItemKind::Dagger,
                ItemSpec {
                    name: "Dagger",
                }
            )
        ]
    }
}
