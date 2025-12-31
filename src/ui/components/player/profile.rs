use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Style, Stylize}, text::{Line, Span}, widgets::{List, ListItem, ListState, Paragraph}, Frame};

use crate::ui::theme::{self as colors, ColorExt};
use tuirealm::{command::{Cmd, CmdResult}, event::{Key, KeyEvent}, props::{Attribute, AttrValue, Props}, Component, Event, MockComponent, NoUserEvent, State, StateValue};

use crate::{combat::{Combatant, HasGold, Named}, ui::Id};
use crate::system::game_state;
use crate::ui::components::utilities::{COIN, CROSSED_SWORDS, HEART, SHIELD, RETURN_ARROW};
use crate::ui::components::widgets::fitted_box::FittedBox;
use super::xp_bar::XpBar;

pub struct PlayerProfile {
    props: Props,
    xp_bar: XpBar,
    list_state: ListState,
}

impl PlayerProfile {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            props: Props::default(),
            xp_bar: XpBar::new(),
            list_state,
        }
    }
}

impl MockComponent for PlayerProfile {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let player = &game_state().player;
        let name = player.name();
        let health = player.get_health();
        let max_health = player.get_max_health();
        let gold = player.gold();
        let attack = player.effective_attack();
        let defense = player.effective_defense();
        let lines = vec![
            Line::from(vec![
                Span::styled(format!("{}", HEART), Style::default().color(colors::RED).bold()),
                Span::raw(" "),
                Span::raw(format!("{}/{}", health, max_health))
            ]),
            Line::from(vec![
                Span::styled(format!("{}", COIN), Style::default().color(colors::YELLOW)),
                Span::raw(" "),
                Span::raw(format!("{}", gold))
            ]),
            Line::from(vec![
                Span::styled(format!("{}", CROSSED_SWORDS), Style::default().color(colors::WHITE)),
                Span::raw(" "),
                Span::raw(format!("{}", attack))
            ]),

            Line::from(vec![
                Span::styled(format!("{}", SHIELD), Style::default().color(colors::WHITE)),
                Span::raw(" "),
                Span::raw(format!("{}", defense))
            ]),
        ];

        let width = lines.iter().map(|l| l.width()).max().unwrap_or(0) as u16;
        let height = lines.len() as u16;

        let profile_box = FittedBox::new(Paragraph::new(lines), width, height)
            .title(name)
            .title_style(Style::default().bold().color(colors::GREEN).on_color(colors::DARK_GRAY));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(profile_box.height() + 2),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(area);

        frame.render_widget(profile_box, chunks[0]);
        self.xp_bar.view(frame, chunks[1]);

        // Render back button
        let selected = self.list_state.selected().unwrap_or(0) == 0;
        let back_style = if selected {
            Style::default().color(colors::YELLOW)
        } else {
            Style::default()
        };
        let back_prefix = if selected { "> " } else { "  " };
        let back_items = vec![ListItem::new(format!("{}{} Back", back_prefix, RETURN_ARROW)).style(back_style)];
        let back_list = List::new(back_items);
        frame.render_stateful_widget(back_list, chunks[2], &mut self.list_state);
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::One(StateValue::Usize(self.list_state.selected().unwrap_or(0)))
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Move(tuirealm::command::Direction::Up) |
            Cmd::Move(tuirealm::command::Direction::Down) => {
                // Only one item (back button), so no movement needed
                CmdResult::None
            }
            Cmd::Submit => {
                // Back button selected
                game_state().current_screen = Id::Menu;
                CmdResult::Submit(self.state())
            }
            _ => CmdResult::None
        }
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for PlayerProfile {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                game_state().current_screen = Id::Menu;
                None
            }
            _ => None
        }
    }
}
