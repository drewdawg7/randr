#[cfg(test)]
use crate::dungeon::{
    definition::{Dungeon, DungeonRoom, DUNGEON_SIZE, MAX_FILL_PERCENT},
    enums::{Direction, DungeonError, RoomType},
};

// ==================== DungeonRoom tests ====================

#[test]
fn dungeon_room_new_creates_monster_room() {
    let room = DungeonRoom::new(RoomType::Monster, 2, 3);

    assert_eq!(room.room_type, RoomType::Monster);
    assert_eq!(room.x, 2);
    assert_eq!(room.y, 3);
    assert!(!room.is_cleared);
    assert!(!room.is_visited);
    assert!(!room.is_revealed);
    assert!(room.chest.is_none());
    assert!(!room.has_healed);
}

#[test]
fn dungeon_room_new_chest_room_has_chest() {
    let room = DungeonRoom::new(RoomType::Chest, 0, 0);

    assert_eq!(room.room_type, RoomType::Chest);
    assert!(room.chest.is_some());
    assert!(!room.is_cleared);
}

#[test]
fn dungeon_room_new_rest_room_is_pre_cleared() {
    let room = DungeonRoom::new(RoomType::Rest, 1, 1);

    assert_eq!(room.room_type, RoomType::Rest);
    assert!(room.is_cleared);
    assert!(room.chest.is_none());
}

#[test]
fn dungeon_room_new_boss_room_not_cleared() {
    let room = DungeonRoom::new(RoomType::Boss, 4, 4);

    assert_eq!(room.room_type, RoomType::Boss);
    assert!(!room.is_cleared);
}

#[test]
fn dungeon_room_visit_sets_visited_and_revealed() {
    let mut room = DungeonRoom::new(RoomType::Monster, 0, 0);
    assert!(!room.is_visited);
    assert!(!room.is_revealed);

    room.visit();

    assert!(room.is_visited);
    assert!(room.is_revealed);
}

#[test]
fn dungeon_room_reveal_only_sets_revealed() {
    let mut room = DungeonRoom::new(RoomType::Monster, 0, 0);

    room.reveal();

    assert!(room.is_revealed);
    assert!(!room.is_visited);
}

#[test]
fn dungeon_room_clear_marks_cleared() {
    let mut room = DungeonRoom::new(RoomType::Monster, 0, 0);
    assert!(!room.is_cleared);

    room.clear();

    assert!(room.is_cleared);
}

#[test]
fn dungeon_room_chest_can_be_taken() {
    let mut room = DungeonRoom::new(RoomType::Chest, 0, 0);
    assert!(room.chest.is_some());

    // Take the chest directly (without calling roll_drops which needs game state)
    let chest = room.chest.take();

    assert!(chest.is_some());
    assert!(room.chest.is_none());
}

#[test]
fn dungeon_room_chest_take_twice_returns_none() {
    let mut room = DungeonRoom::new(RoomType::Chest, 0, 0);

    let _first = room.chest.take();
    let second = room.chest.take();

    assert!(second.is_none());
}

#[test]
fn dungeon_room_open_chest_on_non_chest_room_returns_empty() {
    let mut room = DungeonRoom::new(RoomType::Monster, 0, 0);

    let loot = room.open_chest(0, |_| None);

    assert!(loot.is_empty());
}

// ==================== Direction enum tests ====================

#[test]
fn direction_offset_north() {
    assert_eq!(Direction::North.offset(), (0, -1));
}

#[test]
fn direction_offset_east() {
    assert_eq!(Direction::East.offset(), (1, 0));
}

#[test]
fn direction_offset_south() {
    assert_eq!(Direction::South.offset(), (0, 1));
}

#[test]
fn direction_offset_west() {
    assert_eq!(Direction::West.offset(), (-1, 0));
}

#[test]
fn direction_name_returns_correct_strings() {
    assert_eq!(Direction::North.name(), "North");
    assert_eq!(Direction::East.name(), "East");
    assert_eq!(Direction::South.name(), "South");
    assert_eq!(Direction::West.name(), "West");
}

