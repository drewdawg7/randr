# UI Widgets

Reusable widget components in `src/ui/widgets/`.

## Index

| When using... | Read |
|---------------|------|
| StatRow (label + value rows) | [stat_row.md](stat_row.md) |
| IconValueRow (icon + value) | [icon_value_row.md](icon_value_row.md) |
| ItemStatsDisplay (stat lists) | [item_stats_display.md](item_stats_display.md) |
| GoldDisplay (gold with coin icon) | [gold_display.md](gold_display.md) |
| ItemGrid (5x5 item grid) | [item_grid.md](item_grid.md) |
| CentralDetailPanel (item details) | [central_detail_panel.md](central_detail_panel.md) |
| spawn_nine_slice_panel (nine-slice backgrounds) | [nine_slice.md](nine_slice.md) |
| spawn_three_slice_banner (3-slice horizontal banners) | [three_slice.md](three_slice.md) |

## Adding New Widgets

1. Create file in `src/ui/widgets/`
2. Define component struct and plugin
3. Use observer pattern for building UI on component add
4. Export from `src/ui/widgets/mod.rs`
5. Register plugin in `src/plugins/game.rs`
