use crate::{blacksmith::BlacksmithError, combat::HasGold, entities::Player, item::Item};



pub struct Blacksmith {
    pub name: String,
    pub max_upgrades: i32,
    pub base_upgrade_cost: i32,
}


impl Blacksmith {
    pub fn new(name: String, max_upgrades: i32, base_upgrade_cost: i32) -> Self {
        Self {
            name,
            max_upgrades,
            base_upgrade_cost
        }
    }
    pub fn upgrade_item(&self, player: &mut Player, item: &mut Item) -> Result<(), BlacksmithError>{
        if item.num_upgrades >= self.max_upgrades {
            return Err(BlacksmithError::MaxUpgradesReached)
        }
        let upgrade_cost = self.calc_upgrade_cost(item);
        if upgrade_cost > player.gold {
            return Err(BlacksmithError::NotEnoughGold)
        }
        player.dec_gold(upgrade_cost);
        match item.upgrade() {
            Ok(_) => Ok(()),
            Err(e) => Err(BlacksmithError::ItemError(e))
        }
    }
    pub fn calc_upgrade_cost(&self, item: &Item) -> i32 {
        let multiplier = item.quality.upgrade_cost_multiplier();
        let base = (item.num_upgrades + 1) * self.base_upgrade_cost;
        ((base as f64) * multiplier).round() as i32
    }
}