#[test]
fn direction_all_returns_four_directions() {
    let all = Direction::all();

    assert_eq!(all.len(), 4);
    assert!(all.contains(&Direction::North));
    assert!(all.contains(&Direction::East));
    assert!(all.contains(&Direction::South));
    assert!(all.contains(&Direction::West));
}

// ==================== Dungeon core methods tests ====================

#[test]
fn dungeon_default_creates_empty_grid() {
    let dungeon = Dungeon::default();

    assert_eq!(dungeon.name, "Village Dungeon");
    assert!(!dungeon.is_generated);
    assert_eq!(dungeon.player_position, (0, 0));
    assert!(dungeon.boss.is_none());

    // All rooms should be None
    for row in &dungeon.rooms {
        for room in row {
            assert!(room.is_none());
        }
    }
}

#[test]
fn dungeon_default_has_mob_table() {
    let dungeon = Dungeon::default();

    assert!(!dungeon.mob_table.is_empty());
}

#[cfg(test)]
fn create_test_dungeon_with_rooms() -> Dungeon {
    let mut dungeon = Dungeon::default();
    // Create a simple 3-room dungeon in an L shape:
    // [M] [ ] [ ]
    // [C] [ ] [ ]
    // [R] [ ] [ ]
    dungeon.rooms[0][0] = Some(DungeonRoom::new(RoomType::Monster, 0, 0));
    dungeon.rooms[1][0] = Some(DungeonRoom::new(RoomType::Chest, 0, 1));
    dungeon.rooms[2][0] = Some(DungeonRoom::new(RoomType::Rest, 0, 2));
    dungeon.player_position = (0, 0);
    dungeon
}

#[test]
fn dungeon_get_room_returns_room_at_valid_position() {
    let dungeon = create_test_dungeon_with_rooms();

    let room = dungeon.get_room(0, 0);

    assert!(room.is_some());
    assert_eq!(room.unwrap().room_type, RoomType::Monster);
}

#[test]
fn dungeon_get_room_returns_none_for_empty_position() {
    let dungeon = create_test_dungeon_with_rooms();

    let room = dungeon.get_room(1, 1);

    assert!(room.is_none());
}

#[test]
fn dungeon_get_room_returns_none_for_out_of_bounds() {
    let dungeon = create_test_dungeon_with_rooms();

    assert!(dungeon.get_room(-1, 0).is_none());
    assert!(dungeon.get_room(0, -1).is_none());
    assert!(dungeon.get_room(DUNGEON_SIZE as i32, 0).is_none());
    assert!(dungeon.get_room(0, DUNGEON_SIZE as i32).is_none());
}

#[test]
fn dungeon_get_room_mut_allows_modification() {
    let mut dungeon = create_test_dungeon_with_rooms();

    if let Some(room) = dungeon.get_room_mut(0, 0) {
        room.clear();
    }

    assert!(dungeon.get_room(0, 0).unwrap().is_cleared);
}

#[test]
fn dungeon_current_room_returns_player_room() {
    let dungeon = create_test_dungeon_with_rooms();

    let room = dungeon.current_room();

    assert!(room.is_some());
    assert_eq!(room.unwrap().x, 0);
    assert_eq!(room.unwrap().y, 0);
}

#[test]
fn dungeon_current_room_mut_allows_modification() {
    let mut dungeon = create_test_dungeon_with_rooms();

    if let Some(room) = dungeon.current_room_mut() {
        room.visit();
    }

    assert!(dungeon.current_room().unwrap().is_visited);
}

#[test]
fn dungeon_move_player_success() {
    let mut dungeon = create_test_dungeon_with_rooms();
    dungeon.player_position = (0, 0);

    let result = dungeon.move_player(Direction::South);

    assert!(result.is_ok());
    assert_eq!(dungeon.player_position, (0, 1));
}

#[test]
fn dungeon_move_player_marks_room_visited() {
    let mut dungeon = create_test_dungeon_with_rooms();
    dungeon.player_position = (0, 0);

    dungeon.move_player(Direction::South).unwrap();

    let room = dungeon.get_room(0, 1).unwrap();
    assert!(room.is_visited);
    assert!(room.is_revealed);
}

