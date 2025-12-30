use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::ui::theme::{self as colors, ColorExt};
use tuirealm::{
    command::{Cmd, CmdResult},
    props::{AttrValue, Attribute, Props},
    Component, Event, MockComponent, NoUserEvent, State,
};

use crate::{
    system::game_state,
    ui::Id,
};
use crate::ui::components::utilities::{CROSSED_SWORDS, RETURN_ARROW};
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
                        let combat_rounds = crate::combat::system::enter_combat(&mut gs.player, &mut mob);

                        // Add dropped loot to player inventory
                        for item_kind in &combat_rounds.dropped_loot {
                            let item = gs.spawn_item(*item_kind);
                            let _ = crate::inventory::HasInventory::add_to_inv(&mut gs.player, item);
                        }

                        gs.set_current_combat(combat_rounds);
                        gs.current_screen = Id::Fight;
                    }
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

fn field_header() -> Line<'static> {
    let field = &game_state().town.field;
    Line::from(vec![
        Span::styled(field.name.clone(), Style::default().color(colors::CYAN)),
    ])
}

impl MockComponent for FieldTab {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(0),
            ])
            .split(area);

        // Render header with field name
        let header_line = field_header();
        frame.render_widget(Paragraph::new(header_line), chunks[0]);

        // Render the menu
        self.menu.view(frame, chunks[1]);
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
