use ratatui::{layout::Rect, style::{Style, Stylize}, text::{Line, Span}, widgets::Paragraph, Frame};

use crate::ui::theme::{self as colors, ColorExt};
use tuirealm::{command::{Cmd, CmdResult}, props::{Attribute, AttrValue, Props}, Component, Event, MockComponent, NoUserEvent, State};

use crate::entities::progression::{HasProgression, Progression};
use crate::system::game_state;

const FILLED_SEGMENT: char = '■';
const EMPTY_SEGMENT: char = '□';

pub struct XpBar {
    props: Props,
}

impl XpBar {
    pub fn new() -> Self {
        Self { props: Props::default() }
    }
}

impl MockComponent for XpBar {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let progression = game_state().player.progression();
        let level = progression.level;
        let xp = progression.xp;

        let xp_needed = Progression::xp_to_next_level(level);
        let segments: i32 = 10;
        let filled = ((xp * segments) / xp_needed).min(segments) as usize;

        let mut spans = vec![Span::raw("[")];
        for i in 0..segments as usize {
            if i < filled {
                spans.push(Span::styled(FILLED_SEGMENT.to_string(), Style::default().color(colors::YELLOW).bold()));
            } else {
                spans.push(Span::styled(EMPTY_SEGMENT.to_string(), Style::default().color(colors::DARK_GRAY)));
            }
        }
        spans.push(Span::raw("]"));
        spans.push(Span::raw(format!(" {}/{}", xp, xp_needed)));

        frame.render_widget(Paragraph::new(Line::from(spans)), area);
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

impl Component<Event<NoUserEvent>, NoUserEvent> for XpBar {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        None
    }
}
