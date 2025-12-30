use crate::{blacksmith::Blacksmith, field::definition::Field, store::Store};



pub struct Town {
    pub name: String,
    pub store: Store,
    pub blacksmith: Blacksmith,
    pub field: Field
}


impl Town {
    pub fn new(name: String, store: Store, blacksmith: Blacksmith, field: Field)
    -> Self {
        Self {
            name,
            store,
            blacksmith,
            field
        }
    }
}
