
use ratatui::{layout::{Constraint, Direction, Layout}, style::{Color, Style}, text::{Line, Span}, widgets::Paragraph};
use tuirealm::{command::CmdResult, Component, Event, MockComponent, NoUserEvent, State};

use crate::{combat::{AttackResult, CombatRounds}, ui::{common::ScreenId, components::utilities::back_button, menu_component::{MenuComponent}}};
pub struct FightComponent {
    attack_components: Vec<AttackResultComponent>,
    back_menu: MenuComponent
}

impl FightComponent {
    pub fn new(combat_rounds: CombatRounds, back_screen: ScreenId) -> Self {
        let attack_components = combat_rounds
            .attack_results
            .into_iter()
            .map(AttackResultComponent::new)
            .collect();
        let back_menu = back_button(back_screen);
        Self { attack_components, back_menu }
    }
}

impl MockComponent for FightComponent {
    fn view(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        // Split into attack results area and back menu area
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),      // Attack results take remaining space
                Constraint::Length(3),   // Back menu
            ])
            .split(area);

        // Split attack results area
        let attack_constraints: Vec<Constraint> = self.attack_components
            .iter()
            .map(|_| Constraint::Length(2))
            .collect();

        let attack_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(attack_constraints.as_slice())
            .split(main_chunks[0]);

        for (component, chunk) in self.attack_components.iter_mut().zip(attack_chunks.iter()) {
            component.view(frame, *chunk);
        }

        self.back_menu.view(frame, main_chunks[1]);
    }
    fn query(&self, _attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> {
        None
    }

    fn attr(&mut self, _attr: tuirealm::Attribute, _value: tuirealm::AttrValue) {}

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: tuirealm::command::Cmd) -> tuirealm::command::CmdResult {
        CmdResult::None
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for FightComponent {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        self.back_menu.on(ev)
    }
}
pub struct AttackResultComponent {
    pub attack_result: AttackResult
}

impl AttackResultComponent {
    pub fn new(attack_result: AttackResult) -> Self {
        Self { attack_result }
    }
}


impl MockComponent for AttackResultComponent {
    fn view(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
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
                    Span::styled(
                        format!("{} died.", defender),
                        death_style
                    ),
                ]));
        } else {
            lines.push(Line::from(vec![
                    Span::styled(
                        format!("{}: {} -> {} HP", defender, self.attack_result.target_health_before, self.attack_result.target_health_after),
                        death_style
                    ),
                ]));
        }
        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);

    }

    fn query(&self, _attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> { None }

    fn attr(&mut self, _attr: tuirealm::Attribute, _value: tuirealm::AttrValue) {}

    fn state(&self) -> State { State::None }

    fn perform(&mut self, _cmd: tuirealm::command::Cmd) -> tuirealm::command::CmdResult {
        CmdResult::None    
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for AttackResultComponent {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        None
    }
}
