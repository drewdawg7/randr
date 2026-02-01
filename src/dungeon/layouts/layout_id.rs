#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutId {
    CaveFloor,
    HomeFloor,
}

impl LayoutId {
    pub const ALL: &'static [LayoutId] = &[LayoutId::CaveFloor, LayoutId::HomeFloor];

    pub fn dimensions(&self) -> (usize, usize) {
        match self {
            LayoutId::CaveFloor => (15, 11),
            LayoutId::HomeFloor => (10, 10),
        }
    }
}
