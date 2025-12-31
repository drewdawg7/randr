use ratatui::{layout::{Constraint, Direction, Layout, Rect}, style::Style, text::{Line, Span}, widgets::{Block, Paragraph}, Frame};

use crate::ui::theme::{self as colors, quality_color, ColorExt};
use tuirealm::{command::{Cmd, CmdResult}, event::{Key, KeyEvent, KeyModifiers}, props::{AttrValue, Attribute, Props}, Component, Event, MockComponent, NoUserEvent, State};

use crate::combat::{AttackResult, CombatRounds};
use crate::system::game_state;
use crate::ui::Id;
use crate::ui::components::player::xp_bar::XpBar;
use crate::ui::components::player::item_details::render_item_details;
use crate::ui::components::widgets::menu::{Menu, MenuItem};
use crate::ui::components::widgets::forest_border;
use crate::ui::components::utilities::{COIN, CROSSED_SWORDS, DOUBLE_ARROW_UP, HEART, RETURN_ARROW, item_display, lock_prefix, selection_prefix};
use crate::inventory::{HasInventory, EquipmentSlot};
use crate::item::Item;

#[derive(Clone, Copy, PartialEq)]
enum FightFocus {
    Menu,
    Inventory,
}

pub struct FightScreen {
    props: Props,
    back_menu: Menu,
    xp_bar: XpBar,
    focus: FightFocus,
    inventory_selected: usize,
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
                        let mut combat_rounds = crate::combat::system::enter_combat(&mut gs.player, &mut mob);

                        // Spawn items with quality and add to both dropped_loot and inventory
                        let loot_drops = combat_rounds.loot_drops().to_vec();
                        for (item_kind, quantity) in loot_drops {
                            for _ in 0..quantity {
                                let item = gs.spawn_item(item_kind);
                                combat_rounds.dropped_loot.push(item.clone());
                                let _ = crate::inventory::HasInventory::add_to_inv(&mut gs.player, item);
                            }
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
        Self {
            props: Props::default(),
            back_menu,
            xp_bar: XpBar::new(),
            focus: FightFocus::Menu,
            inventory_selected: 0,
        }
    }

    fn get_inventory_items(&self) -> Vec<(Item, EquipmentSlot)> {
        let gs = game_state();
        let player = &gs.player;
        let mut items = Vec::new();

        if let Some(inv_item) = player.get_equipped_item(EquipmentSlot::Weapon) {
            items.push((inv_item.item.clone(), EquipmentSlot::Weapon));
        }
        if let Some(inv_item) = player.get_equipped_item(EquipmentSlot::OffHand) {
            items.push((inv_item.item.clone(), EquipmentSlot::OffHand));
        }
        if let Some(inv_item) = player.get_equipped_item(EquipmentSlot::Ring) {
            items.push((inv_item.item.clone(), EquipmentSlot::Ring));
        }
        for inv_item in player.get_inventory_items().iter() {
            if let Some(slot) = inv_item.item.item_type.equipment_slot() {
                items.push((inv_item.item.clone(), slot));
            }
        }
        items
    }

    fn get_inventory_count(&self) -> usize {
        self.get_inventory_items().len()
    }

    fn get_selected_item(&self) -> Option<Item> {
        let items = self.get_inventory_items();
        items.get(self.inventory_selected).map(|(item, _)| item.clone())
    }

    fn toggle_equip_selected(&mut self) {
        let items = self.get_inventory_items();
        if let Some((item, slot)) = items.get(self.inventory_selected) {
            let gs = game_state();
            if item.is_equipped {
                let _ = gs.player.unequip_item(*slot);
            } else {
                gs.player.equip_from_inventory(item.item_uuid, *slot);
            }
        }
    }
}

