pub mod definition;
pub mod enums;
pub mod specs;

#[cfg(test)]
mod tests;

pub use definition::Recipe;
pub use specs::RecipeId;
pub use enums::{ForgeMaterial, RecipeError};
