use std::collections::HashMap;

use crate::{entities::mob::MobId, registry::Registry};

use super::super::FieldId;

#[derive(Clone)]
pub struct FieldSpec {
    pub field_id: FieldId,
    pub name: &'static str,
    pub mob_weights: HashMap<MobId, i32>,
}

pub type FieldRegistry = Registry<FieldId, FieldSpec>;
