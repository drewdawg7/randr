use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::ui::theme::{self as colors, ColorExt};
use crate::ui::components::widgets::{forest_border, ember_border, wood_border};
use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State, StateValue,
};

/// Trait for tab content that can handle both rendering and events
pub trait TabContent: MockComponent + Component<Event<NoUserEvent>, NoUserEvent> {}

/// Blanket implementation: any type implementing both traits gets TabContent for free
impl<T: MockComponent + Component<Event<NoUserEvent>, NoUserEvent>> TabContent for T {}

/// A tab entry with a label and content component
pub struct TabEntry {
    pub label: Line<'static>,
    pub content: Box<dyn TabContent>,
    pub content_height: Option<u16>,
}

impl TabEntry {
    pub fn new<C: TabContent + 'static>(label: Line<'static>, content: C) -> Self {
        Self {
            label,
            content: Box::new(content),
            content_height: None,
        }
    }

    pub fn with_height<C: TabContent + 'static>(
        label: Line<'static>,
        content: C,
        content_height: u16,
    ) -> Self {
        Self {
            label,
            content: Box::new(content),
            content_height: Some(content_height),
        }
    }
}

/// A reusable tabbed container component
pub struct TabbedContainer {
    props: Props,
    tabs: Vec<TabEntry>,
    active_tab: usize,
}

impl TabbedContainer {
    pub fn new(tabs: Vec<TabEntry>) -> Self {
        Self {
            props: Props::default(),
            tabs,
            active_tab: 0,
        }
    }

    fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    fn switch_tab(&mut self, delta: i32) {
        let count = self.tab_count();
        if count == 0 {
            return;
        }
        let new_idx = (self.active_tab as i32 + delta).rem_euclid(count as i32) as usize;
        self.active_tab = new_idx;
    }

    /// Check if the active tab is a Field tab (by checking if label contains "Field")
    fn is_field_tab_active(&self) -> bool {
        if let Some(tab) = self.tabs.get(self.active_tab) {
            tab.label.spans.iter().any(|span| span.content.contains("Field"))
        } else {
            false
        }
    }

    /// Check if the active tab is a Blacksmith tab (by checking if label contains "Blacksmith")
    fn is_blacksmith_tab_active(&self) -> bool {
        if let Some(tab) = self.tabs.get(self.active_tab) {
            tab.label.spans.iter().any(|span| span.content.contains("Blacksmith"))
        } else {
            false
        }
    }

    /// Check if the active tab is a Store tab (by checking if label contains "Store")
    fn is_store_tab_active(&self) -> bool {
        if let Some(tab) = self.tabs.get(self.active_tab) {
            tab.label.spans.iter().any(|span| span.content.contains("Store"))
        } else {
            false
        }
    }

    /// Check if the active tab needs a decorative border
    fn needs_border(&self) -> bool {
        self.is_field_tab_active() || self.is_blacksmith_tab_active() || self.is_store_tab_active()
    }
}

