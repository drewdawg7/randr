use super::enums::{LocationId, LocationType};

pub trait Location {
    fn id(&self) -> LocationId;
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    fn location_type(&self) -> LocationType {
        self.id().location_type()
    }
}
