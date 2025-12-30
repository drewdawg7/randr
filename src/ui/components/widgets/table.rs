use tuirealm::{Component, Event, MockComponent, Frame, command::{Cmd, CmdResult}, props::{Attribute, AttrValue, Props}, State, NoUserEvent};
use ratatui::widgets::{Table as RatatuiTable, Row as RatatuiRow, Cell};
use ratatui::layout::{Rect, Constraint};
use ratatui::style::{Style, Modifier};

use crate::ui::theme::{self as colors, ColorExt};

pub struct Header {
    pub label: String,
}

impl Header {
    pub fn new(label: &str) -> Self {
        Self { label: label.into() }
    }
}

pub struct Row {
    pub cells: Vec<String>,
}

impl Row {
    pub fn new<I, T>(cells: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: ToString,
    {
        Self {
            cells: cells.into_iter().map(|v| v.to_string()).collect(),
        }
    }
}

pub struct Table {
    props: Props,
    headers: Vec<Header>,
    rows: Vec<Row>,
}

impl Table {

    pub fn from_items<T, F, R, const N: usize>(
        headers: [Header; N],
        items: &[T],
        mut extract: F,
    ) -> Self
    where
        F: FnMut(&T) -> R,
        R: IntoIterator,
        R::Item: ToString,
    {
        let rows: Vec<Row> = items
            .iter()
            .map(|item| Row::new(extract(item)))
            .collect();

        Self {
            props: Props::default(),
            headers: headers.into(),
            rows,
        }
    }


    fn compute_widths(&self) -> Vec<Constraint> {
        let mut widths: Vec<usize> = self.headers.iter().map(|h| h.label.len()).collect();

        for row in &self.rows {
            for (i, cell) in row.cells.iter().enumerate() {
                if i >= widths.len() {
                    widths.push(cell.len());
                } else {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }

        widths
            .into_iter()
            .map(|w| Constraint::Length((w + 2) as u16))
            .collect()
    }

    pub fn content_width(&self) -> u16 {
        let mut widths: Vec<usize> = self.headers.iter().map(|h| h.label.len()).collect();
        for row in &self.rows {
            for (i, cell) in row.cells.iter().enumerate() {
                if i >= widths.len() {
                    widths.push(cell.len());
                } else {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }
        // Sum column widths + padding (2 per column) + separators
        widths.iter().map(|w| w + 2).sum::<usize>() as u16 + 2
    }

    pub fn content_height(&self) -> u16 {
        // header + rows
        (1 + self.rows.len()) as u16
    }

    pub fn to_widget(&self) -> RatatuiTable<'_> {
        let header_cells: Vec<Cell> = self
            .headers
            .iter()
            .map(|h| {
                Cell::from(h.label.clone())
                    .style(Style::default().color(colors::YELLOW).add_modifier(Modifier::BOLD))
            })
            .collect();

        let header_row = RatatuiRow::new(header_cells).height(1);

        let rows: Vec<RatatuiRow> = self
            .rows
            .iter()
            .map(|row| {
                let cells: Vec<Cell> = row
                    .cells
                    .iter()
                    .map(|c| Cell::from(c.clone()))
                    .collect();
                RatatuiRow::new(cells)
            })
            .collect();

        let widths = self.compute_widths();

        RatatuiTable::new(rows, &widths).header(header_row)
    }
}

impl MockComponent for Table {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self.to_widget(), area);
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

impl Component<Event<NoUserEvent>, NoUserEvent> for Table {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        None
    }
}
