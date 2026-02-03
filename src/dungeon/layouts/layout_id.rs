#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutId {
    CaveFloor,
    HomeFloor,
}

impl LayoutId {
    pub const ALL: &'static [LayoutId] = &[LayoutId::CaveFloor, LayoutId::HomeFloor];
}
