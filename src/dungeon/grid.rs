use bevy::prelude::*;
use tracing::instrument;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EntitySize {
    pub width: f32,
    pub height: f32,
}

impl Default for EntitySize {
    fn default() -> Self {
        Self {
            width: 32.0,
            height: 32.0,
        }
    }
}

impl EntitySize {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    pub fn single(tile_size: f32) -> Self {
        Self {
            width: tile_size,
            height: tile_size,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct OccupiedEntity {
    entity: Entity,
    pos: Vec2,
    size: EntitySize,
}

#[derive(Resource, Default)]
pub struct Occupancy {
    entities: Vec<OccupiedEntity>,
    player_pos: Option<Vec2>,
    player_size: EntitySize,
}

impl Occupancy {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_player(&mut self, pos: Vec2, size: EntitySize) {
        self.player_pos = Some(pos);
        self.player_size = size;
    }

    pub fn update_player_pos(&mut self, pos: Vec2) {
        self.player_pos = Some(pos);
    }

    pub fn can_place(&self, pos: Vec2, size: EntitySize) -> bool {
        !self.overlaps_any(pos, size)
    }

    #[instrument(level = "debug", skip(self), fields(pos = ?pos, size = ?size, entity = ?entity))]
    pub fn occupy(&mut self, pos: Vec2, size: EntitySize, entity: Entity) {
        self.entities.push(OccupiedEntity { entity, pos, size });
    }

    pub fn vacate(&mut self, entity: Entity) {
        self.entities.retain(|e| e.entity != entity);
    }

    pub fn entity_at(&self, pos: Vec2, radius: f32) -> Option<Entity> {
        self.entities
            .iter()
            .find(|e| Self::point_in_rect(pos, e.pos, e.size, radius))
            .map(|e| e.entity)
    }

    pub fn entity_overlapping(&self, pos: Vec2, size: EntitySize) -> Option<Entity> {
        self.entities
            .iter()
            .find(|e| Self::rects_overlap(pos, size, e.pos, e.size))
            .map(|e| e.entity)
    }

    fn overlaps_any(&self, pos: Vec2, size: EntitySize) -> bool {
        if let Some(player_pos) = self.player_pos {
            if Self::rects_overlap(pos, size, player_pos, self.player_size) {
                return true;
            }
        }
        self.entities
            .iter()
            .any(|e| Self::rects_overlap(pos, size, e.pos, e.size))
    }

    fn point_in_rect(point: Vec2, rect_pos: Vec2, rect_size: EntitySize, tolerance: f32) -> bool {
        let half_w = rect_size.width / 2.0 + tolerance;
        let half_h = rect_size.height / 2.0 + tolerance;
        (point.x - rect_pos.x).abs() < half_w && (point.y - rect_pos.y).abs() < half_h
    }

    fn rects_overlap(pos1: Vec2, size1: EntitySize, pos2: Vec2, size2: EntitySize) -> bool {
        let half_w1 = size1.width / 2.0;
        let half_h1 = size1.height / 2.0;
        let half_w2 = size2.width / 2.0;
        let half_h2 = size2.height / 2.0;

        (pos1.x - pos2.x).abs() < half_w1 + half_w2
            && (pos1.y - pos2.y).abs() < half_h1 + half_h2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn occupancy_basic() {
        let mut occupancy = Occupancy::new();
        let entity = Entity::from_raw_u32(1).unwrap();
        let pos = Vec2::new(100.0, 100.0);
        let size = EntitySize::new(32.0, 32.0);

        assert!(occupancy.can_place(pos, size));
        occupancy.occupy(pos, size, entity);
        assert!(!occupancy.can_place(pos, size));
        assert_eq!(occupancy.entity_overlapping(pos, size), Some(entity));

        occupancy.vacate(entity);
        assert!(occupancy.can_place(pos, size));
    }

    #[test]
    fn occupancy_no_overlap() {
        let mut occupancy = Occupancy::new();
        let entity = Entity::from_raw_u32(1).unwrap();
        let pos1 = Vec2::new(100.0, 100.0);
        let pos2 = Vec2::new(200.0, 200.0);
        let size = EntitySize::new(32.0, 32.0);

        occupancy.occupy(pos1, size, entity);
        assert!(occupancy.can_place(pos2, size));
    }

    #[test]
    fn occupancy_player_collision() {
        let mut occupancy = Occupancy::new();
        let player_pos = Vec2::new(100.0, 100.0);
        let player_size = EntitySize::new(32.0, 32.0);
        occupancy.set_player(player_pos, player_size);

        assert!(!occupancy.can_place(player_pos, player_size));
        assert!(occupancy.can_place(Vec2::new(200.0, 200.0), player_size));
    }
}