#[test]
fn dungeon_move_player_fails_when_no_room() {
    let mut dungeon = create_test_dungeon_with_rooms();
    dungeon.player_position = (0, 0);

    let result = dungeon.move_player(Direction::East);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DungeonError::NoRoomAtPosition);
    assert_eq!(dungeon.player_position, (0, 0)); // Position unchanged
}

#[test]
fn dungeon_move_player_fails_when_out_of_bounds() {
    let mut dungeon = create_test_dungeon_with_rooms();
    dungeon.player_position = (0, 0);

    let result = dungeon.move_player(Direction::North);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), DungeonError::NoRoomAtPosition);
}

#[test]
fn dungeon_available_directions_returns_valid_moves() {
    let dungeon = create_test_dungeon_with_rooms();

    let directions = dungeon.available_directions();

    // From (0,0), only South has a room
    assert_eq!(directions.len(), 1);
    assert!(directions.contains(&Direction::South));
}

#[test]
fn dungeon_available_directions_empty_when_isolated() {
    let mut dungeon = Dungeon::default();
    dungeon.rooms[2][2] = Some(DungeonRoom::new(RoomType::Monster, 2, 2));
    dungeon.player_position = (2, 2);

    let directions = dungeon.available_directions();

    assert!(directions.is_empty());
}

#[test]
fn dungeon_reveal_adjacent_rooms_reveals_neighbors() {
    let mut dungeon = create_test_dungeon_with_rooms();

    dungeon.reveal_adjacent_rooms(0, 1);

    // Room at (0,0) should be revealed (north of 0,1)
    assert!(dungeon.get_room(0, 0).unwrap().is_revealed);
    // Room at (0,2) should be revealed (south of 0,1)
    assert!(dungeon.get_room(0, 2).unwrap().is_revealed);
}

#[test]
fn dungeon_mark_start_visited_marks_player_room() {
    let mut dungeon = create_test_dungeon_with_rooms();
    assert!(!dungeon.get_room(0, 0).unwrap().is_visited);

    dungeon.mark_start_visited();

    assert!(dungeon.get_room(0, 0).unwrap().is_visited);
}

#[test]
fn dungeon_room_count_counts_correctly() {
    let dungeon = create_test_dungeon_with_rooms();

    assert_eq!(dungeon.room_count(), 3);
}

#[test]
fn dungeon_room_count_empty_dungeon() {
    let dungeon = Dungeon::default();

    assert_eq!(dungeon.room_count(), 0);
}

#[test]
fn dungeon_cleared_count_counts_cleared_rooms() {
    let mut dungeon = create_test_dungeon_with_rooms();
    // Rest room at (0,2) is pre-cleared

    assert_eq!(dungeon.cleared_count(), 1);

    dungeon.get_room_mut(0, 0).unwrap().clear();
    assert_eq!(dungeon.cleared_count(), 2);
}

#[test]
fn dungeon_is_completed_false_when_rooms_not_cleared() {
    let dungeon = create_test_dungeon_with_rooms();

    assert!(!dungeon.is_completed());
}

#[test]
fn dungeon_is_completed_true_when_all_cleared() {
    let mut dungeon = create_test_dungeon_with_rooms();
    dungeon.get_room_mut(0, 0).unwrap().clear();
    dungeon.get_room_mut(0, 1).unwrap().clear();
    // (0,2) is Rest room, already cleared

    assert!(dungeon.is_completed());
}

#[test]
fn dungeon_get_neighbors_returns_adjacent_rooms() {
    let dungeon = create_test_dungeon_with_rooms();
    let room = dungeon.get_room(0, 1).unwrap();

    let neighbors = dungeon.get_neighbors(room);

    // North (0,0) exists, East (1,1) doesn't, South (0,2) exists, West (-1,1) doesn't
    assert_eq!(neighbors.len(), 4);
    assert!(neighbors[0].is_some()); // North
    assert!(neighbors[1].is_none()); // East
    assert!(neighbors[2].is_some()); // South
    assert!(neighbors[3].is_none()); // West
}

// ==================== Dungeon generation tests ====================

#[test]
fn dungeon_generate_creates_rooms() {
    let mut dungeon = Dungeon::default();

    dungeon.generate();

    assert!(dungeon.is_generated);
    assert!(dungeon.room_count() >= 5); // MIN_ROOMS
}

