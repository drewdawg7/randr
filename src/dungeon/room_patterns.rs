//! Composable room patterns for dungeon layout design.

use rand::Rng;

use super::layout::DungeonLayout;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self { x, y, width, height }
    }

    pub fn right(&self) -> usize {
        self.x + self.width
    }

    pub fn bottom(&self) -> usize {
        self.y + self.height
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }
}

pub trait RoomPattern {
    fn apply(&self, layout: &mut DungeonLayout, bounds: Rect, rng: &mut impl Rng);
}

#[derive(Clone)]
pub enum RoomPatternKind {}

impl RoomPattern for RoomPatternKind {
    fn apply(&self, _layout: &mut DungeonLayout, _bounds: Rect, _rng: &mut impl Rng) {
        match *self {}
    }
}

#[derive(Clone, Default)]
pub struct ComposedPattern(Vec<RoomPatternKind>);

impl ComposedPattern {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(mut self, pattern: RoomPatternKind) -> Self {
        self.0.push(pattern);
        self
    }
}

impl RoomPattern for ComposedPattern {
    fn apply(&self, layout: &mut DungeonLayout, bounds: Rect, rng: &mut impl Rng) {
        for pattern in &self.0 {
            pattern.apply(layout, bounds, rng);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_contains_works() {
        let r = Rect::new(5, 5, 10, 10);
        assert!(r.contains(5, 5));
        assert!(r.contains(14, 14));
        assert!(!r.contains(4, 5));
        assert!(!r.contains(15, 5));
    }
}
