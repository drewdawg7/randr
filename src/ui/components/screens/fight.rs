use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State,
};

use crate::combat::{ActiveCombat, CombatPhase};
use crate::commands::{apply_result, execute, GameCommand};
use crate::stats::HasStats;
use crate::system::{game_state, CombatSource};
use crate::ui::theme::{self as colors, quality_color, ColorExt};
use crate::ui::components::utilities::{COIN, CROSSED_SWORDS, DOUBLE_ARROW_UP, HEART, RETURN_ARROW};
use crate::ui::components::widgets::border::BorderTheme;


// Placeholder ASCII art (same dimensions for player and enemy)
const PLAYER_ART: &[&str] = &[
    r"  O   ",
    r" -|- ",
    r" / \  ",
];

const ENEMY_ART: &[&str] = &[
    r" /\_/\ ",
    r"( o.o )",
    r" > ^ < ",
];

const ART_WIDTH: u16 = 9;  // 7 chars + 2 for border
const ART_HEIGHT: u16 = 5; // 3 lines + 2 for border

#[derive(Clone, Copy, PartialEq)]
enum FightSelection {
    Attack,
    Run,
}

#[derive(Clone, Copy, PartialEq)]
enum ResultSelection {
    FightAgain,
    Continue,
}

pub struct FightScreen {
    props: Props,
    selection: FightSelection,
    result_selection: ResultSelection,
    victory_processed: bool,
    defeat_processed: bool,
}

impl FightScreen {
    pub fn new() -> Self {
        Self {
            props: Props::default(),
            selection: FightSelection::Attack,
            result_selection: ResultSelection::FightAgain,
            victory_processed: false,
            defeat_processed: false,
        }
    }

    fn reset_for_new_combat(&mut self) {
        self.selection = FightSelection::Attack;
        self.result_selection = ResultSelection::FightAgain;
        self.victory_processed = false;
        self.defeat_processed = false;
    }

    fn execute_player_attack(&mut self) {
        let result = execute(GameCommand::PlayerAttack);
        apply_result(&result);

        // Track victory/defeat for UI state
        let gs = game_state();
        if let Some(combat) = gs.active_combat() {
            if combat.phase == CombatPhase::Victory {
                self.victory_processed = true;
            } else if combat.phase == CombatPhase::Defeat {
                self.defeat_processed = true;
            }
        }
    }

    fn execute_run(&mut self) {
        let result = execute(GameCommand::PlayerRun);
        apply_result(&result);
        self.reset_for_new_combat();
    }

    fn return_from_combat(&mut self) {
        let result = execute(GameCommand::ReturnFromCombat);
        apply_result(&result);
        // Reset combat source to default
        game_state().combat_source = CombatSource::default();
        self.reset_for_new_combat();
    }

    fn start_new_fight(&mut self) {
        let gs = game_state();
        // Only allow "Fight Again" if from Field
        if gs.combat_source != CombatSource::Field {
            self.return_from_combat();
            return;
        }
        let result = execute(GameCommand::StartNewFight);
        apply_result(&result);
        if result.success {
            self.reset_for_new_combat();
        }
    }
}