#[test]
fn dungeon_generate_respects_max_fill() {
    let mut dungeon = Dungeon::default();

    dungeon.generate();

    let max_rooms = ((DUNGEON_SIZE * DUNGEON_SIZE) as f32 * MAX_FILL_PERCENT) as usize;
    assert!(dungeon.room_count() <= max_rooms);
}

#[test]
fn dungeon_generate_has_chest_room() {
    let mut dungeon = Dungeon::default();

    dungeon.generate();

    let has_chest = dungeon
        .rooms
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|r| r.as_ref())
        .any(|r| r.room_type == RoomType::Chest);

    assert!(has_chest, "Dungeon should have at least one Chest room");
}

#[test]
fn dungeon_generate_has_rest_room() {
    let mut dungeon = Dungeon::default();

    dungeon.generate();

    let has_rest = dungeon
        .rooms
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|r| r.as_ref())
        .any(|r| r.room_type == RoomType::Rest);

    assert!(has_rest, "Dungeon should have at least one Rest room");
}

#[test]
fn dungeon_generate_has_boss_room() {
    let mut dungeon = Dungeon::default();

    dungeon.generate();

    let boss_count = dungeon
        .rooms
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|r| r.as_ref())
        .filter(|r| r.room_type == RoomType::Boss)
        .count();

    assert_eq!(boss_count, 1, "Dungeon should have exactly one Boss room");
}

#[test]
fn dungeon_generate_entry_room_is_visited() {
    let mut dungeon = Dungeon::default();

    dungeon.generate();

    let entry_room = dungeon.current_room().unwrap();
    assert!(entry_room.is_visited);
    assert!(entry_room.is_revealed);
}

#[test]
fn dungeon_generate_adjacent_rooms_revealed() {
    let mut dungeon = Dungeon::default();

    dungeon.generate();

    let (x, y) = dungeon.player_position;

    // Check all adjacent positions for revealed rooms
    let offsets = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    for (dx, dy) in offsets {
        if let Some(room) = dungeon.get_room(x + dx, y + dy) {
            assert!(
                room.is_revealed,
                "Adjacent room at ({}, {}) should be revealed",
                x + dx,
                y + dy
            );
        }
    }
}

#[test]
fn dungeon_generate_player_starts_on_edge() {
    let mut dungeon = Dungeon::default();

    dungeon.generate();

    let (x, y) = dungeon.player_position;
    let size = DUNGEON_SIZE as i32;

    let on_edge = x == 0 || x == size - 1 || y == 0 || y == size - 1;
    assert!(on_edge, "Player should start on dungeon edge");
}

#[test]
fn dungeon_generate_entry_room_is_monster_type() {
    let mut dungeon = Dungeon::default();

    dungeon.generate();

    let entry_room = dungeon.current_room().unwrap();
    assert_eq!(
        entry_room.room_type,
        RoomType::Monster,
        "Entry room should be Monster type"
    );
}

#[test]
fn dungeon_reset_clears_dungeon() {
    let mut dungeon = Dungeon::default();
    dungeon.generate();
    assert!(dungeon.is_generated);
    assert!(dungeon.room_count() > 0);

    dungeon.reset();

    assert!(!dungeon.is_generated);
    assert_eq!(dungeon.room_count(), 0);
    assert_eq!(dungeon.player_position, (0, 0));
}

// ==================== Integration/simulation tests ====================

#[test]
fn player_explores_dungeon_simulation() {
    let mut dungeon = Dungeon::default();
    dungeon.generate();

    let initial_visited = dungeon
        .rooms
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|r| r.as_ref())
        .filter(|r| r.is_visited)
        .count();

    // Entry room should be visited
    assert_eq!(initial_visited, 1);

    // Move through available directions until we've visited multiple rooms
    let mut moves = 0;
    let max_moves = 10;

    while moves < max_moves {
        let directions = dungeon.available_directions();
        if directions.is_empty() {
            break;
        }

        // Try to move in first available direction
        let _ = dungeon.move_player(directions[0]);
        moves += 1;
    }

    let final_visited = dungeon
        .rooms
        .iter()
        .flat_map(|row| row.iter())
        .filter_map(|r| r.as_ref())
        .filter(|r| r.is_visited)
        .count();

    assert!(
        final_visited > 1,
        "Should have visited more than entry room"
    );
}

