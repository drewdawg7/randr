use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::Paragraph, Frame};
use tuirealm::{command::{Cmd, CmdResult}, props::{AttrValue, Attribute, Props}, Component, Event, MockComponent, NoUserEvent, State};

use crate::combat::AttackResult;
use crate::system::game_state;

pub struct FightComponent {
    props: Props,
}

impl FightComponent {
    pub fn new() -> Self {
        Self { props: Props::default() }
    }
}

impl MockComponent for FightComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let Some(combat_rounds) = game_state().current_combat() else {
            return;
        };

        let mut attack_components: Vec<AttackResultComponent> = combat_rounds
            .attack_results
            .iter()
            .cloned()
            .map(AttackResultComponent::new)
            .collect();

        let attack_constraints: Vec<Constraint> = attack_components
            .iter()
            .map(|_| Constraint::Length(2))
            .collect();

        let attack_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(attack_constraints.as_slice())
            .split(area);

        for (component, chunk) in attack_components.iter_mut().zip(attack_chunks.iter()) {
            component.view(frame, *chunk);
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for FightComponent {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        None
    }
}

pub struct AttackResultComponent {
    props: Props,
    attack_result: AttackResult
}

impl AttackResultComponent {
    pub fn new(attack_result: AttackResult) -> Self {
        Self { props: Props::default(), attack_result }
    }
}

impl MockComponent for AttackResultComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let attacker = &self.attack_result.attacker;
        let defender = &self.attack_result.defender;

        let attacker_style = Style::default().fg(Color::Green);
        let defender_style = Style::default().fg(Color::Green);
        let death_style = Style::default().fg(Color::Red);
        let styled_attacker = Span::styled(attacker, attacker_style);
        let styled_defender = Span::styled(defender, defender_style);

        let mut lines = vec![
            Line::from(vec![
                styled_attacker,
                Span::raw(" did "),
                Span::raw(self.attack_result.damage_to_target.to_string()),
                Span::raw(" damage to "),
                styled_defender,
            ])
        ];

        if self.attack_result.target_died {
            lines.push(Line::from(vec![
                Span::styled(format!("{} died.", defender), death_style),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::styled(
                    format!("{}: {} -> {} HP", defender, self.attack_result.target_health_before, self.attack_result.target_health_after),
                    death_style
                ),
            ]));
        }

        frame.render_widget(Paragraph::new(lines), area);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for AttackResultComponent {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        None
    }
}
