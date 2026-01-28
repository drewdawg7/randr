use super::floor_type::FloorType;
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
        HomeFloor {
            name: "Home",
            layout_id: LayoutId::HomeFloor,
            spawn_table: SpawnTable::new().npc(MobId::Merchant, 1..=1),
        }
        MainDungeon1 {
            name: "Dungeon - Floor 1",
            layout_id: LayoutId::CaveFloor,
            spawn_table: SpawnTable::new()
                .mob(MobId::Goblin, 5)
                .mob(MobId::Slime, 3)
                .mob_count(3..=4)
                .guaranteed_mob(MobId::DwarfDefender, 1)
                .guaranteed_mob(MobId::DwarfWarrior, 1)
                .guaranteed_mob(MobId::DwarfMiner, 1)
                .guaranteed_mob(MobId::DwarfKing, 1)
                .rock(0..=4)
                .forge_chance(0.33)
                .anvil_chance(0.33)
                .npc_chance(MobId::Merchant, 0.33)
                .stairs(1..=1),
        }
        MainDungeon2 {
            name: "Dungeon - Floor 2",
            layout_id: LayoutId::CaveFloor,
            spawn_table: SpawnTable::new()
                .mob(MobId::Goblin, 5)
                .mob(MobId::Slime, 3)
                .mob_count(3..=4)
                .guaranteed_mob(MobId::DwarfDefender, 1)
                .guaranteed_mob(MobId::DwarfWarrior, 1)
                .guaranteed_mob(MobId::DwarfMiner, 1)
                .guaranteed_mob(MobId::DwarfKing, 1)
                .rock(0..=4)
                .forge_chance(0.33)
                .anvil_chance(0.33)
                .npc_chance(MobId::Merchant, 0.33)
                .stairs(1..=1),
        }
        MainDungeon3 {
            name: "Dungeon - Floor 3",
            layout_id: LayoutId::CaveFloor,
            spawn_table: SpawnTable::new()
                .mob(MobId::Goblin, 5)
                .mob(MobId::Slime, 3)
                .mob_count(3..=4)
                .guaranteed_mob(MobId::DwarfDefender, 1)
                .guaranteed_mob(MobId::DwarfWarrior, 1)
                .guaranteed_mob(MobId::DwarfMiner, 1)
                .guaranteed_mob(MobId::DwarfKing, 1)
                .rock(0..=4)
                .forge_chance(0.33)
                .anvil_chance(0.33)
                .npc_chance(MobId::Merchant, 0.33),
        }
    }
}

impl FloorId {
    pub fn floor_type(&self) -> FloorType {
        FloorType::CaveFloor
    }
}
