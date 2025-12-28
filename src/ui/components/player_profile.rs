use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::*, text::{Line, Span}, widgets::Paragraph, Frame};
use tuirealm::{command::{Cmd, CmdResult}, props::{Attribute, AttrValue, Props}, Component, Event, MockComponent, NoUserEvent, State};

use crate::{combat::{Combatant, HasGold, Named}, ui::utilities::SHIELD};
use crate::system::game_state;
use crate::ui::components::utilities::{COIN, CROSSED_SWORDS, HEART};
use crate::ui::fittedbox::FittedBox;
use super::xp_bar::XpBar;

pub struct PlayerProfile {
    props: Props,
    xp_bar: XpBar,
}

impl PlayerProfile {
    pub fn new() -> Self {
        Self {
            props: Props::default(),
            xp_bar: XpBar::new(),
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
        let attack = player.attack_power();
        let defense = player.defense();
        let lines = vec![
            Line::from(vec![
                Span::styled(format!("{}", HEART), Style::default().red().bold()),
                Span::raw(" "),
                Span::raw(format!("{}/{}", health, max_health))
            ]),
            Line::from(vec![
                Span::styled(format!("{}", COIN), Style::default().yellow()),
                Span::raw(" "),
                Span::raw(format!("{}", gold))
            ]),
            Line::from(vec![
                Span::styled(format!("{}", CROSSED_SWORDS), Style::default().white()),
                Span::raw(" "),
                Span::raw(format!("{}", attack))
            ]),

            Line::from(vec![
                Span::styled(format!("{}", SHIELD), Style::default().white()),
                Span::raw(" "),
                Span::raw(format!("{}", defense))
            ]),
        ];

        let width = lines.iter().map(|l| l.width()).max().unwrap_or(0) as u16;
        let height = lines.len() as u16;

        let profile_box = FittedBox::new(Paragraph::new(lines), width, height)
            .title(name)
            .title_style(Style::default().bold().green().on_dark_gray());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(profile_box.height() + 2), Constraint::Length(1), Constraint::Min(0)])
            .split(area);

        frame.render_widget(profile_box, chunks[0]);
        self.xp_bar.view(frame, chunks[1]);
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for PlayerProfile {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        None
    }
}
