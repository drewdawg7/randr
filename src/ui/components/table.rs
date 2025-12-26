use tuirealm::{Component, Event, MockComponent, Frame, command::{Cmd, CmdResult}, props::{Attribute, AttrValue}, State, NoUserEvent};
use ratatui::widgets::{Table as RatatuiTable, Row as RatatuiRow, Cell, Block, Borders};
use ratatui::layout::{Rect, Constraint};
use ratatui::style::{Style, Color, Modifier};

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

pub struct TableComponent {
    headers: Vec<Header>,
    rows: Vec<Row>,
}

impl TableComponent {
    pub fn new(headers: Vec<Header>) -> Self {
        Self {
            headers,
            rows: Vec::new(),
        }
    }

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
            headers: headers.into(),
            rows,
        }
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
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
}

impl MockComponent for TableComponent {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        let header_cells: Vec<Cell> = self
            .headers
            .iter()
            .map(|h| {
                Cell::from(h.label.clone())
                    .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
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

        let table = RatatuiTable::new(rows, &widths)
            .header(header_row)
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(table, area);
    }

    fn query(&self, _attr: Attribute) -> Option<AttrValue> {
        None
    }

    fn attr(&mut self, _attr: Attribute, _value: AttrValue) {}

    fn state(&self) -> State {
        State::None
    }

    fn perform(&mut self, _cmd: Cmd) -> CmdResult {
        CmdResult::None
    }
}

impl Component<Event<NoUserEvent>, NoUserEvent> for TableComponent {
    fn on(&mut self, _ev: Event<NoUserEvent>) -> Option<Event<NoUserEvent>> {
        None
    }
}