#[test]
fn clearing_rooms_progresses_completion() {
    let mut dungeon = create_test_dungeon_with_rooms();

    // Initially: Rest room is cleared, Monster and Chest are not
    assert!(!dungeon.is_completed());

    // Clear Monster room
    dungeon.get_room_mut(0, 0).unwrap().clear();
    assert!(!dungeon.is_completed());

    // Clear Chest room
    dungeon.get_room_mut(0, 1).unwrap().clear();
    assert!(dungeon.is_completed());
}

#[test]
fn dungeon_navigation_full_walkthrough() {
    let mut dungeon = create_test_dungeon_with_rooms();
    dungeon.player_position = (0, 0);
    dungeon.mark_start_visited();

    // Move south to Chest room
    assert!(dungeon.move_player(Direction::South).is_ok());
    assert_eq!(dungeon.player_position, (0, 1));
    assert!(dungeon.current_room().unwrap().is_visited);

    // Take chest and clear room (avoid open_chest which needs game state)
    let current = dungeon.current_room_mut().unwrap();
    let _ = current.chest.take();
    current.clear();

    // Move south to Rest room
    assert!(dungeon.move_player(Direction::South).is_ok());
    assert_eq!(dungeon.player_position, (0, 2));

    // Rest room should already be cleared
    assert!(dungeon.current_room().unwrap().is_cleared);

    // Move back north
    assert!(dungeon.move_player(Direction::North).is_ok());
    assert_eq!(dungeon.player_position, (0, 1));

    // All rooms visited
    assert!(dungeon.get_room(0, 0).unwrap().is_visited);
    assert!(dungeon.get_room(0, 1).unwrap().is_visited);
    assert!(dungeon.get_room(0, 2).unwrap().is_visited);
}

#[test]
fn rest_room_healing_state_tracking() {
    let mut room = DungeonRoom::new(RoomType::Rest, 0, 0);

    assert!(!room.has_healed);

    room.has_healed = true;

    assert!(room.has_healed);
}

// ==================== Edge case tests ====================

#[test]
fn dungeon_boundary_checks_all_corners() {
    let dungeon = Dungeon::default();
    let max = DUNGEON_SIZE as i32 - 1;

    // Valid corner positions (but empty)
    assert!(dungeon.get_room(0, 0).is_none());
    assert!(dungeon.get_room(max, 0).is_none());
    assert!(dungeon.get_room(0, max).is_none());
    assert!(dungeon.get_room(max, max).is_none());

    // Just outside bounds
    assert!(dungeon.get_room(-1, 0).is_none());
    assert!(dungeon.get_room(0, -1).is_none());
    assert!(dungeon.get_room(max + 1, 0).is_none());
    assert!(dungeon.get_room(0, max + 1).is_none());
}

#[test]
fn multiple_generates_reset_dungeon() {
    let mut dungeon = Dungeon::default();

    dungeon.generate();
    let first_position = dungeon.player_position;
    let first_room_count = dungeon.room_count();

    // Generate again - should create new layout
    dungeon.generate();

    // Dungeon should still be valid
    assert!(dungeon.is_generated);
    assert!(dungeon.room_count() >= 5);

    // Position may or may not be the same (random)
    let _ = (first_position, first_room_count);
}

#[test]
fn dungeon_rooms_connected() {
    let mut dungeon = Dungeon::default();
    dungeon.generate();

    // Every room should have at least one neighbor (due to generation algorithm)
    let offsets = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    for row in &dungeon.rooms {
        for room_opt in row {
            if let Some(room) = room_opt {
                let has_neighbor = offsets.iter().any(|(dx, dy)| {
                    dungeon.get_room(room.x + dx, room.y + dy).is_some()
                });
                assert!(
                    has_neighbor,
                    "Room at ({}, {}) should have at least one neighbor",
                    room.x, room.y
                );
            }
        }
    }
}
