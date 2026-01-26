#[cfg(test)]
use crate::entities::progression::{Progression, HasProgression};

// ==================== Progression::new() tests ====================

#[test]
fn progression_new_starts_at_level_one() {
    let prog = Progression::new();
    assert_eq!(prog.level, 1);
}

#[test]
fn progression_new_starts_with_zero_xp() {
    let prog = Progression::new();
    assert_eq!(prog.xp, 0);
}

#[test]
fn progression_new_starts_with_zero_total_xp() {
    let prog = Progression::new();
    assert_eq!(prog.total_xp, 0);
}

// ==================== Progression::xp_to_next_level() tests ====================

#[test]
fn xp_to_next_level_at_level_one_is_fifty() {
    assert_eq!(Progression::xp_to_next_level(1), 50);
}

#[test]
fn xp_to_next_level_at_level_two_is_one_hundred() {
    assert_eq!(Progression::xp_to_next_level(2), 100);
}

#[test]
fn xp_to_next_level_scales_with_level() {
    // Formula is 50 * level
    assert_eq!(Progression::xp_to_next_level(1), 50);
    assert_eq!(Progression::xp_to_next_level(5), 250);
    assert_eq!(Progression::xp_to_next_level(10), 500);
    assert_eq!(Progression::xp_to_next_level(100), 5000);
}

// ==================== Progression::add_xp() tests ====================

#[test]
fn add_xp_increases_xp_without_level_up() {
    let mut prog = Progression::new();
    prog.add_xp(30);

    assert_eq!(prog.xp, 30);
    assert_eq!(prog.level, 1);
}

#[test]
fn add_xp_tracks_total_xp_correctly() {
    let mut prog = Progression::new();
    prog.add_xp(30);
    prog.add_xp(10);

    assert_eq!(prog.total_xp, 40);
}

#[test]
fn add_xp_triggers_level_up_when_threshold_reached() {
    let mut prog = Progression::new();
    prog.add_xp(50); // Exactly level 1 -> 2 threshold

    assert_eq!(prog.level, 2);
    assert_eq!(prog.xp, 0); // No excess XP
}

#[test]
fn add_xp_carries_over_excess_xp_after_level_up() {
    let mut prog = Progression::new();
    prog.add_xp(60); // 10 XP over threshold

    assert_eq!(prog.level, 2);
    assert_eq!(prog.xp, 10); // 60 - 50 = 10 carried over
}

#[test]
fn add_xp_returns_number_of_levels_gained() {
    let mut prog = Progression::new();

    // No level up
    let gained = prog.add_xp(30);
    assert_eq!(gained, 0);

    // One level up
    let gained = prog.add_xp(20); // Now at 50 XP total
    assert_eq!(gained, 1);
}

#[test]
fn add_xp_handles_multiple_level_ups_in_single_call() {
    let mut prog = Progression::new();
    // Level 1->2 = 50 XP, Level 2->3 = 100 XP
    // Total needed for level 3: 150 XP
    let gained = prog.add_xp(150);

    assert_eq!(gained, 2);
    assert_eq!(prog.level, 3);
    assert_eq!(prog.xp, 0); // Exactly enough, no excess
}

#[test]
fn add_xp_handles_multiple_level_ups_with_excess() {
    let mut prog = Progression::new();
    // Level 1->2 = 50 XP, Level 2->3 = 100 XP
    // Giving 175 XP: Level 3 with 25 XP remaining
    let gained = prog.add_xp(175);

    assert_eq!(gained, 2);
    assert_eq!(prog.level, 3);
    assert_eq!(prog.xp, 25); // 175 - 50 - 100 = 25
}

#[test]
fn add_xp_total_xp_tracks_across_level_ups() {
    let mut prog = Progression::new();
    prog.add_xp(50);  // Level up to 2
    prog.add_xp(100); // Level up to 3
    prog.add_xp(25);  // Partial progress

    assert_eq!(prog.total_xp, 175);
    assert_eq!(prog.level, 3);
    assert_eq!(prog.xp, 25);
}

// ==================== Edge cases ====================

#[test]
fn add_xp_exactly_at_threshold_no_overflow() {
    let mut prog = Progression::new();
    prog.add_xp(50); // Exactly threshold

    assert_eq!(prog.level, 2);
    assert_eq!(prog.xp, 0);
    assert_eq!(prog.total_xp, 50);
}

