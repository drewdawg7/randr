#[derive(Debug)]
pub enum StoreError {
    OutOfStock,
    NotEnoughGold,
    InventoryFull,
    InvalidIndex,
}