impl MockComponent for FightScreen {
    fn view(&mut self, frame: &mut Frame, _area: Rect) {
        let gs = game_state();

        let Some(combat) = gs.active_combat() else {
            // No active combat - show empty screen
            return;
        };

        let player = &gs.player;
        let frame_size = frame.area();

        // Fill background
        let bg = Block::default().style(Style::default().on_color(colors::FIGHT_BG));
        frame.render_widget(bg, frame_size);

        // Main layout: combatants, combat log, footer
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(11), // Combatants area (art + name + HP + stats)
                Constraint::Min(5),     // Combat history log
                Constraint::Length(5),  // Footer (actions or results)
            ])
            .split(frame_size);

        let combatants_area = main_chunks[0];
        let combat_log_area = main_chunks[1];
        let footer_area = main_chunks[2];

        // === COMBATANTS: Player (left) | Enemy (right) ===
        render_combatants(frame, combatants_area, player, combat);

        // === COMBAT HISTORY LOG ===
        render_combat_log(frame, combat_log_area, combat);

        // === FOOTER: Actions or Results ===
        match combat.phase {
            CombatPhase::Victory | CombatPhase::Defeat => {
                let show_fight_again = game_state().combat_source == CombatSource::Field;
                render_results(frame, footer_area, combat, self.result_selection, show_fight_again);
            }
            _ => {
                render_action_menu(frame, footer_area, self.selection);
            }
        }

        // ASCII art border (like other screens)
        let border = BorderTheme::Forest;
        let border_style = Style::default().on_color(colors::FIGHT_BG);

        // Top and bottom borders
        let top_border = border.generate_top_border(frame_size.width);
        let bottom_border = border.generate_bottom_border(frame_size.width);
        let top_area = Rect::new(0, 0, frame_size.width, 1);
        let bottom_area = Rect::new(0, frame_size.height.saturating_sub(1), frame_size.width, 1);
        frame.render_widget(Paragraph::new(top_border).style(border_style), top_area);
        frame.render_widget(Paragraph::new(bottom_border).style(border_style), bottom_area);

        // Left and right borders
        let content_height = frame_size.height.saturating_sub(2);
        for row in 0..content_height {
            let left_char = border.generate_left_border_char(row);
            let right_char = border.generate_right_border_char(row);
            let left_area = Rect::new(0, 1 + row, 1, 1);
            let right_area = Rect::new(frame_size.width.saturating_sub(1), 1 + row, 1, 1);
            frame.render_widget(Paragraph::new(Line::from(left_char)).style(border_style), left_area);
            frame.render_widget(Paragraph::new(Line::from(right_char)).style(border_style), right_area);
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

impl Component<Event<NoUserEvent>, NoUserEvent> for FightScreen {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        let gs = game_state();
        let Some(combat) = gs.active_combat() else {
            return None;
        };

        let phase = combat.phase;
        let _ = combat;

        match phase {
            CombatPhase::PlayerTurn => {
                match ev {
                    Event::Keyboard(KeyEvent { code: Key::Up, .. }) |
                    Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                        self.selection = match self.selection {
                            FightSelection::Attack => FightSelection::Run,
                            FightSelection::Run => FightSelection::Attack,
                        };
                    }
                    Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                        match self.selection {
                            FightSelection::Attack => {
                                self.execute_player_attack();
                            }
                            FightSelection::Run => {
                                self.execute_run();
                            }
                        }
                    }
                    Event::Keyboard(KeyEvent { code: Key::Backspace, .. }) => {
                        // Backspace is a shortcut to run
                        self.execute_run();
                    }
                    _ => {}
                }
            }
            CombatPhase::Victory | CombatPhase::Defeat => {
                match ev {
                    Event::Keyboard(KeyEvent { code: Key::Left, .. }) |
                    Event::Keyboard(KeyEvent { code: Key::Right, .. }) => {
                        if phase == CombatPhase::Victory {
                            self.result_selection = match self.result_selection {
                                ResultSelection::FightAgain => ResultSelection::Continue,
                                ResultSelection::Continue => ResultSelection::FightAgain,
                            };
                        }
                    }
                    Event::Keyboard(KeyEvent { code: Key::Enter, .. }) => {
                        match self.result_selection {
                            ResultSelection::FightAgain if phase == CombatPhase::Victory => {
                                self.start_new_fight();
                            }
                            ResultSelection::Continue | ResultSelection::FightAgain => {
                                self.return_from_combat();
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        None
    }
}

fn render_combatants(frame: &mut Frame, area: Rect, player: &crate::player::Player, combat: &ActiveCombat) {
    // Split into left (player) and right (enemy) halves
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_area = chunks[0];
    let right_area = chunks[1];

    // === PLAYER (left side, left-aligned) ===
    render_player_panel(frame, left_area, player);

    // === ENEMY (right side, right-aligned) ===
    render_combatant_right(
        frame,
        right_area,
        &combat.mob.name,
        ENEMY_ART,
        combat.mob.hp(),
        combat.mob.max_hp(),
    );
}

fn render_player_panel(frame: &mut Frame, area: Rect, player: &crate::player::Player) {
    use crate::entities::progression::{HasProgression, Progression};
    use crate::combat::HasGold;

    // Layout: art box, name, HP, HP bar, stats
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(ART_HEIGHT), // Art box
            Constraint::Length(1),          // Name
            Constraint::Length(1),          // HP text
            Constraint::Length(1),          // HP bar
            Constraint::Length(1),          // Level + XP
            Constraint::Length(1),          // Gold
            Constraint::Min(0),             // Remaining
        ])
        .split(area);

    // Art box (left-aligned with padding)
    let art_x = area.x + 2;
    let art_rect = Rect::new(art_x, chunks[0].y, ART_WIDTH, ART_HEIGHT);

    let art_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().color(colors::GREY));
    frame.render_widget(art_block, art_rect);

    let art_lines: Vec<Line> = PLAYER_ART
        .iter()
        .map(|line| Line::from(Span::styled(*line, Style::default().color(colors::WHITE))))
        .collect();
    let inner_rect = Rect::new(art_rect.x + 1, art_rect.y + 1, art_rect.width - 2, art_rect.height - 2);
    frame.render_widget(Paragraph::new(art_lines), inner_rect);

    // Name (left-aligned)
    let name_line = Line::from(vec![
        Span::raw("  "),
        Span::styled("Player", Style::default().color(colors::WHITE)),
    ]);
    frame.render_widget(Paragraph::new(name_line), chunks[1]);

    // HP text
    let hp = player.hp();
    let max_hp = player.max_hp();
    let hp_pct = (hp as f64 / max_hp as f64 * 100.0).max(0.0) as u16;
    let hp_text = Line::from(vec![
        Span::raw("  "),
        Span::styled(format!("{} ", HEART), Style::default().color(colors::RED)),
        Span::raw(format!("{}/{}", hp.max(0), max_hp)),
    ]);
    frame.render_widget(Paragraph::new(hp_text), chunks[2]);

    // HP bar
    let mut bar_spans = vec![Span::raw("  ")];
    bar_spans.extend(hp_bar_spans(hp_pct, 10));
    let bar_line = Line::from(bar_spans);
    frame.render_widget(Paragraph::new(bar_line), chunks[3]);

    // Level + XP
    let prog = player.progression();
    let xp_to_next = Progression::xp_to_next_level(prog.level);
    let level_line = Line::from(vec![
        Span::raw("  "),
        Span::styled(format!("{} ", DOUBLE_ARROW_UP), Style::default().color(colors::CYAN)),
        Span::raw(format!("Lv.{} ", prog.level)),
        Span::styled(format!("{}/{}", prog.xp, xp_to_next), Style::default().color(colors::GREY)),
    ]);
    frame.render_widget(Paragraph::new(level_line), chunks[4]);

    // Gold
    let gold_line = Line::from(vec![
        Span::raw("  "),
        Span::styled(format!("{} ", COIN), Style::default().color(colors::YELLOW)),
        Span::raw(format!("{}", player.gold())),
    ]);
    frame.render_widget(Paragraph::new(gold_line), chunks[5]);
}

fn render_combatant_right(
    frame: &mut Frame,
    area: Rect,
    name: &str,
    art: &[&str],
    hp: i32,
    max_hp: i32,
) {
    // Layout: art box, name, HP
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(ART_HEIGHT), // Art box
            Constraint::Length(1),          // Name
            Constraint::Length(1),          // HP text
            Constraint::Length(1),          // HP bar
            Constraint::Min(0),             // Remaining
        ])
        .split(area);

    // Art box (right-aligned with padding)
    let art_x = area.x + area.width - ART_WIDTH - 2;
    let art_rect = Rect::new(art_x, chunks[0].y, ART_WIDTH, ART_HEIGHT);

    let art_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().color(colors::GREY));
    frame.render_widget(art_block, art_rect);

    let art_lines: Vec<Line> = art
        .iter()
        .map(|line| Line::from(Span::styled(*line, Style::default().color(colors::WHITE))))
        .collect();
    let inner_rect = Rect::new(art_rect.x + 1, art_rect.y + 1, art_rect.width - 2, art_rect.height - 2);
    frame.render_widget(Paragraph::new(art_lines), inner_rect);

    // Name (right-aligned)
    let name_line = Line::from(Span::styled(name, Style::default().color(colors::WHITE)));
    let name_area = Rect::new(area.x, chunks[1].y, area.width - 2, 1);
    frame.render_widget(Paragraph::new(name_line).alignment(Alignment::Right), name_area);

    // HP text (right-aligned)
    let hp_pct = (hp as f64 / max_hp as f64 * 100.0).max(0.0) as u16;
    let hp_text = Line::from(vec![
        Span::styled(format!("{} ", HEART), Style::default().color(colors::RED)),
        Span::raw(format!("{}/{}", hp.max(0), max_hp)),
    ]);
    let hp_area = Rect::new(area.x, chunks[2].y, area.width - 2, 1);
    frame.render_widget(Paragraph::new(hp_text).alignment(Alignment::Right), hp_area);

    // HP bar (right-aligned)
    let bar_line = Line::from(hp_bar_spans(hp_pct, 10));
    let bar_area = Rect::new(area.x, chunks[3].y, area.width - 2, 1);
    frame.render_widget(Paragraph::new(bar_line).alignment(Alignment::Right), bar_area);
}

