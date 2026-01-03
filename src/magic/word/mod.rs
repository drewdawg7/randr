mod definition;
mod specs;

pub use definition::{Element, WordId, WordProperties, WordSpec};
pub type WordRegistry = crate::registry::Registry<WordId, WordSpec>;
