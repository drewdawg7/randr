use std::collections::HashMap;

use crate::mine::{definition::Mine, rock::RockId};

impl Default for Mine {
    fn default() -> Self {
        let mut rock_weights = HashMap::new();
        rock_weights.insert(RockId::Tin, 2);
        rock_weights.insert(RockId::Coal, 2);
        rock_weights.insert(RockId::Copper, 2);
        rock_weights.insert(RockId::Mixed, 1);
        Self {
            name: "Village Mine".to_string(),
            rock_weights,
            current_rock: None,
        }
    }
}