impl MockComponent for TabbedContainer {
    fn view(&mut self, frame: &mut Frame, _area: Rect) {
        // Use the actual frame size for absolute positioning
        let frame_size = frame.area();
        let is_field_tab = self.is_field_tab_active();
        let is_blacksmith_tab = self.is_blacksmith_tab_active();
        let is_store_tab = self.is_store_tab_active();
        let has_border = self.needs_border();

        // Calculate offsets for decorative borders (1 row/col each for top, bottom, left, right)
        let y_offset: u16 = if has_border { 1 } else { 0 };
        let x_offset: u16 = if has_border { 1 } else { 0 };
        // When we have borders, reserve space for both top (y_offset) and bottom (1 more)
        let y_reserve: u16 = if has_border { 2 } else { 0 };

        let full_area = Rect {
            x: x_offset,
            y: y_offset,
            width: frame_size.width.saturating_sub(x_offset * 2), // Account for left and right
            height: frame_size.height.saturating_sub(y_reserve),  // Account for top and bottom
        };

        // Layout: [Tabs (height 1)] [Content (flex)]
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Tab bar
                Constraint::Min(0),    // Content area
            ])
            .split(full_area);

        // Render tab bar with styled spans
        let tab_spans: Vec<Span> = self.tabs
            .iter()
            .enumerate()
            .flat_map(|(i, tab)| {
                let is_selected = i == self.active_tab;

                // Build the label spans with proper styling
                let mut label_spans: Vec<Span> = Vec::new();

                // Add separator before non-first tabs
                if i == 0 {
                    // No leading space for first tab - flush with left edge
                } else {
                    label_spans.push(Span::styled(" | ", Style::default().color(colors::DARK_GRAY)));
                }

                // Active tab gets background highlight
                let bg_style = if is_selected {
                    Style::default().bg(colors::HEADER_BG)
                } else {
                    Style::default()
                };

                for span in tab.label.spans.iter() {
                    let styled_span = if is_selected {
                        // Keep original foreground color, add background, make bold
                        Span::styled(span.content.clone(), span.style.bg(colors::HEADER_BG).bold())
                    } else {
                        // Normal styling for unselected tabs
                        Span::styled(span.content.clone(), span.style)
                    };
                    label_spans.push(styled_span);
                }

                // Trailing space with same background as the tab
                label_spans.push(Span::styled(" ", bg_style));

                label_spans
            })
            .collect();

        let tabs_line = Line::from(tab_spans);
        frame.render_widget(Paragraph::new(tabs_line), chunks[0]);

        // Use full available width for content area (consistent with fight screen)
        let box_width = full_area.width;

        // Determine themed background based on active tab
        let tab_bg = if is_store_tab {
            colors::STORE_BG
        } else if is_blacksmith_tab {
            colors::BLACKSMITH_BG
        } else if is_field_tab {
            colors::FIELD_BG
        } else {
            colors::BACKGROUND
        };

        // Fill background ONLY inside the bordered area (not full chunks[1])
        let bordered_content = Rect {
            x: x_offset,
            y: chunks[1].y,
            width: box_width,
            height: chunks[1].height,
        };
        let bg_fill = Block::default().style(Style::default().on_color(tab_bg));
        frame.render_widget(bg_fill, bordered_content);

        // Render active tab content - use full available height (no capping)
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            // Use full available height for content
            let box_height = chunks[1].height;
            let content_box_area = Rect {
                x: x_offset,
                y: y_offset + 1, // Right below tab bar (accounting for offset)
                width: box_width,
                height: box_height,
            };

            // Render content directly in the content box area
            tab.content.view(frame, content_box_area);

            // Render decorative borders based on active tab
            if has_border {
                // Bottom border at the last row of the frame
                let bottom_y = frame_size.height.saturating_sub(1);
                // Total border width includes left border char + content + right border char
                let total_border_width = box_width + 2;
                let border_area_top = Rect { x: 0, y: 0, width: total_border_width, height: 1 };
                let border_area_bottom = Rect { x: 0, y: bottom_y, width: total_border_width, height: 1 };

                // Left/right borders span from y_offset to bottom_y (exclusive)
                let border_height = bottom_y.saturating_sub(y_offset);

                // Border style with themed background
                let border_style = Style::default().bg(tab_bg);

                if is_field_tab {
                    // Forest borders for Field tab
                    let top_border = forest_border::generate_top_border(total_border_width);
                    let bottom_border = forest_border::generate_bottom_border(total_border_width);
                    frame.render_widget(Paragraph::new(top_border).style(border_style), border_area_top);
                    frame.render_widget(Paragraph::new(bottom_border).style(border_style), border_area_bottom);

                    // Left and right borders
                    for row in 0..border_height {
                        let left_char = forest_border::generate_left_border_char(row);
                        let right_char = forest_border::generate_right_border_char(row);
                        let left_area = Rect { x: 0, y: y_offset + row, width: 1, height: 1 };
                        let right_area = Rect { x: x_offset + box_width, y: y_offset + row, width: 1, height: 1 };
                        frame.render_widget(Paragraph::new(Line::from(left_char)).style(border_style), left_area);
                        frame.render_widget(Paragraph::new(Line::from(right_char)).style(border_style), right_area);
                    }
                } else if is_blacksmith_tab {
                    // Ember borders for Blacksmith tab
                    let top_border = ember_border::generate_top_border(total_border_width);
                    let bottom_border = ember_border::generate_bottom_border(total_border_width);
                    frame.render_widget(Paragraph::new(top_border).style(border_style), border_area_top);
                    frame.render_widget(Paragraph::new(bottom_border).style(border_style), border_area_bottom);

                    // Left and right borders
                    for row in 0..border_height {
                        let left_char = ember_border::generate_left_border_char(row);
                        let right_char = ember_border::generate_right_border_char(row);
                        let left_area = Rect { x: 0, y: y_offset + row, width: 1, height: 1 };
                        let right_area = Rect { x: x_offset + box_width, y: y_offset + row, width: 1, height: 1 };
                        frame.render_widget(Paragraph::new(Line::from(left_char)).style(border_style), left_area);
                        frame.render_widget(Paragraph::new(Line::from(right_char)).style(border_style), right_area);
                    }
                } else if is_store_tab {
                    // Wood borders for Store tab
                    let top_border = wood_border::generate_top_border(total_border_width);
                    let bottom_border = wood_border::generate_bottom_border(total_border_width);
                    frame.render_widget(Paragraph::new(top_border).style(border_style), border_area_top);
                    frame.render_widget(Paragraph::new(bottom_border).style(border_style), border_area_bottom);

                    // Left and right borders
                    for row in 0..border_height {
                        let left_char = wood_border::generate_left_border_char(row);
                        let right_char = wood_border::generate_right_border_char(row);
                        let left_area = Rect { x: 0, y: y_offset + row, width: 1, height: 1 };
                        let right_area = Rect { x: x_offset + box_width, y: y_offset + row, width: 1, height: 1 };
                        frame.render_widget(Paragraph::new(Line::from(left_char)).style(border_style), left_area);
                        frame.render_widget(Paragraph::new(Line::from(right_char)).style(border_style), right_area);
                    }
                }
            }
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.active_tab))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        // Forward commands to active tab content
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            tab.content.perform(cmd)
        } else {
            CmdResult::None
        }
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for TabbedContainer {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        match ev {
            // Tab switching with Left/Right - TabbedContainer handles these
            Event::Keyboard(KeyEvent { code: Key::Left, .. }) => {
                self.switch_tab(-1);
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Right, .. }) => {
                self.switch_tab(1);
                None
            }
            // Forward ALL other events to the active tab
            _ => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                    tab.content.on(ev)
                } else {
                    None
                }
            }
        }
    }
}
