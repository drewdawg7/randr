use tuirealm::{command::{Cmd, CmdResult}, props::{AttrValue, Attribute}, Component, Event, Frame, MockComponent, NoUserEvent, State};
use tuirealm::event::{Key, KeyEvent};
use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, text::{Line, Span}, widgets::Paragraph};

use crate::{combat::Named, entities::Player, system::game_state, ui::common::ScreenId};
use super::menu_component::{MenuComponent, MenuItem};

pub struct MainMenuHeader<'a> {
    pub player: &'a Player
}

impl<'a> MainMenuHeader<'a> {
    pub fn new(player: &'a Player) -> Self {
        Self { player }
    }
}

impl<'a> MockComponent for MainMenuHeader<'a> {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let player_name = self.player.name();
        let style = Style::default().bold().green().underlined();
        let line = Line::from(vec![
            Span::raw("Hello, "),
            Span::styled(player_name, style)
        ]);
        let paragraph = Paragraph::new(line);
        frame.render_widget(paragraph, area);
    }

    fn attr(&mut self, _attr: Attribute, _value: AttrValue) { }
    fn state(&self) -> State {State::None}
    fn query(&self, _attr: Attribute) -> Option<AttrValue> { None }
    fn perform(&mut self, _cmd: Cmd) -> CmdResult {CmdResult::None}
}

// MainMenu component that combines header and menu
pub struct MainMenu {
    menu: MenuComponent,
}

impl MainMenu {
    pub fn new() -> Self {
        let items = vec![
            MenuItem {
                label: "Fight".to_string(),
                action: Box::new(|| {
                    game_state().current_screen = ScreenId::Fight;
                })
            },
            MenuItem {
                label: "Store".to_string(),
                action: Box::new(|| {
                    game_state().current_screen = ScreenId::Store;
                })
            },
            MenuItem {
                label: "Profile".to_string(),
                action: Box::new(|| {
                    game_state().current_screen = ScreenId::Profile;
                })
            },
            MenuItem {
                label: "Quit".to_string(),
                action: Box::new(|| {
                    game_state().current_screen = ScreenId::Quit;
                })
            },
        ];

        Self {
            menu: MenuComponent::new(items),
        }
    }
}

impl MockComponent for MainMenu {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0)
            ])
            .split(area);

        let mut header = MainMenuHeader::new(&game_state().player);
        header.view(frame, chunks[0]);

        self.menu.view(frame, chunks[1]);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.menu.query(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.menu.attr(attr, value)
    }

    fn state(&self) -> State {
        self.menu.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.menu.perform(cmd)
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for MainMenu {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        self.menu.on(ev)
    }
}
