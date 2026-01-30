#[cfg(test)]
use crate::entities::progression::Progression;

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
fn xp_to_next_level_at_level_two_is_fifty_five() {
    assert_eq!(Progression::xp_to_next_level(2), 55);
}

#[test]
fn xp_to_next_level_scales_exponentially() {
    assert_eq!(Progression::xp_to_next_level(1), 50);
    assert_eq!(Progression::xp_to_next_level(5), 73);
    assert_eq!(Progression::xp_to_next_level(10), 118);
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
    prog.add_xp(50);

    assert_eq!(prog.level, 2);
    assert_eq!(prog.xp, 0);
}

#[test]
fn add_xp_carries_over_excess_xp_after_level_up() {
    let mut prog = Progression::new();
    prog.add_xp(60);

    assert_eq!(prog.level, 2);
    assert_eq!(prog.xp, 10);
}

#[test]
fn add_xp_returns_number_of_levels_gained() {
    let mut prog = Progression::new();

    let gained = prog.add_xp(30);
    assert_eq!(gained, 0);

    let gained = prog.add_xp(20);
    assert_eq!(gained, 1);
}

#[test]
fn add_xp_handles_multiple_level_ups_in_single_call() {
    let mut prog = Progression::new();
    let gained = prog.add_xp(105);

    assert_eq!(gained, 2);
    assert_eq!(prog.level, 3);
    assert_eq!(prog.xp, 0);
}

#[test]
fn add_xp_handles_multiple_level_ups_with_excess() {
    let mut prog = Progression::new();
    let gained = prog.add_xp(130);

    assert_eq!(gained, 2);
    assert_eq!(prog.level, 3);
    assert_eq!(prog.xp, 25);
}

#[test]
fn add_xp_total_xp_tracks_across_level_ups() {
    let mut prog = Progression::new();
    prog.add_xp(50);
    prog.add_xp(55);
    prog.add_xp(25);

    assert_eq!(prog.total_xp, 130);
    assert_eq!(prog.level, 3);
    assert_eq!(prog.xp, 25);
}

// ==================== Edge cases ====================

#[test]
fn add_xp_exactly_at_threshold_no_overflow() {
    let mut prog = Progression::new();
    prog.add_xp(50);

    assert_eq!(prog.level, 2);
    assert_eq!(prog.xp, 0);
    assert_eq!(prog.total_xp, 50);
}

#[test]
fn xp_threshold_increases_at_each_level() {
    let mut prog = Progression::new();

    prog.add_xp(50);
    assert_eq!(prog.level, 2);

    prog.add_xp(54);
    assert_eq!(prog.level, 2);
    prog.add_xp(1);
    assert_eq!(prog.level, 3);

    prog.add_xp(60);
    assert_eq!(prog.level, 3);
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
    let gained = prog.add_xp(233);

    assert_eq!(gained, 4);
    assert_eq!(prog.level, 5);
    assert_eq!(prog.xp, 0);
}
