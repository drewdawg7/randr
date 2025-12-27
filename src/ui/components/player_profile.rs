
use ratatui::{layout::{Constraint, Direction, Layout}, style::*, text::{Line, Span}, widgets::Paragraph};
use tuirealm::{command::CmdResult, Component, Event, MockComponent, NoUserEvent, State};

use crate::{combat::{HasGold, Named}, entities::progression::{HasProgression, Progression}, system::game_state, ui::{common::ScreenId, components::utilities::{back_button, COIN, CROSSED_SWORDS, HEART}, fittedbox::FittedBox, menu_component::MenuComponent}};

const FILLED_SEGMENT: char = '■';
const EMPTY_SEGMENT: char = '□';

fn xp_bar(progression: &Progression) -> Line<'static> {
    let current_xp = progression.xp;
    let xp_needed = Progression::xp_to_next_level(progression.level);
    let segments = 10;

    let filled = ((current_xp * segments) / xp_needed).min(segments) as usize;

    let mut spans = vec![Span::raw("[")];

    for i in 0..segments as usize {
        if i < filled {
            spans.push(Span::styled(
                FILLED_SEGMENT.to_string(),
                Style::default().fg(Color::Yellow).bold(),
            ));
        } else {
            spans.push(Span::styled(
                EMPTY_SEGMENT.to_string(),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }

    spans.push(Span::raw("]"));
    spans.push(Span::raw(format!(" {}/{}", current_xp, xp_needed)));

    Line::from(spans)
}




pub struct PlayerProfile{
    back_button: MenuComponent,
}

impl PlayerProfile {
    pub fn new() -> Self {
        Self { 
            back_button: back_button(ScreenId::Menu),
        }
    }
}


impl MockComponent for PlayerProfile {
    fn view(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let player = &game_state().player;

        let lines = vec![
            Line::from(vec![
                Span::styled(format!("{}", HEART), Style::default().red().bold()),
                Span::raw(" "),
                Span::raw(format!("{}/{}", player.get_health(), player.get_max_health()))
            ]),
            Line::from(vec![
                Span::styled(format!("{}", COIN), Style::default().yellow()),
                Span::raw(" "),
                Span::raw(format!("{}", player.gold()))
            ]),
            Line::from(vec![
                Span::styled(format!("{}", CROSSED_SWORDS), Style::default().white()),
                Span::raw(" "),
                Span::raw(format!("{}", player.get_attack()))
            ]),
            xp_bar(player.progression()),
        ];

        let width = lines.iter().map(|l| l.width()).max().unwrap_or(0) as u16;
        let height = lines.len() as u16;

        let profile_box = FittedBox::new(Paragraph::new(lines), width, height)
            .title(player.name())
            .title_style(Style::default().bold().green().on_dark_gray());

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(profile_box.height()),
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        frame.render_widget(profile_box, chunks[0]);
        self.back_button.view(frame, chunks[1]);
    }

    fn attr(&mut self, _attr: tuirealm::Attribute, _value: tuirealm::AttrValue) { }
    fn state(&self) -> tuirealm::State {State::None}
    fn query(&self, _attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> { None }
    fn perform(&mut self, _cmd: tuirealm::command::Cmd) -> tuirealm::command::CmdResult {CmdResult::None}
}


impl Component<Event<NoUserEvent>, NoUserEvent> for PlayerProfile {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        self.back_button.on(ev)
    }
}


impl Default for PlayerProfile {
    fn default() -> Self {
        Self { 
            back_button: back_button(ScreenId::Menu),
        }
    }
}