#[test]
fn xp_threshold_increases_at_each_level() {
    let mut prog = Progression::new();

    // Level 1 needs 50 XP
    prog.add_xp(50);
    assert_eq!(prog.level, 2);

    // Level 2 needs 100 XP
    prog.add_xp(99);
    assert_eq!(prog.level, 2); // Not enough
    prog.add_xp(1);
    assert_eq!(prog.level, 3);

    // Level 3 needs 150 XP
    prog.add_xp(149);
    assert_eq!(prog.level, 3); // Not enough
    prog.add_xp(1);
    assert_eq!(prog.level, 4);
}

#[test]
fn multiple_small_xp_gains_eventually_level_up() {
    let mut prog = Progression::new();

    for _ in 0..5 {
        prog.add_xp(10);
    }

    assert_eq!(prog.level, 2);
    assert_eq!(prog.xp, 0);
    assert_eq!(prog.total_xp, 50);
}

#[test]
fn large_xp_gain_handles_many_level_ups() {
    let mut prog = Progression::new();
    // XP needed: L1->2: 50, L2->3: 100, L3->4: 150, L4->5: 200
    // Total for level 5: 500 XP
    let gained = prog.add_xp(500);

    assert_eq!(gained, 4);
    assert_eq!(prog.level, 5);
    assert_eq!(prog.xp, 0);
}

// ==================== HasProgression trait tests ====================

#[cfg(test)]
struct MockProgressor {
    progression: Progression,
    level_up_count: i32,
}

#[cfg(test)]
impl MockProgressor {
    fn new() -> Self {
        Self {
            progression: Progression::new(),
            level_up_count: 0,
        }
    }
}

#[cfg(test)]
impl HasProgression for MockProgressor {
    fn progression(&self) -> &Progression {
        &self.progression
    }

    fn progression_mut(&mut self) -> &mut Progression {
        &mut self.progression
    }

    fn on_level_up(&mut self) {
        self.level_up_count += 1;
    }
}

#[test]
fn has_progression_level_returns_current_level() {
    let mock = MockProgressor::new();
    assert_eq!(mock.level(), 1);
}

#[test]
fn has_progression_level_updates_after_xp_gain() {
    let mut mock = MockProgressor::new();
    mock.gain_xp(50);
    assert_eq!(mock.level(), 2);
}

#[test]
fn has_progression_gain_xp_returns_levels_gained() {
    let mut mock = MockProgressor::new();

    let gained = mock.gain_xp(30);
    assert_eq!(gained, 0);

    let gained = mock.gain_xp(20);
    assert_eq!(gained, 1);
}

#[test]
fn has_progression_gain_xp_calls_on_level_up_once_per_level() {
    let mut mock = MockProgressor::new();
    mock.gain_xp(50); // One level up

    assert_eq!(mock.level_up_count, 1);
}

#[test]
fn has_progression_gain_xp_calls_on_level_up_multiple_times() {
    let mut mock = MockProgressor::new();
    mock.gain_xp(150); // Two level ups

    assert_eq!(mock.level_up_count, 2);
}

#[test]
fn has_progression_gain_xp_no_level_up_no_callback() {
    let mut mock = MockProgressor::new();
    mock.gain_xp(25); // Not enough to level

    assert_eq!(mock.level_up_count, 0);
}

// ==================== Integration tests ====================

#[test]
fn progression_gameplay_scenario() {
    // Simulate a typical gameplay session
    // XP curve: 10% increase per level (50 base)
    // Level 1->2: 50, Level 2->3: 55, Level 3->4: 61, Level 4->5: 67
    let mut mock = MockProgressor::new();

    // Kill a few weak mobs (10 XP each)
    mock.gain_xp(10);
    mock.gain_xp(10);
    mock.gain_xp(10);
    assert_eq!(mock.level(), 1);
    assert_eq!(mock.level_up_count, 0);

    // Kill stronger mob, level up (30 + 20 = 50, exactly enough)
    mock.gain_xp(20);
    assert_eq!(mock.level(), 2);
    assert_eq!(mock.level_up_count, 1);

    // Continue grinding - need 55 for next level
    mock.gain_xp(30);
    mock.gain_xp(25);
    assert_eq!(mock.level(), 3);
    assert_eq!(mock.level_up_count, 2);

    // Boss kill with big XP reward causes multiple level ups
    // Level 3->4: 61, Level 4->5: 67 = 128 total needed for 2 levels
    let levels_gained = mock.gain_xp(128);
    assert_eq!(levels_gained, 2);
    assert_eq!(mock.level(), 5);
    assert_eq!(mock.level_up_count, 4); // Total level ups in session
}