impl MockComponent for FightScreen {
    fn view(&mut self, frame: &mut Frame, _area: Rect) {
        let gs = game_state();
        let Some(combat_rounds) = gs.current_combat() else {
            return;
        };

        let frame_size = frame.area();

        // Border offsets
        let y_offset: u16 = 1;
        let x_offset: u16 = 1;

        // Reserve space for top border (1) and bottom border (1)
        // Content area is everything between the borders
        let content_height = frame_size.height.saturating_sub(2); // 2 = top + bottom borders
        let content_area = Rect {
            x: x_offset,
            y: y_offset,
            width: frame_size.width.saturating_sub(x_offset * 2),
            height: content_height,
        };

        // Fill background with FIGHT_BG
        let bg_fill = Block::default().style(Style::default().on_color(colors::FIGHT_BG));
        frame.render_widget(bg_fill, content_area);

        // First split: header area (full width) then body area
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // player stats header (full width)
                Constraint::Length(1), // separator (full width)
                Constraint::Min(0),    // body area
            ])
            .split(content_area);

        let header_area = main_chunks[0];
        let separator_area = main_chunks[1];
        let body_area = main_chunks[2];

        // Split body area horizontally: left for battle, right for inventory
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(body_area);
        let left_panel = horizontal_chunks[0];
        let right_panel = horizontal_chunks[1];

        // Only show the last 4 rounds of combat
        let attack_results: Vec<_> = combat_rounds
            .attack_results
            .iter()
            .rev()
            .take(4)
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

        // Fixed height for combat rounds (4 rounds * 3 lines each = 12)
        const MAX_ROUNDS: usize = 4;
        const ROUND_HEIGHT: u16 = 3; // 2 lines content + 1 separator
        const COMBAT_AREA_HEIGHT: u16 = (MAX_ROUNDS as u16) * ROUND_HEIGHT;

        // Build constraints for left panel (battle content)
        let left_constraints = vec![
            Constraint::Length(COMBAT_AREA_HEIGHT), // fixed combat rounds area
            Constraint::Length(1),                  // spacer
            Constraint::Length(summary_height),     // summary section
            Constraint::Length(3),                  // menu (fight again + back)
            Constraint::Min(0),                     // absorb remaining space at end
        ];

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(left_constraints.as_slice())
            .split(left_panel);

        // Split the combat area into individual round slots
        let combat_area = left_chunks[0];
        let round_constraints: Vec<Constraint> = (0..MAX_ROUNDS)
            .map(|_| Constraint::Length(ROUND_HEIGHT))
            .collect();
        let round_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(round_constraints)
            .split(combat_area);

        // Render player stats header using horizontal layout (full width)
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
            .split(header_area);

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

        // Render separator line after header (full width)
        let separator = Line::from(vec![
            Span::styled("─".repeat(content_area.width as usize), Style::default().color(colors::DARK_FOREST)),
        ]);
        frame.render_widget(Paragraph::new(separator), separator_area);

        // Render attack results in the fixed round slots
        for (i, component) in attack_components.iter_mut().enumerate() {
            component.view(frame, round_chunks[i]);
        }

        // Render battle summary (fixed position after combat area + spacer)
        render_battle_summary(frame, left_chunks[2], combat_rounds);

        // Render back button (fixed position)
        self.back_menu.view(frame, left_chunks[3]);

        // Split right panel: inventory list (left) and item details (right)
        let right_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(right_panel);

        // Render inventory list
        let inv_focused = self.focus == FightFocus::Inventory;
        let inventory_items = self.get_inventory_items();
        render_inventory_panel(frame, right_chunks[0], inv_focused, self.inventory_selected, &inventory_items);

        // Render item details for selected item
        let selected_item = self.get_selected_item();
        render_item_details(frame, right_chunks[1], selected_item.as_ref());

        // Render forest borders
        let total_border_width = content_area.width + 2;
        // Bottom border is at the last row of the frame
        let bottom_y = frame_size.height.saturating_sub(1);
        let border_height = content_height;

        // Border style with themed background
        let border_style = Style::default().on_color(colors::FIGHT_BG);

        // Top and bottom borders
        let border_area_top = Rect { x: 0, y: 0, width: total_border_width, height: 1 };
        let border_area_bottom = Rect { x: 0, y: bottom_y, width: total_border_width, height: 1 };

        let top_border = forest_border::generate_top_border(total_border_width);
        let bottom_border = forest_border::generate_bottom_border(total_border_width);
        frame.render_widget(Paragraph::new(top_border).style(border_style), border_area_top);
        frame.render_widget(Paragraph::new(bottom_border).style(border_style), border_area_bottom);

        // Left and right borders
        for row in 0..border_height {
            let left_char = forest_border::generate_left_border_char(row);
            let right_char = forest_border::generate_right_border_char(row);
            let left_area = Rect { x: 0, y: y_offset + row, width: 1, height: 1 };
            let right_area = Rect { x: x_offset + content_area.width, y: y_offset + row, width: 1, height: 1 };
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
        self.back_menu.state()
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.back_menu.perform(cmd)
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for FightScreen {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        // Handle Shift+Tab to toggle focus
        if let Event::Keyboard(KeyEvent { code: Key::BackTab, modifiers: KeyModifiers::SHIFT }) = ev {
            self.focus = match self.focus {
                FightFocus::Menu => FightFocus::Inventory,
                FightFocus::Inventory => FightFocus::Menu,
            };
            return None;
        }

        match self.focus {
            FightFocus::Menu => self.back_menu.on(ev),
            FightFocus::Inventory => {
                // Handle inventory navigation
                let item_count = self.get_inventory_count();
                if item_count == 0 {
                    return None;
                }

                match ev {
                    Event::Keyboard(KeyEvent { code: Key::Up, .. }) => {
                        if self.inventory_selected == 0 {
                            self.inventory_selected = item_count.saturating_sub(1);
                        } else {
                            self.inventory_selected = self.inventory_selected.saturating_sub(1);
                        }
                        None
                    }
                    Event::Keyboard(KeyEvent { code: Key::Down, .. }) => {
                        self.inventory_selected = (self.inventory_selected + 1) % item_count;
                        None
                    }
                    Event::Keyboard(KeyEvent { code: Key::Char('E'), modifiers: KeyModifiers::SHIFT }) => {
                        self.toggle_equip_selected();
                        None
                    }
                    Event::Keyboard(KeyEvent { code: Key::Char('L'), modifiers: KeyModifiers::SHIFT }) => {
                        let items = self.get_inventory_items();
                        if let Some((item, _slot)) = items.get(self.inventory_selected) {
                            let item_uuid = item.item_uuid;
                            if let Some(inv_item) = game_state().player.find_item_by_uuid_mut(item_uuid) {
                                inv_item.item.toggle_lock();
                            }
                        }
                        None
                    }
                    _ => None
                }
            }
        }
    }
}

