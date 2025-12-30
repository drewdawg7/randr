use crate::{entities::mob::MobKind, field::definition::Field};



impl Default for Field {
    fn default() -> Self {
        Self {
            name: "The Village Field".to_string(),
            spawnable_mobs: vec![
                MobKind::Goblin,
                MobKind::Slime
            ]
        }
    }
}
