/// Economic value trait for items that can be bought/sold.
/// Used by shop system, blacksmith upgrades, and trading.
pub trait WorthGold {
    fn gold_value(&self) -> i32;
    fn purchase_price(&self) -> i32 {
        self.gold_value()
    }
    fn sell_price(&self) -> i32 {
        self.gold_value() / 2
    }
}
