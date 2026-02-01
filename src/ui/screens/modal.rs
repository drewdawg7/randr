use bevy::prelude::*;

/// Resource tracking which modal is currently open, if any.
#[derive(Resource, Default)]
pub struct ActiveModal {
    pub modal: Option<ModalType>,
}

// ============================================================================
// Modal Lifecycle Events
// ============================================================================

/// Event to request opening a modal.
///
/// Use `commands.trigger(OpenModal(ModalType::Inventory))` to trigger.
/// The modal registry observers will handle the actual spawning.
#[derive(Message, Debug, Clone, Copy)]
pub struct OpenModal(pub ModalType);

/// Event to request closing a modal.
///
/// Use `commands.trigger(CloseModal(ModalType::Inventory))` to trigger.
/// The modal registry observers will handle the actual closing and cleanup.
#[derive(Message, Debug, Clone, Copy)]
pub struct CloseModal(pub ModalType);

/// Types of modals available in the game.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalType {
    Profile,
    Inventory,
    Keybinds,
    MonsterCompendium,
    FightModal,
    ResultsModal,
    MerchantModal,
    ForgeModal,
    AnvilModal,
    SkillsModal,
}

#[derive(Component, Default)]
pub struct ModalOverlay;

pub const MODAL_OVERLAY_COLOR: Color = Color::srgba(0.0, 0.0, 0.0, 0.7);
pub const MODAL_OVERLAY_Z_INDEX: i32 = 100;

#[derive(Bundle, Default)]
pub struct ModalOverlayBundle {
    pub marker: ModalOverlay,
    pub node: Node,
    pub background: BackgroundColor,
    pub z_index: ZIndex,
}

impl ModalOverlayBundle {
    pub fn new() -> Self {
        Self {
            marker: ModalOverlay,
            node: Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background: BackgroundColor(MODAL_OVERLAY_COLOR),
            z_index: ZIndex(MODAL_OVERLAY_Z_INDEX),
        }
    }
}

/// Component marker for modal content container.
#[derive(Component)]
pub struct ModalContent;

/// Plugin that manages modal state and lifecycle.
pub struct ModalPlugin;

impl Plugin for ModalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveModal>()
            .add_event::<OpenModal>()
            .add_event::<CloseModal>();
    }
}

// ============================================================================
// Modal Run Conditions
// ============================================================================
//
// Use these functions with `.run_if()` to conditionally run modal input systems.
// This is more efficient than checking modal state inside each handler.
//
// Example:
// ```
// app.add_systems(Update, (
//     handle_inventory_modal_tab,
//     handle_inventory_modal_navigation,
//     handle_inventory_modal_select,
// ).run_if(in_inventory_modal));
// ```

/// Run condition: returns true when the inventory modal is active.
pub fn in_inventory_modal(active_modal: Res<ActiveModal>) -> bool {
    active_modal.modal == Some(ModalType::Inventory)
}

/// Run condition: returns true when the merchant modal is active.
pub fn in_merchant_modal(active_modal: Res<ActiveModal>) -> bool {
    active_modal.modal == Some(ModalType::MerchantModal)
}

/// Run condition: returns true when the forge modal is active.
pub fn in_forge_modal(active_modal: Res<ActiveModal>) -> bool {
    active_modal.modal == Some(ModalType::ForgeModal)
}

/// Run condition: returns true when the anvil modal is active.
pub fn in_anvil_modal(active_modal: Res<ActiveModal>) -> bool {
    active_modal.modal == Some(ModalType::AnvilModal)
}

/// Run condition: returns true when the fight modal is active.
pub fn in_fight_modal(active_modal: Res<ActiveModal>) -> bool {
    active_modal.modal == Some(ModalType::FightModal)
}

/// Run condition: returns true when the results modal is active.
pub fn in_results_modal(active_modal: Res<ActiveModal>) -> bool {
    active_modal.modal == Some(ModalType::ResultsModal)
}

/// Run condition: returns true when the skills modal is active.
pub fn in_skills_modal(active_modal: Res<ActiveModal>) -> bool {
    active_modal.modal == Some(ModalType::SkillsModal)
}

/// Run condition: returns true when the monster compendium modal is active.
pub fn in_monster_compendium_modal(active_modal: Res<ActiveModal>) -> bool {
    active_modal.modal == Some(ModalType::MonsterCompendium)
}
