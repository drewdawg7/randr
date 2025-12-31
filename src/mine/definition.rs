use std::collections::HashMap;

use crate::mine::rock::{Rock, RockId};

pub struct Mine {
    pub name: String,
    pub rock_weights: HashMap<RockId, i32>,
    pub rocks: HashMap<RockId, Rock>,
}
