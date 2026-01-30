use bevy::prelude::*;

use crate::assets::GameFonts;
use crate::item::Item;
use crate::stats::StatType;

use super::{ItemStatsDisplay, OutlinedText};

pub struct ItemDetailDisplayPlugin;

impl Plugin for ItemDetailDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_item_detail_display);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PriceDisplay {
    Buy(i32),
    Sell(i32),
}

impl PriceDisplay {
    fn label(&self) -> String {
        match self {
            PriceDisplay::Buy(price) => format!("Price: {}g", price),
            PriceDisplay::Sell(price) => format!("Sell: {}g", price),
        }
    }
}

#[derive(Component)]
pub struct ItemDetailDisplay {
    name: String,
    item_type: String,
    quality_name: String,
    quality_color: Color,
    quantity: u32,
    stats: Vec<(StatType, i32)>,
    comparison: Option<Vec<(StatType, i32)>>,
    price: Option<PriceDisplay>,
}

impl ItemDetailDisplay {
    pub fn new(item: &Item) -> Self {
        let stats = item
            .stats
            .stats()
            .iter()
            .map(|(t, si)| (*t, si.current_value))
            .collect();

        Self {
            name: item.name.clone(),
            item_type: format!("{}", item.item_type),
            quality_name: item.quality.display_name().to_string(),
            quality_color: item.quality.color(),
            quantity: 1,
            stats,
            comparison: None,
            price: None,
        }
    }

    pub fn with_quantity(mut self, quantity: u32) -> Self {
        self.quantity = quantity;
        self
    }

    pub fn with_price(mut self, price: PriceDisplay) -> Self {
        self.price = Some(price);
        self
    }

    pub fn with_comparison(mut self, comparison: Vec<(StatType, i32)>) -> Self {
        self.comparison = Some(comparison);
        self
    }
}

fn on_add_item_detail_display(
    trigger: Trigger<OnAdd, ItemDetailDisplay>,
    mut commands: Commands,
    query: Query<&ItemDetailDisplay>,
    game_fonts: Res<GameFonts>,
) {
    let entity = trigger.entity();
    let Ok(display) = query.get(entity) else {
        return;
    };

    let name = display.name.clone();
    let item_type = display.item_type.clone();
    let quality_name = display.quality_name.clone();
    let quality_color = display.quality_color;
    let quantity = display.quantity;
    let stats = display.stats.clone();
    let comparison = display.comparison.clone();
    let price = display.price;

    commands
        .entity(entity)
        .remove::<ItemDetailDisplay>()
        .insert(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::FlexStart,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                OutlinedText::new(&name)
                    .with_font_size(16.0)
                    .with_color(quality_color),
            );

            parent.spawn((
                Text::new(&item_type),
                game_fonts.pixel_font(14.0),
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));

            parent.spawn((
                Text::new(&quality_name),
                game_fonts.pixel_font(14.0),
                TextColor(quality_color),
            ));

            if quantity > 1 {
                parent.spawn((
                    Text::new(format!("Qty: {}", quantity)),
                    game_fonts.pixel_font(14.0),
                    TextColor(Color::srgb(0.3, 0.8, 0.3)),
                ));
            }

            if let Some(price_display) = price {
                parent.spawn((
                    Text::new(price_display.label()),
                    game_fonts.pixel_font(14.0),
                    TextColor(Color::srgb(0.9, 0.8, 0.2)),
                ));
            }

            if !stats.is_empty() {
                let mut stats_display = ItemStatsDisplay::from_stats_iter(stats)
                    .with_font_size(14.0)
                    .with_color(Color::srgb(0.85, 0.85, 0.85));

                if let Some(comp) = comparison {
                    stats_display = stats_display.with_comparison(comp);
                }

                parent.spawn(stats_display);
            }
        });
}
