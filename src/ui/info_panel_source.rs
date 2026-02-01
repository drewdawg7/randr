use crate::ui::screens::forge_modal::ForgeSlotIndex;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InfoPanelSource {
    Store { selected_index: usize },
    Inventory { selected_index: usize },
    Equipment { selected_index: usize },
    ForgeSlot { slot: ForgeSlotIndex },
    Recipe { selected_index: usize },
    None,
}