pub struct AttackResultComponent {
    props: Props,
    attack_result: AttackResult,
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

        let mut lines = vec![
            // Simple attack line
            Line::from(vec![
                Span::raw(format!("{} dealt ", attacker)),
                Span::styled(self.attack_result.damage_to_target.to_string(), Style::default().color(colors::RED)),
                Span::raw(format!(" damage to {}", defender)),
            ]),
        ];

        if self.attack_result.target_died {
            lines.push(Line::from(vec![
                Span::styled(format!("{} has been slain!", defender), Style::default().color(colors::RED)),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::raw(format!("{} HP: {} → {}", defender, self.attack_result.target_health_before, self.attack_result.target_health_after)),
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
    let mut lines = Vec::new();

    if combat.player_won {
        lines.push(Line::from(Span::styled("== Victory ==", Style::default().color(colors::YELLOW))));
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", COIN), Style::default().color(colors::YELLOW)),
            Span::raw(format!("+{} gold", combat.gold_gained)),
        ]));
        lines.push(Line::from(vec![
            Span::styled(format!("{} ", DOUBLE_ARROW_UP), Style::default().color(colors::CYAN)),
            Span::raw(format!("+{} XP", combat.xp_gained)),
        ]));

        for item in &combat.dropped_loot {
            let color = quality_color(item.quality);
            lines.push(Line::from(vec![
                Span::raw("+ "),
                Span::styled(item.name.to_string(), Style::default().color(color)),
                Span::raw(" dropped"),
            ]));
        }
    } else {
        lines.push(Line::from(Span::styled("== Defeat ==", Style::default().color(colors::RED))));
        lines.push(Line::from("You have been slain..."));
    }

    frame.render_widget(Paragraph::new(lines), area);
}

fn render_inventory_panel(frame: &mut Frame, area: Rect, focused: bool, selected: usize, items: &[(Item, EquipmentSlot)]) {
    let gs = game_state();
    let player = &gs.player;
    let used_slots = player.inventory().items.len();
    let max_slots = player.inventory().max_slots();

    // Split into slot counter and list
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(area);

    // Slot counter
    let slot_line = Line::from(Span::styled(
        format!("{}/{}", used_slots, max_slots),
        Style::default().color(colors::GREY),
    ));
    frame.render_widget(Paragraph::new(slot_line), chunks[0]);

    // Render inventory items
    let mut lines: Vec<Line> = Vec::new();

    for (idx, (item, _slot)) in items.iter().enumerate() {
        let is_selected = focused && selected == idx;
        lines.push(Line::from(vec![
            selection_prefix(is_selected),
            lock_prefix(item),
            item_display(item, None),
        ]));
    }

    if lines.is_empty() {
        lines.push(Line::from(Span::styled("(empty)", Style::default().color(colors::GREY))));
    }

    frame.render_widget(Paragraph::new(lines), chunks[1]);
}