fn render_combat_log(frame: &mut Frame, area: Rect, combat: &ActiveCombat) {
    let mut lines: Vec<Line> = Vec::new();

    // Group attacks into rounds (each round = player attack + enemy counter-attack)
    let attacks = &combat.rounds.attack_results;
    let mut round_num = 0;

    // Process attacks in pairs (player attack, then enemy attack)
    let mut i = 0;
    while i < attacks.len() {
        round_num += 1;

        // Round header
        lines.push(Line::from(Span::styled(
            format!("Round {}", round_num),
            Style::default().color(colors::YELLOW),
        )));

        // First attack of the round
        let atk1 = &attacks[i];
        let color1 = if atk1.attacker == "Player" { colors::GREEN } else { colors::RED };
        lines.push(Line::from(vec![
            Span::styled(&atk1.attacker, Style::default().color(color1)),
            Span::raw(" dealt "),
            Span::styled(atk1.damage_to_target.to_string(), Style::default().color(color1)),
            Span::raw(format!(" damage to {}", atk1.defender)),
        ]));
        i += 1;

        // Second attack of the round (if exists)
        if i < attacks.len() {
            let atk2 = &attacks[i];
            let color2 = if atk2.attacker == "Player" { colors::GREEN } else { colors::RED };
            lines.push(Line::from(vec![
                Span::styled(&atk2.attacker, Style::default().color(color2)),
                Span::raw(" dealt "),
                Span::styled(atk2.damage_to_target.to_string(), Style::default().color(color2)),
                Span::raw(format!(" damage to {}", atk2.defender)),
            ]));
            i += 1;
        }

        lines.push(Line::from("")); // Blank line between rounds
    }

    // Only show the last 3 rounds (each round = header + 2-3 lines + blank)
    // Calculate how many lines per round (~4) and keep last 3 rounds worth
    let max_lines = 12; // ~3 rounds * 4 lines each
    let skip = lines.len().saturating_sub(max_lines);
    let visible_lines: Vec<Line> = lines.into_iter().skip(skip).collect();

    if visible_lines.is_empty() {
        let empty_msg = vec![Line::from(Span::styled(
            "No attacks yet...",
            Style::default().color(colors::GREY),
        ))];
        frame.render_widget(Paragraph::new(empty_msg).alignment(Alignment::Center), area);
    } else {
        frame.render_widget(Paragraph::new(visible_lines).alignment(Alignment::Center), area);
    }
}

