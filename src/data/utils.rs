use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct StatRange(pub i32, pub i32);

impl StatRange {
    pub fn start(&self) -> i32 {
        self.0
    }

    pub fn end(&self) -> i32 {
        self.1
    }

    pub fn scale(&self, multiplier: f32) -> Self {
        Self(
            (self.0 as f32 * multiplier).round() as i32,
            (self.1 as f32 * multiplier).round() as i32,
        )
    }
}
