//! Test module for entity_macros
//! This file tests the define_entity! macro with actual game types

use crate::stats::{StatSheet, StatType};

// Define a simple test entity to verify the macro works
entity_macros::define_entity! {
    spec TestItemSpec {
        pub name: &'static str,
        pub gold_value: i32,
        pub stats: StatSheet,
    }

    id TestItemId;

    variants {
        TestSword {
            name: "Test Sword",
            gold_value: 15,
            stats: StatSheet::new().with(StatType::Attack, 10),
        }
        TestShield {
            name: "Test Shield",
            gold_value: 20,
            stats: StatSheet::new().with(StatType::Defense, 5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_lookup() {
        let sword_spec = TestItemId::TestSword.spec();
        assert_eq!(sword_spec.name, "Test Sword");
        assert_eq!(sword_spec.gold_value, 15);
    }

    #[test]
    fn test_all_variants() {
        assert_eq!(TestItemId::ALL.len(), 2);
        assert!(TestItemId::ALL.contains(&TestItemId::TestSword));
        assert!(TestItemId::ALL.contains(&TestItemId::TestShield));
    }

    #[test]
    fn test_stats() {
        let sword_spec = TestItemId::TestSword.spec();
        assert_eq!(sword_spec.stats.value(StatType::Attack), 10);
    }
}
