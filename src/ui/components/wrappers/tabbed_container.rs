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

/// A tab entry with a label and content component
pub struct TabEntry {
    pub label: Line<'static>,
    pub content: Box<dyn MockComponent>,
    pub content_height: Option<u16>,
}

impl TabEntry {
    pub fn new<C: MockComponent + 'static>(label: Line<'static>, content: C) -> Self {
        Self {
            label,
            content: Box::new(content),
            content_height: None,
        }
    }

    pub fn with_height<C: MockComponent + 'static>(
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

        // Calculate offsets for decorative borders
        let y_offset: u16 = if has_border { 1 } else { 0 };
        let x_offset: u16 = if has_border { 1 } else { 0 };

        let full_area = Rect {
            x: x_offset,
            y: y_offset,
            width: frame_size.width.saturating_sub(x_offset * 2), // Account for left and right
            height: frame_size.height.saturating_sub(y_offset),
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
                    label_spans.push(Span::styled(" ", Style::default()));
                }

                for span in tab.label.spans.iter() {
                    let styled_span = if is_selected {
                        // Keep the original foreground color, make bold
                        Span::styled(span.content.clone(), span.style.bold())
                    } else {
                        // Dim unselected tabs
                        Span::styled(span.content.clone(), Style::default().color(colors::DARK_GRAY))
                    };
                    label_spans.push(styled_span);
                }

                // Trailing space
                label_spans.push(Span::styled(" ", Style::default()));

                label_spans
            })
            .collect();

        let tabs_line = Line::from(tab_spans);
        frame.render_widget(Paragraph::new(tabs_line), chunks[0]);

        // Calculate tab bar width for the content box
        let tab_bar_width: usize = self.tabs
            .iter()
            .enumerate()
            .map(|(i, tab)| {
                let label_width: usize = tab.label.spans.iter()
                    .map(|s| s.content.chars().count())
                    .sum();
                // First tab: no leading space, just trailing space (1)
                // Other tabs: " | " (3) + leading space (1) + trailing space (1) = 5
                let extra = if i == 0 { 1 } else { 5 };
                label_width + extra
            })
            .sum();

        // Fill entire content area with BACKGROUND first
        let bg_fill = Block::default().style(Style::default().on_color(colors::BACKGROUND));
        frame.render_widget(bg_fill, chunks[1]);

        // Render active tab content inside a bordered box
        let box_width = (tab_bar_width as u16).min(full_area.width);
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            // Create content box with width matching tab bar and height from content_height
            let box_height = tab.content_height
                .map(|h| h + 2) // Add 2 for borders
                .unwrap_or(chunks[1].height)
                .min(chunks[1].height);
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
                let bottom_y = content_box_area.y + content_box_area.height;
                // Total border width includes left border char + content + right border char
                let total_border_width = box_width + 2;
                let border_area_top = Rect { x: 0, y: 0, width: total_border_width, height: 1 };
                let border_area_bottom = Rect { x: 0, y: bottom_y, width: total_border_width, height: 1 };

                // Calculate the number of rows for left/right borders (from tab bar to bottom border)
                let border_height = bottom_y - y_offset;

                if is_field_tab {
                    // Forest borders for Field tab
                    let top_border = forest_border::generate_top_border(total_border_width);
                    let bottom_border = forest_border::generate_bottom_border(total_border_width);
                    frame.render_widget(Paragraph::new(top_border), border_area_top);
                    frame.render_widget(Paragraph::new(bottom_border), border_area_bottom);

                    // Left and right borders
                    for row in 0..border_height {
                        let left_char = forest_border::generate_left_border_char(row);
                        let right_char = forest_border::generate_right_border_char(row);
                        let left_area = Rect { x: 0, y: y_offset + row, width: 1, height: 1 };
                        let right_area = Rect { x: x_offset + box_width, y: y_offset + row, width: 1, height: 1 };
                        frame.render_widget(Paragraph::new(Line::from(left_char)), left_area);
                        frame.render_widget(Paragraph::new(Line::from(right_char)), right_area);
                    }
                } else if is_blacksmith_tab {
                    // Ember borders for Blacksmith tab
                    let top_border = ember_border::generate_top_border(total_border_width);
                    let bottom_border = ember_border::generate_bottom_border(total_border_width);
                    frame.render_widget(Paragraph::new(top_border), border_area_top);
                    frame.render_widget(Paragraph::new(bottom_border), border_area_bottom);

                    // Left and right borders
                    for row in 0..border_height {
                        let left_char = ember_border::generate_left_border_char(row);
                        let right_char = ember_border::generate_right_border_char(row);
                        let left_area = Rect { x: 0, y: y_offset + row, width: 1, height: 1 };
                        let right_area = Rect { x: x_offset + box_width, y: y_offset + row, width: 1, height: 1 };
                        frame.render_widget(Paragraph::new(Line::from(left_char)), left_area);
                        frame.render_widget(Paragraph::new(Line::from(right_char)), right_area);
                    }
                } else if is_store_tab {
                    // Wood borders for Store tab
                    let top_border = wood_border::generate_top_border(total_border_width);
                    let bottom_border = wood_border::generate_bottom_border(total_border_width);
                    frame.render_widget(Paragraph::new(top_border), border_area_top);
                    frame.render_widget(Paragraph::new(bottom_border), border_area_bottom);

                    // Left and right borders
                    for row in 0..border_height {
                        let left_char = wood_border::generate_left_border_char(row);
                        let right_char = wood_border::generate_right_border_char(row);
                        let left_area = Rect { x: 0, y: y_offset + row, width: 1, height: 1 };
                        let right_area = Rect { x: x_offset + box_width, y: y_offset + row, width: 1, height: 1 };
                        frame.render_widget(Paragraph::new(Line::from(left_char)), left_area);
                        frame.render_widget(Paragraph::new(Line::from(right_char)), right_area);
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
            // Tab switching with Left/Right
            Event::Keyboard(KeyEvent { code: Key::Left, .. }) => {
                self.switch_tab(-1);
                None
            }
            Event::Keyboard(KeyEvent { code: Key::Right, .. }) => {
                self.switch_tab(1);
                None
            }
            // Forward Up/Down/Enter to active tab content
            Event::Keyboard(KeyEvent { code: Key::Up, .. })
            | Event::Keyboard(KeyEvent { code: Key::Down, .. })
            | Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                    // Convert event to command for MockComponent
                    let cmd = match ev {
                        Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                            Cmd::Move(tuirealm::command::Direction::Up)
                        }
                        Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                            Cmd::Move(tuirealm::command::Direction::Down)
                        }
                        Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => Cmd::Submit,
                        _ => return None,
                    };
                    tab.content.perform(cmd);
                }
                None
            }
            _ => None,
        }
    }
}
