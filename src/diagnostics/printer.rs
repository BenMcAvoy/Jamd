#![allow(dead_code)]

use termion::color::{Fg, Red, Reset};

use crate::text;

use super::Diagnostic;
use std::cmp::{max, min};

pub struct Printer<'a> {
    text: &'a text::Source,
    diagnostics: &'a [Diagnostic],
}

const PREFIX_LENGTH: usize = 8;

impl<'a> Printer<'a> {
    pub const fn new(text: &'a text::Source, diagnostics: &'a [Diagnostic]) -> Self {
        Self { text, diagnostics }
    }

    /// Stringify the diagnostic.
    /// ## Format:
    /// let <red>x<reset> = 5;
    ///          │
    ///          └─ Error message here (<file>:<line>:<column>)
    ///
    pub fn stringify_diagnostic(&self, diagnostic: &Diagnostic) -> String {
        let index = self.text.line_index(diagnostic.span.start);
        let start = self.text.line_start(index);
        let line = self.text.get_line(index);

        // Do saturation for safety
        let column = diagnostic.span.start.saturating_sub(start);
        // let column = diagnostic.span.start - start;

        let prefix_start = max(0, column as isize - PREFIX_LENGTH as isize) as usize;
        let prefix_end = column;

        let suffix_start = min(column + diagnostic.span.length(), line.len()) + 1;
        let suffix_end = min(suffix_start + PREFIX_LENGTH, line.len());

        let prefix = &line[prefix_start..prefix_end];
        let span = &line[prefix_end..suffix_start];
        let suffix = &line[suffix_start..suffix_end];

        let indent = max(PREFIX_LENGTH, column);

        let arrow_pointers = format!(
            "{:indent$}{}",
            "",
            "│".repeat(diagnostic.span.length()),
            indent = indent
        );
        // let arrow_line = format!("{:indent$}│", "", indent = indent);

        let error_message = format!("{:indent$}└─ {}", "", diagnostic.message, indent = indent);

        // format!("{}{}{}{}{}\n{}\n{}\n{}", prefix, Fg(Red), span, Fg(Reset), suffix, arrow_pointers, arrow_line, error_message)
        // format!(
        //     "{prefix}{red}{span}{reset}{suffix}{red}\n{arrow_pointers}\n{error_message}{reset}",
        //     red = Fg(Red),
        //     reset = Fg(Reset)
        // )

        format!(
            "{prefix}{red}{span}{reset}{suffix}{red}\n{error_message}{reset}",
            red = Fg(Red),
            reset = Fg(Reset)
        )
    }

    pub fn print(&self) {
        for diagnostic in self.diagnostics {
            println!("{}", self.stringify_diagnostic(diagnostic));
        }
    }
}