fn render_action_menu(frame: &mut Frame, area: Rect, selection: FightSelection) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Spacer
            Constraint::Length(1), // Attack
            Constraint::Length(1), // Run
            Constraint::Min(0),    // Remaining
        ])
        .split(area);

    let attack_prefix = if selection == FightSelection::Attack { "> " } else { "  " };
    let run_prefix = if selection == FightSelection::Run { "> " } else { "  " };

    let attack_style = if selection == FightSelection::Attack {
        Style::default().color(colors::YELLOW)
    } else {
        Style::default().color(colors::WHITE)
    };

    let run_style = if selection == FightSelection::Run {
        Style::default().color(colors::YELLOW)
    } else {
        Style::default().color(colors::WHITE)
    };

    let attack_line = Line::from(vec![
        Span::styled(attack_prefix, attack_style),
        Span::styled(format!("{} Attack", CROSSED_SWORDS), attack_style),
    ]);
    let run_line = Line::from(vec![
        Span::styled(run_prefix, run_style),
        Span::styled(format!("{} Run", RETURN_ARROW), run_style),
    ]);

    frame.render_widget(Paragraph::new(attack_line).alignment(Alignment::Center), chunks[1]);
    frame.render_widget(Paragraph::new(run_line).alignment(Alignment::Center), chunks[2]);
}

