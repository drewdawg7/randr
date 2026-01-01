use std::collections::HashMap;

use crate::{item::ItemId, registry::Registry};

use super::super::enums::RecipeId;

#[derive(Clone)]
pub struct RecipeSpec {
    pub name: &'static str,
    pub ingredients: HashMap<ItemId, u32>,
    pub output: ItemId,
    pub output_quantity: u32,
}

pub type RecipeRegistry = Registry<RecipeId, RecipeSpec>;
