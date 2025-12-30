use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::{Modifier, Style}, text::{Line, Span}, widgets::Paragraph, Frame};

use crate::ui::theme::{self as colors, ColorExt};
use tuirealm::{command::{Cmd, CmdResult}, props::{AttrValue, Attribute, Props}, Component, Event, MockComponent, NoUserEvent, State};

use crate::combat::{AttackResult, CombatRounds};
use crate::system::game_state;
use crate::ui::Id;
use crate::ui::components::player::xp_bar::XpBar;
use crate::ui::components::widgets::menu::{Menu, MenuItem};
use crate::ui::components::utilities::{COIN, CROSSED_SWORDS, DOUBLE_ARROW_UP, HEART, RETURN_ARROW};

pub struct FightScreen {
    props: Props,
    back_menu: Menu,
    xp_bar: XpBar,
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
        Self { props: Props::default(), back_menu, xp_bar: XpBar::new() }
    }
}

impl MockComponent for FightScreen {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let gs = game_state();
        let Some(combat_rounds) = gs.current_combat() else {
            return;
        };

        // Only show the last 3 rounds of combat
        let attack_results: Vec<_> = combat_rounds
            .attack_results
            .iter()
            .rev()
            .take(3)
            .rev()
            .cloned()
            .collect();

        let mut attack_components: Vec<AttackResultComponent> = attack_results
            .into_iter()
            .map(AttackResultComponent::new)
            .collect();

        // Calculate summary height: header + gold + xp + items (or "no items")
        let summary_height = if combat_rounds.player_won {
            4 + combat_rounds.dropped_loot.len().max(1) - 1
        } else {
            2 // defeat header + message
        } as u16;

        // Fixed height for combat rounds (3 rounds * 2 lines each = 6)
        const MAX_ROUNDS: usize = 3;
        const ROUND_HEIGHT: u16 = 2;
        const COMBAT_AREA_HEIGHT: u16 = (MAX_ROUNDS as u16) * ROUND_HEIGHT;

        // Build constraints: player stats header + spacer + combat area (fixed) + spacer + summary + back button + trailing spacer
        let constraints = vec![
            Constraint::Length(1),                  // player stats header
            Constraint::Length(1),                  // spacer after header
            Constraint::Length(COMBAT_AREA_HEIGHT), // fixed combat rounds area
            Constraint::Length(1),                  // spacer
            Constraint::Length(summary_height),     // summary section
            Constraint::Length(3),                  // menu (fight again + back)
            Constraint::Min(0),                     // absorb remaining space at end
        ];

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints.as_slice())
            .split(area);

        // Split the combat area into individual round slots
        let combat_area = chunks[2];
        let round_constraints: Vec<Constraint> = (0..MAX_ROUNDS)
            .map(|_| Constraint::Length(ROUND_HEIGHT))
            .collect();
        let round_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(round_constraints)
            .split(combat_area);

        // Render player stats header using horizontal layout
        let player = &gs.player;
        let hp_style = Style::default().color(colors::RED);
        let xp_style = Style::default().color(colors::CYAN);
        let gold_style = Style::default().color(colors::YELLOW);

        let header_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(11), // HP section
                Constraint::Length(9),  // XP icon + level
                Constraint::Length(20), // XpBar component
                Constraint::Min(0),     // Gold section
            ])
            .split(chunks[0]);

        // HP
        let hp_line = Line::from(vec![
            Span::styled(format!("{} ", HEART), hp_style),
            Span::raw(format!("{}/{}  |  ", player.get_health(), player.get_max_health())),
        ]);
        frame.render_widget(Paragraph::new(hp_line), header_chunks[0]);

        // XP icon + level
        let xp_label = Line::from(vec![
            Span::styled(format!("{} ", DOUBLE_ARROW_UP), xp_style),
            Span::raw(format!("Lv.{} ", player.prog.level)),
        ]);
        frame.render_widget(Paragraph::new(xp_label), header_chunks[1]);

        // XpBar component
        self.xp_bar.view(frame, header_chunks[2]);

        // Gold
        let gold_line = Line::from(vec![
            Span::raw("  |  "),
            Span::styled(format!("{} ", COIN), gold_style),
            Span::raw(format!("{}", player.gold)),
        ]);
        frame.render_widget(Paragraph::new(gold_line), header_chunks[3]);

        // Render attack results in the fixed round slots
        for (i, component) in attack_components.iter_mut().enumerate() {
            component.view(frame, round_chunks[i]);
        }

        // Render battle summary (fixed position after combat area + spacer)
        render_battle_summary(frame, chunks[4], combat_rounds);

        // Render back button (fixed position)
        self.back_menu.view(frame, chunks[5]);
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
    let gold_icon_style = Style::default().color(colors::YELLOW);
    let xp_icon_style = Style::default().color(colors::CYAN);
    let item_icon_style = Style::default().color(colors::MAGENTA);
    let defeat_style = Style::default().color(colors::RED).add_modifier(Modifier::BOLD);

    let mut lines = Vec::new();

    if combat.player_won {
        lines.push(Line::from(Span::styled("== Battle Results ==", header_style)));
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", COIN), gold_icon_style),
            Span::raw(format!("+{} gold", combat.gold_gained)),
        ]));
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", DOUBLE_ARROW_UP), xp_icon_style),
            Span::raw(format!("+{} XP", combat.xp_gained)),
        ]));

        if combat.dropped_loot.is_empty() {
            lines.push(Line::from(Span::raw("No items dropped")));
        } else {
            let gs = game_state();
            for item_kind in &combat.dropped_loot {
                let item_name = gs.get_item_name(*item_kind);
                lines.push(Line::from(vec![
                    Span::styled("+ ", item_icon_style),
                    Span::raw(item_name),
                ]));
            }
        }
    } else {
        lines.push(Line::from(Span::styled("== DEFEAT ==", defeat_style)));
        lines.push(Line::from(Span::raw("You have been slain...")));
    }

    frame.render_widget(Paragraph::new(lines), area);
}
