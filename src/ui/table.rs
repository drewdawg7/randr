use std::io::{Stdout};

use crossterm::style::Print;

use crate::ui::print_to_screen;

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
    fn new<I, T>(cells: I) -> Self
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
    pub headers: Vec<Header>,
    pub rows: Vec<Row>
}


impl Table {
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
        let mut rows = Vec::with_capacity(items.len());

        for item in items {
            rows.push(Row::new(extract(item)));
        }

        Self {
            headers: headers.into(),
            rows,
        }
    }
    pub fn push_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn print(&self, stdout: &mut Stdout) {
        let widths = self.compute_widths();

        // header
        self.print_cells(
            stdout,
            self.headers.iter().map(|h| h.label.as_str()),
            &widths,
        );
        print_to_screen(stdout, Print("\n"));

        // separator
        self.print_separator(stdout, &widths);
        print_to_screen(stdout, Print("\n"));

        // rows
        for row in &self.rows {
            self.print_cells(
                stdout,
                row.cells.iter().map(String::as_str),
                &widths,
            );
            print_to_screen(stdout, Print("\n"));
        }
    }

    fn compute_widths(&self) -> Vec<usize> {
        let mut widths: Vec<usize> =
            self.headers.iter().map(|h| h.label.len()).collect();

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
    }

    fn print_cells<'a>(
        &self,
        stdout: &mut Stdout,
        cells: impl IntoIterator<Item = &'a str>,
        widths: &[usize],
    ) {
        for (i, cell) in cells.into_iter().enumerate() {
            let s = format!("{:<width$}", cell, width = widths[i]);
            print_to_screen(stdout, Print(s));

            if i + 1 < widths.len() {
                print_to_screen(stdout, Print(" | "));
            }
        }
    }

    fn print_separator(&self, stdout: &mut Stdout, widths: &[usize]) {
        for (i, w) in widths.iter().enumerate() {
            print_to_screen(stdout, Print("-".repeat(*w)));
            if i + 1 < widths.len() {
                print_to_screen(stdout, Print("-+-"));
            }
        }
    }
}
