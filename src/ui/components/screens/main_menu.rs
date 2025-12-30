use tuirealm::{command::{Cmd, CmdResult}, props::{AttrValue, Attribute, Props}, Component, Event, Frame, MockComponent, NoUserEvent, State};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, text::{Line, Span}, widgets::Paragraph};

use crate::ui::theme::{self as colors, ColorExt};

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

impl MockComponent for MainMenuScreen {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let player_name = self.props
            .get(Attribute::Title)
            .map(|v| v.unwrap_string())
            .unwrap_or_else(|| game_state().player.name().to_string());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(area);

        let style = Style::default().bold().color(colors::GREEN).underlined();
        let line = Line::from(vec![
            Span::raw("Hello, "),
            Span::styled(player_name, style)
        ]);
        frame.render_widget(Paragraph::new(line), chunks[0]);
        self.menu.view(frame, chunks[1]);
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
