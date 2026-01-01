use crate::blacksmith::Blacksmith;

impl Default for Blacksmith {
    fn default() -> Self {
        Self {
            name: "Blacksmith".to_string(),
            max_upgrades: 4,
            base_upgrade_cost: 5,
            fuel_amount: 0
        }
    }
}
