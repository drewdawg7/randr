# Dragon ASCII Art

## Location
`src/ui/components/dungeon/dragon_art.rs`

## Purpose
Static dragon boss art for dungeon boss rooms. Large creature design showcasing multi-color techniques and fill patterns.

## Dimensions
- Width: 41 characters
- Height: 19 rows
- Constants: `DRAGON_WIDTH`, `DRAGON_HEIGHT`

## Structure (19 rows)
1. Wings spread - `\_____     _____/` pattern
2. Wing membranes - dashed lines with backticks
3-6. Upper body/wings - `}` `{` wing edges with `@@@@@@` fill
7. Face crown - decorative header with `^` accent
8. Eyes - `d` and `b` characters for eyes with `@@@` inner glow
9. Snout - with `~` fire accents
10-11. Mouth/neck - converging body with fire hints
12-13. Chest - scaling down with `";\"`  detail
14-15. Belly/lower body - fire breath `-==-` accents
16-17. Tail base - narrowing pattern
18. Tail tip - with fire trail
19. Feet - `(_(_(_)` claw patterns

## Fill Techniques
- **High-density fill**: `@@@@@@` for body mass (dark green)
- **Medium-density fill**: `######` for membrane texture (dark green)
- **Edge highlights**: `}` `{` `/` `\` for depth (lighter green)
- **Accents**: Fire symbols `-==-` interspersed for glowing effect

## Colors Used
| Variable | Color Constant | Purpose |
|----------|---------------|---------|
| `scale_dark` | `DARK_FOREST` | Deep body shadows, main fill |
| `scale` | `FOREST_GREEN` | Main body structure |
| `scale_light` | `LIME_GREEN` | Edge highlights |
| `scale_pale` | `PALE_GREEN` | Bright accents |
| `eye` | `EMBER_RED` | Glowing eyes (`d` `b`) |
| `fire` | `FLAME_ORANGE` | Fire breath accents |
| `fire_hot` | `BRIGHT_YELLOW` | Hot fire center |
| `claw` | `LIGHT_STONE` | Claws, horns |
| `teeth` | `WHITE` | Teeth |
| `inner` | `DEEP_ORANGE` | Inner glow (mouth area) |

## Depth Techniques
1. **Layered greens**: Dark -> Medium -> Light from inside to edges
2. **Fire contrast**: Orange/yellow fire against dark green scales
3. **Structural edges**: Light colored `}` `{` bracket chars suggest 3D form
4. **Inner glow**: Deep orange in mouth/chest area suggests internal heat

## Design Notes
- Dragon faces forward (symmetrical)
- Eyes use `d` and `b` for stylized dragon eyes with red glow
- Fire breath runs through center (`-==-` patterns)
- Feet at bottom ground the creature
- Wing spread at top for imposing presence

## Usage
```rust
use crate::ui::components::dungeon::dragon_art::{render_dragon_art, DRAGON_WIDTH, DRAGON_HEIGHT};

let lines: Vec<Line<'static>> = render_dragon_art();
// Render with Paragraph or direct buffer access
```

## Example Line Construction
```rust
// Row 8: Eyes - the most important row!
Line::from(vec![
    Span::styled("   ", scale),
    Span::styled("/ ", scale_light),
    Span::styled("{", scale),
    Span::styled("@@@@@@@", scale_dark),
    Span::styled("(", scale),
    Span::styled("d", eye),              // Left eye
    Span::styled("\\", teeth),
    Span::styled("@@@", inner),          // Inner glow
    Span::styled("/", teeth),
    Span::styled("b", eye),              // Right eye
    Span::styled(")", scale),
    Span::styled("@@@@@@@", scale_dark),
    Span::styled("} ", scale),
    Span::styled("\\", scale_light),
])
```
