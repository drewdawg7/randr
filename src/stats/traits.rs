use crate::stats::{StatSheet};



pub trait HasStats {
    fn get_stat_sheet(&self) -> &StatSheet;
    fn get_stat_sheet_mut(&mut self) -> &mut StatSheet;
}
