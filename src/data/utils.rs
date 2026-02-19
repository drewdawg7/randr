use std::ops::RangeInclusive;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct StatRange(pub i32, pub i32);

impl From<StatRange> for RangeInclusive<i32> {
    fn from(r: StatRange) -> Self {
        r.0..=r.1
    }
}
