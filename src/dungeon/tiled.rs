use super::layouts::LayoutId;

pub fn map_path(layout_id: LayoutId) -> &'static str {
    match layout_id {
        LayoutId::CaveFloor => "maps/cave_floor.tmx",
        LayoutId::HomeFloor => "maps/home_floor.tmx",
    }
}
