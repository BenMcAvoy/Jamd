#![allow(dead_code)]

use crate::text;

use super::Diagnostic;

pub struct Printer<'a> {
    text: &'a text::Source,
    diagnostics: &'a [Diagnostic],
}

impl<'a> Printer<'a> {
    pub const fn new(text: &'a text::Source, diagnostics: &'a [Diagnostic]) -> Self {
        Self { text, diagnostics }
    }

    pub fn stringify_diagnostic(&self, diagnostic: &Diagnostic) -> Result<String, ()> {
        let index = self.text.line_index(diagnostic.span.start);
        let line = self.text.get_line(index);
        let line_start = self.text.line_start(index);

        let column = diagnostic.span.start - line_start;
        let prefix_start: usize = std::cmp::max(0, column as isize - 2) as usize;

        todo!();
    }
}
