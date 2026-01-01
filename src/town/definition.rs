use crate::{location::{Blacksmith, Field, Store}, mine::Mine};

pub struct Town {
    pub name: String,
    pub store: Store,
    pub blacksmith: Blacksmith,
    pub field: Field,
    pub mine: Mine,
}

impl Town {
    pub fn new(name: String, store: Store, blacksmith: Blacksmith, field: Field, mine: Mine) -> Self {
        Self {
            name,
            store,
            blacksmith,
            field,
            mine,
        }
    }
}
