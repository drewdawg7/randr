use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Modifier, Style}, text::{Line, Span}, widgets::Paragraph, Frame};

use crate::ui::theme::{self as colors, ColorExt};
use tuirealm::{command::{Cmd, CmdResult}, props::{AttrValue, Attribute, Props}, Component, Event, MockComponent, NoUserEvent, State};

use crate::combat::{AttackResult, CombatRounds};
use crate::system::game_state;
use crate::ui::Id;
use crate::ui::components::widgets::menu::{Menu, MenuItem};
use crate::ui::components::utilities::{CROSSED_SWORDS, RETURN_ARROW};

pub struct FightScreen {
    props: Props,
    back_menu: Menu,
}

impl FightScreen {
    pub fn new() -> Self {
        let back_menu = Menu::new(vec![
            MenuItem {
                label: format!("{} Fight again", CROSSED_SWORDS),
                action: Box::new(|| {
                    let field = &game_state().town.field;
                    if let Ok(mut mob) = field.spawn_mob() {
                        let gs = game_state();
                        let combat_rounds = crate::combat::system::enter_combat(&mut gs.player, &mut mob);

                        // Add dropped loot to player inventory
                        for item_kind in &combat_rounds.dropped_loot {
                            let item = gs.spawn_item(*item_kind);
                            let _ = crate::inventory::HasInventory::add_to_inv(&mut gs.player, item);
                        }

                        gs.set_current_combat(combat_rounds);
                    }
                }),
            },
            MenuItem {
                label: format!("{} Back", RETURN_ARROW),
                action: Box::new(|| {
                    game_state().current_screen = Id::Town;
                }),
            },
        ]);
        Self { props: Props::default(), back_menu }
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

        // Calculate summary height: header + gold + xp + items (or "no items")
        let summary_height = if combat_rounds.player_won {
            4 + combat_rounds.dropped_loot.len().max(1) - 1
        } else {
            2 // defeat header + message
        } as u16;

        let num_attacks = attack_components.len();

        // Build constraints: attack results + spacer + summary + back button + trailing spacer
        let mut constraints: Vec<Constraint> = attack_components
            .iter()
            .map(|_| Constraint::Length(2))
            .collect();
        constraints.push(Constraint::Length(1)); // spacer
        constraints.push(Constraint::Length(summary_height)); // summary section
        constraints.push(Constraint::Length(3)); // menu (fight again + back)
        constraints.push(Constraint::Min(0)); // absorb remaining space at end

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints.as_slice())
            .split(area);

        // Render attack results
        for (i, component) in attack_components.iter_mut().enumerate() {
            component.view(frame, chunks[i]);
        }

        // Render battle summary (after attacks + spacer)
        let summary_idx = num_attacks + 1;
        render_battle_summary(frame, chunks[summary_idx], combat_rounds);

        // Render back button
        let back_idx = num_attacks + 2;
        self.back_menu.view(frame, chunks[back_idx]);
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        self.back_menu.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.back_menu.perform(cmd)
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for FightScreen {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        self.back_menu.on(ev)
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

        let attacker_style = Style::default().color(colors::GREEN);
        let defender_style = Style::default().color(colors::GREEN);
        let death_style = Style::default().color(colors::RED);
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
    let header_style = Style::default().color(colors::YELLOW).add_modifier(Modifier::BOLD);
    let gold_style = Style::default().color(colors::YELLOW);
    let xp_style = Style::default().color(colors::CYAN);
    let item_style = Style::default().color(colors::MAGENTA);
    let defeat_style = Style::default().color(colors::RED).add_modifier(Modifier::BOLD);

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
