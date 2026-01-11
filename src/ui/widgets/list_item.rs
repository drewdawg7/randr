//! Generic selectable list item component.

use std::marker::PhantomData;

use bevy::prelude::*;

use crate::ui::Selectable;

/// Generic marker component for selectable list items.
///
/// Use this with a marker type to distinguish between different lists:
/// - `SelectableListItem<StoreMarker>` for store items
/// - `SelectableListItem<BlacksmithMarker>` for blacksmith items
/// - `SelectableListItem<AlchemistMarker>` for alchemist items
#[derive(Component)]
pub struct SelectableListItem<T: Send + Sync + 'static> {
    pub index: usize,
    pub name: String,
    _marker: PhantomData<T>,
}

impl<T: Send + Sync + 'static> SelectableListItem<T> {
    pub fn new(index: usize, name: impl Into<String>) -> Self {
        Self {
            index,
            name: name.into(),
            _marker: PhantomData,
        }
    }
}

impl<T: Send + Sync + 'static> Selectable for SelectableListItem<T> {
    fn index(&self) -> usize {
        self.index
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Marker for store list items.
#[derive(Debug, Clone, Copy)]
pub struct StoreMarker;

/// Marker for blacksmith list items.
#[derive(Debug, Clone, Copy)]
pub struct BlacksmithMarker;

/// Marker for alchemist recipe list items.
#[derive(Debug, Clone, Copy)]
pub struct AlchemistMarker;

/// Type alias for store list items.
pub type StoreListItem = SelectableListItem<StoreMarker>;

/// Type alias for blacksmith list items.
pub type BlacksmithListItem = SelectableListItem<BlacksmithMarker>;

/// Type alias for alchemist recipe list items.
pub type AlchemistRecipeItem = SelectableListItem<AlchemistMarker>;
