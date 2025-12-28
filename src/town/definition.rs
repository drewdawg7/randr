use crate::{blacksmith::Blacksmith, store::Store};



pub struct Town {
    pub name: String,
    pub store: Store,
    pub blacksmith: Blacksmith
}


impl Town {
    pub fn new(name: String, store: Store, blacksmith: Blacksmith) -> Self {
        Self {
            name,
            store,
            blacksmith,
        }
    }
}
