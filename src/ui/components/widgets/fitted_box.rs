
use ratatui::{layout::Rect, style::Style, widgets::{Block, Borders, Widget}};

pub struct FittedBox<W: Widget> {
    widget: W,
    width: u16,
    height: u16,
    title: Option<String>,
    title_style: Style,
}

impl<W: Widget> FittedBox<W> {
    pub fn new(widget: W, width: u16, height: u16) -> Self {
        Self {
            widget,
            width,
            height,
            title: None,
            title_style: Style::default(),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn title_style(mut self, style: Style) -> Self {
        self.title_style = style;
        self
    }

    pub fn height(&self) -> u16 {
        self.height + 2
    }
}

impl<W: Widget> Widget for FittedBox<W> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        let title_width = self.title.as_ref().map(|t| t.len()).unwrap_or(0) as u16;
        let width = (self.width + 2).max(title_width + 2);
        let height = self.height + 2;

        let fitted_area = Rect::new(
            area.x,
            area.y,
            width.min(area.width),
            height.min(area.height),
        );

        let mut block = Block::default().borders(Borders::ALL);
        if let Some(t) = self.title {
            block = block.title(t).title_style(self.title_style);
        }

        let inner_area = block.inner(fitted_area);
        block.render(fitted_area, buf);
        self.widget.render(inner_area, buf);
    }
}