fn render_results(frame: &mut Frame, area: Rect, combat: &ActiveCombat, selection: ResultSelection, show_fight_again: bool) {
    let is_victory = combat.phase == CombatPhase::Victory;

    let mut lines = Vec::new();

    if is_victory {
        lines.push(Line::from(Span::styled("== Victory ==", Style::default().color(colors::YELLOW))));
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", COIN), Style::default().color(colors::YELLOW)),
            Span::raw(format!("+{} gold  ", combat.gold_gained)),
            Span::styled(format!("{} ", DOUBLE_ARROW_UP), Style::default().color(colors::CYAN)),
            Span::raw(format!("+{} XP", combat.xp_gained)),
        ]));

        // Show dropped items
        for loot_drop in &combat.loot_drops {
            let color = quality_color(loot_drop.item.quality);
            lines.push(Line::from(vec![
                Span::raw("+ "),
                Span::styled(loot_drop.item.name.to_string(), Style::default().color(color)),
            ]));
        }

        // Menu options - only show "Fight Again" if from Field
        if show_fight_again {
            let fight_prefix = if selection == ResultSelection::FightAgain { "> " } else { "  " };
            let cont_prefix = if selection == ResultSelection::Continue { "> " } else { "  " };
            let fight_style = if selection == ResultSelection::FightAgain {
                Style::default().color(colors::YELLOW)
            } else {
                Style::default().color(colors::WHITE)
            };
            let cont_style = if selection == ResultSelection::Continue {
                Style::default().color(colors::YELLOW)
            } else {
                Style::default().color(colors::WHITE)
            };

            lines.push(Line::from(vec![
                Span::styled(fight_prefix, fight_style),
                Span::styled(format!("{} Fight Again", CROSSED_SWORDS), fight_style),
                Span::raw("    "),
                Span::styled(cont_prefix, cont_style),
                Span::styled(format!("{} Continue", RETURN_ARROW), cont_style),
            ]));
        } else {
            // Dungeon combat - only show Continue
            lines.push(Line::from(vec![
                Span::styled("> ", Style::default().color(colors::YELLOW)),
                Span::styled(format!("{} Continue", RETURN_ARROW), Style::default().color(colors::YELLOW)),
            ]));
        }
    } else {
        lines.push(Line::from(Span::styled("== Defeat ==", Style::default().color(colors::RED))));
        lines.push(Line::from("You have been slain..."));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("> ", Style::default().color(colors::YELLOW)),
            Span::styled(format!("{} Continue", RETURN_ARROW), Style::default().color(colors::YELLOW)),
        ]));
    }

    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), area);
}

const FILLED_HEART: char = '❤';
const EMPTY_HEART: char = '♡';

fn hp_bar_spans(pct: u16, width: u16) -> Vec<Span<'static>> {
    let filled = ((pct as f64 / 100.0) * width as f64).round() as u16;
    let empty = width.saturating_sub(filled);

    vec![
        Span::raw("["),
        Span::styled(
            FILLED_HEART.to_string().repeat(filled as usize),
            Style::default().color(colors::RED),
        ),
        Span::styled(
            EMPTY_HEART.to_string().repeat(empty as usize),
            Style::default().color(colors::DARK_GRAY),
        ),
        Span::raw("]"),
    ]
}

#[allow(dead_code)]
fn hp_color(pct: u16) -> ratatui::style::Color {
    if pct > 60 {
        colors::GREEN
    } else if pct > 30 {
        colors::YELLOW
    } else {
        colors::RED
    }
}
