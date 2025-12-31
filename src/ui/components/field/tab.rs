use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    Frame,
};

use crate::ui::theme::{self as colors, ColorExt};
use tuirealm::{
    command::{Cmd, CmdResult},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State,
};

use crate::{
    combat::HasGold,
    entities::progression::HasProgression,
    stats::HasStats,
    system::game_state,
    ui::Id,
};
use crate::ui::components::utilities::{render_location_header, COIN, CROSSED_SWORDS, DOUBLE_ARROW_UP, HEART, PICKAXE, RETURN_ARROW};
use crate::ui::components::widgets::menu::{Menu, MenuItem};

pub struct FieldTab {
    props: Props,
    menu: Menu,
}

impl FieldTab {
    pub fn new() -> Self {
        let items = vec![
            MenuItem {
                label: format!("{} Fight", CROSSED_SWORDS),
                action: Box::new(move || {
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
                        gs.current_screen = Id::Fight;
                    }
                }),
            },
            MenuItem {
                label: format!("{} Mine", PICKAXE),
                action: Box::new(|| {
                    game_state().current_screen = Id::Mine;
                }),
            },
            MenuItem {
                label: format!("{} Back", RETURN_ARROW),
                action: Box::new(|| {
                    game_state().current_screen = Id::Menu;
                }),
            },
        ];

        Self {
            props: Props::default(),
            menu: Menu::new(items),
        }
    }
}

impl Default for FieldTab {
    fn default() -> Self {
        Self::new()
    }
}

fn field_header() -> Vec<Line<'static>> {
    use crate::entities::progression::Progression;

    let gs = game_state();
    let field = &gs.town.field;
    let player = &gs.player;

    // Get player stats
    let current_hp = player.hp();
    let max_hp = player.max_hp();
    let gold = player.gold();
    let progression = player.progression();
    let level = progression.level;
    let current_xp = progression.xp;
    let xp_to_next = Progression::xp_to_next_level(level);

    vec![
        // Line 1: Field name
        Line::from(vec![
            Span::styled(field.name.clone(), Style::default().color(colors::FOREST_GREEN)),
        ]),
        // Line 2: HP | Level XP | Gold
        Line::from(vec![
            Span::styled(format!("{} ", HEART), Style::default().color(colors::RED)),
            Span::raw(format!("{}/{}", current_hp, max_hp)),
            Span::raw("  |  "),
            Span::styled(format!("{} ", DOUBLE_ARROW_UP), Style::default().color(colors::CYAN)),
            Span::raw(format!("{} ", level)),
            Span::raw(format!("{}/{}", current_xp, xp_to_next)),
            Span::raw("  |  "),
            Span::styled(format!("{} ", COIN), Style::default().color(colors::YELLOW)),
            Span::raw(format!("{}", gold)),
        ]),
    ]
}

impl MockComponent for FieldTab {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Render header with field name and get remaining area
        let header_lines = field_header();
        let content_area = render_location_header(frame, area, header_lines, colors::FIELD_BG, colors::FOREST_GREEN);

        // Render the menu in remaining area
        self.menu.view(frame, content_area);
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        self.menu.state()
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        self.menu.perform(cmd)
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for FieldTab {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        self.menu.on(ev)
    }
}
