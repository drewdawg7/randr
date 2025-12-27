use ratatui::{style::{Style, Stylize}, text::{Line, Span}, widgets::Paragraph};
use tuirealm::{command::CmdResult, MockComponent, State};

use crate::{combat::Named, entities::Player};




pub struct MainMenuHeader<'a> {
    pub player: &'a Player
}



impl<'a> MainMenuHeader<'a> {
    pub fn new(player: &'a Player) -> Self {
        Self { player }
    }
}


impl<'a> MockComponent for MainMenuHeader<'a> {
    fn view(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let player_name = self.player.name();
        let style = Style::default().bold().green().underlined();
        let line = Line::from(vec![
            Span::raw("Hello, "),
            Span::styled(player_name, style)
        ]);
        let paragraph = Paragraph::new(line);
        frame.render_widget(paragraph, area);

    }

    fn attr(&mut self, _attr: tuirealm::Attribute, _value: tuirealm::AttrValue) { }
    fn state(&self) -> tuirealm::State {State::None}
    fn query(&self, _attr: tuirealm::Attribute) -> Option<tuirealm::AttrValue> { None }
    fn perform(&mut self, _cmd: tuirealm::command::Cmd) -> tuirealm::command::CmdResult {CmdResult::None}
}
