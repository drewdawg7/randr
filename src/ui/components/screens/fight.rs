use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Color, Modifier, Style}, text::{Line, Span}, widgets::Paragraph, Frame};
use tuirealm::{command::{Cmd, CmdResult}, props::{AttrValue, Attribute, Props}, Component, Event, MockComponent, NoUserEvent, State};

use crate::combat::{AttackResult, CombatRounds};
use crate::system::game_state;

pub struct FightScreen {
    props: Props,
}

impl FightScreen {
    pub fn new() -> Self {
        Self { props: Props::default() }
    }
}

impl MockComponent for FightScreen {
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

        // Build constraints: attack results + summary section
        let mut constraints: Vec<Constraint> = attack_components
            .iter()
            .map(|_| Constraint::Length(2))
            .collect();
        constraints.push(Constraint::Length(1)); // spacer
        constraints.push(Constraint::Min(5)); // summary section

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints.as_slice())
            .split(area);

        // Render attack results
        for (i, component) in attack_components.iter_mut().enumerate() {
            component.view(frame, chunks[i]);
        }

        // Render battle summary
        let summary_area = chunks[chunks.len() - 1];
        render_battle_summary(frame, summary_area, combat_rounds);
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

impl Component<Event<NoUserEvent>, NoUserEvent> for FightScreen {
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

fn render_battle_summary(frame: &mut Frame, area: Rect, combat: &CombatRounds) {
    let header_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
    let gold_style = Style::default().fg(Color::Yellow);
    let xp_style = Style::default().fg(Color::Cyan);
    let item_style = Style::default().fg(Color::Magenta);
    let defeat_style = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);

    let mut lines = Vec::new();

    if combat.player_won {
        lines.push(Line::from(Span::styled("== Battle Results ==", header_style)));
        lines.push(Line::from(vec![
            Span::styled(format!("+{} gold", combat.gold_gained), gold_style),
        ]));
        lines.push(Line::from(vec![
            Span::styled(format!("+{} XP", combat.xp_gained), xp_style),
        ]));

        if combat.dropped_loot.is_empty() {
            lines.push(Line::from(Span::raw("No items dropped")));
        } else {
            let gs = game_state();
            for item_kind in &combat.dropped_loot {
                let item_name = gs.get_item_name(*item_kind);
                lines.push(Line::from(vec![
                    Span::styled(format!("+ {}", item_name), item_style),
                ]));
            }
        }
    } else {
        lines.push(Line::from(Span::styled("== DEFEAT ==", defeat_style)));
        lines.push(Line::from(Span::raw("You have been slain...")));
    }

    frame.render_widget(Paragraph::new(lines), area);
}
