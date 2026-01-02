use tuirealm::{command::{Cmd, CmdResult}, props::{AttrValue, Attribute, Props}, Component, Event, Frame, MockComponent, NoUserEvent, State};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, text::{Line, Span}};

use crate::ui::theme as colors;
use crate::ui::components::backgrounds::{render_stone_wall, render_decorative_border};

use crate::{combat::Named, system::game_state, ui::{utilities::{HOUSE, OPEN_DOOR, PERSON}, Id}};
use crate::ui::components::widgets::menu::{Menu, MenuItem};

pub struct MainMenuScreen {
    props: Props,
    menu: Menu,
}


impl Default for MainMenuScreen {
   fn default() -> Self {
        let items = vec![
            MenuItem {
                label: format!("{} Town", HOUSE).to_string(),
                action: Box::new(|| { game_state().current_screen = Id::Town; })
            },
            MenuItem {
                label: format!("{} Profile", PERSON).to_string(),
                action: Box::new(|| { game_state().current_screen = Id::Profile; })
            },
            MenuItem {
                label: format!("{} Quit", OPEN_DOOR).to_string(),
                action: Box::new(|| { game_state().current_screen = Id::Quit; })
            },
        ];
        Self {
            props: Props::default(),
            menu: Menu::new(items),
        }
   }
}

// Menu dimensions for centering
const MENU_HEIGHT: u16 = 5; // greeting line + 3 menu items + padding
const MENU_WIDTH: u16 = 20;
const BORDER_WIDTH: u16 = 7; // Width of border patterns on each side

impl MockComponent for MainMenuScreen {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Render stone wall background first
        render_stone_wall(frame, area);

        // Render decorative border on top
        render_decorative_border(frame, area);

        // Calculate inner area (inside the border)
        let inner_area = Rect {
            x: area.x + BORDER_WIDTH,
            y: area.y + 2, // Skip top corner and horizontal bar rows
            width: area.width.saturating_sub(BORDER_WIDTH * 2),
            height: area.height.saturating_sub(4), // Skip 2 rows top and 2 rows bottom
        };

        // Center the menu content within the inner area
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(2),           // Top space (smaller)
                Constraint::Length(MENU_HEIGHT),
                Constraint::Fill(3),           // Bottom space (larger)
            ])
            .split(inner_area);

        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(MENU_WIDTH),
                Constraint::Fill(1),
            ])
            .split(vertical_chunks[1]);

        let centered_area = horizontal_chunks[1];

        // Render player greeting and menu
        let player_name = self.props
            .get(Attribute::Title)
            .map(|v| v.unwrap_string())
            .unwrap_or_else(|| game_state().player.name().to_string());

        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(0)])
            .split(centered_area);

        // Greeting with explicit styling for visibility over background
        let text_style = Style::default().fg(colors::WHITE);
        let name_style = Style::default().bold().fg(colors::GREEN).underlined();
        let line = Line::from(vec![
            Span::styled("Hello, ", text_style),
            Span::styled(player_name, name_style)
        ]);

        // Render greeting directly to buffer to preserve background
        render_line_to_buffer(frame, content_chunks[0], line);

        self.menu.view(frame, content_chunks[1]);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr).or_else(|| self.menu.query(attr))
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        match attr {
            Attribute::Title => self.props.set(attr, value),
            _ => self.menu.attr(attr, value),
        }
    }

    fn state(&self) -> State {
        self.menu.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.menu.perform(cmd)
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for MainMenuScreen {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        self.menu.on(ev)
    }
}

/// Renders a Line directly to the buffer, skipping spaces to preserve background
fn render_line_to_buffer(frame: &mut Frame, area: Rect, line: Line) {
    let buf = frame.buffer_mut();
    let y = area.y;
    let mut x = area.x;

    for span in line.spans.iter() {
        let has_style = span.style.fg.is_some() || span.style.bg.is_some();
        for ch in span.content.chars() {
            // Skip spaces in unstyled spans to preserve background
            if ch == ' ' && !has_style {
                x += 1;
                continue;
            }
            if x < area.x + area.width {
                if let Some(cell) = buf.cell_mut((x, y)) {
                    cell.set_char(ch);
                    if let Some(fg) = span.style.fg {
                        cell.set_fg(fg);
                    }
                    if span.style.add_modifier.contains(ratatui::style::Modifier::BOLD) {
                        cell.set_style(cell.style().add_modifier(ratatui::style::Modifier::BOLD));
                    }
                    if span.style.add_modifier.contains(ratatui::style::Modifier::UNDERLINED) {
                        cell.set_style(cell.style().add_modifier(ratatui::style::Modifier::UNDERLINED));
                    }
                }
            }
            x += 1;
        }
    }
}
