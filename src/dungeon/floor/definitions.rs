//! Floor definitions using the entity_macros system.
//!
//! This file consolidates:
//! - FloorSpec struct definition
//! - FloorId enum
//! - All floor spec constants
//! - The spec() method on FloorId

use crate::dungeon::layouts::LayoutId;
use crate::dungeon::spawn::SpawnTable;
use crate::mob::MobId;

entity_macros::define_data! {
    spec FloorSpec {
        pub name: &'static str,
        pub layout_id: LayoutId,
        pub spawn_table: SpawnTable,
    }

    id FloorId;

    variants {
        GoblinCave1 {
            name: "Goblin Cave - Floor 1",
            layout_id: LayoutId::StartingRoom,
            spawn_table: SpawnTable::new()
                .mob(MobId::Goblin, 5)
                .mob(MobId::Slime, 3)
                .mob_count(3..=5)
                .guaranteed_mob(MobId::BlackDragon, 1)
                .npc(MobId::Merchant, 1..=1)
                .chest(1..=2)
                .stairs(1..=1)
                .rock(2..=4)
                .forge(1..=1),
        }
        HomeFloor {
            name: "Home",
            layout_id: LayoutId::HomeLayout,
            spawn_table: SpawnTable::empty(),
        }
    }
}
