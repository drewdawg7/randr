
use std::collections::HashMap;

use crate::mine::{definition::Mine, rock::{Rock, RockId}};

impl Default for Mine {
    fn default() -> Self {
        let mut rock_weights = HashMap::new();
        rock_weights.insert(RockId::Tin, 1);
        rock_weights.insert(RockId::Coal, 1);
        rock_weights.insert(RockId::Copper, 1);
        let mut rocks = HashMap::new();
        rocks.insert(RockId::Tin, Rock::tin_rock());
        rocks.insert(RockId::Coal, Rock::coal_rock());
        rocks.insert(RockId::Copper, Rock::copper_rock());
        Self {
            name: "Village Mine".to_string(),
            rock_weights,
            rocks
        }
    }
}
