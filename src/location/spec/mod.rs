pub mod definition;
pub mod specs;
pub mod traits;

pub use definition::{
    BlacksmithData, FieldData, LocationData, LocationRegistry, LocationSpec, MineData, StoreData,
};
pub use specs::{VILLAGE_BLACKSMITH, VILLAGE_FIELD, VILLAGE_MINE, VILLAGE_STORE};
