# Question Templates by Category

Templates for AskUserQuestion calls during feature planning. Adapt based on codebase exploration findings.

## Layout/Position

```json
{
  "question": "Where should the [feature] appear?",
  "header": "Position",
  "options": [
    {"label": "Left panel", "description": "Next to existing [component]"},
    {"label": "Right panel", "description": "In the detail area"},
    {"label": "Bottom section", "description": "Below the main content"},
    {"label": "Modal/Overlay", "description": "As a popup dialog"}
  ]
}
```

## Display Format

```json
{
  "question": "How should [data] be displayed?",
  "header": "Format",
  "options": [
    {"label": "Grid", "description": "Visual tiles in rows/columns"},
    {"label": "List", "description": "Vertical list with details"},
    {"label": "Compact", "description": "Minimal, icon-only display"},
    {"label": "Detailed", "description": "Full information panel"}
  ]
}
```

## Data Display

```json
{
  "question": "What information should be shown for each [item]?",
  "header": "Data",
  "multiSelect": true,
  "options": [
    {"label": "Name", "description": "Item/entity name"},
    {"label": "Stats", "description": "Numerical values and properties"},
    {"label": "Icon", "description": "Visual representation"},
    {"label": "Description", "description": "Detailed text explanation"}
  ]
}
```

## Interaction

```json
{
  "question": "How should users interact with [feature]?",
  "header": "Interaction",
  "options": [
    {"label": "Click to select", "description": "Single click selects item"},
    {"label": "Click to activate", "description": "Single click triggers action"},
    {"label": "Hover preview", "description": "Show details on hover"},
    {"label": "Read-only", "description": "Display only, no interaction"}
  ]
}
```

## Navigation

```json
{
  "question": "What keyboard navigation is needed?",
  "header": "Navigation",
  "multiSelect": true,
  "options": [
    {"label": "Arrow keys", "description": "Move between items"},
    {"label": "Tab switching", "description": "Switch between panels"},
    {"label": "Enter to confirm", "description": "Confirm selection"},
    {"label": "Escape to close", "description": "Close/cancel action"}
  ]
}
```

## Empty/Edge States

```json
{
  "question": "What should happen when [condition]?",
  "header": "Edge case",
  "options": [
    {"label": "Show message", "description": "Display helpful text"},
    {"label": "Hide section", "description": "Don't show the area at all"},
    {"label": "Show placeholder", "description": "Gray/empty state indicator"},
    {"label": "Disable interaction", "description": "Show but prevent action"}
  ]
}
```

## Ordering/Sorting

```json
{
  "question": "How should [items] be ordered?",
  "header": "Sort order",
  "options": [
    {"label": "Alphabetical", "description": "A-Z by name"},
    {"label": "By type", "description": "Grouped by category"},
    {"label": "By value", "description": "Highest to lowest"},
    {"label": "By recency", "description": "Most recent first"}
  ]
}
```

## Styling

```json
{
  "question": "What visual style should [feature] use?",
  "header": "Style",
  "options": [
    {"label": "Match existing (Recommended)", "description": "Use same style as [similar component]"},
    {"label": "Highlighted", "description": "Stand out with accent colors"},
    {"label": "Subtle", "description": "Blend into background"},
    {"label": "Custom", "description": "New style for this feature"}
  ]
}
```

## Priority

```json
{
  "question": "What priority level for this feature?",
  "header": "Priority",
  "options": [
    {"label": "High", "description": "Needed soon, blocks other work"},
    {"label": "Medium", "description": "Important but not urgent"},
    {"label": "Low", "description": "Nice to have, can wait"},
    {"label": "Backlog", "description": "Future consideration"}
  ]
}
```

## Combining Questions

Group related questions in single AskUserQuestion calls (max 4):

**Example: Initial Design Questions**
```json
{
  "questions": [
    {"question": "Where should X appear?", "header": "Position", ...},
    {"question": "How should data be displayed?", "header": "Format", ...},
    {"question": "What info to show?", "header": "Data", "multiSelect": true, ...},
    {"question": "How to interact?", "header": "Interaction", ...}
  ]
}
```

**Example: Follow-up Details**
```json
{
  "questions": [
    {"question": "What keyboard nav?", "header": "Navigation", "multiSelect": true, ...},
    {"question": "Empty state behavior?", "header": "Edge case", ...},
    {"question": "Priority level?", "header": "Priority", ...}
  ]
}
```
