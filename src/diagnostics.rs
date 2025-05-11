use std::{collections::BTreeMap, fmt};

use crate::{
    source::{FileId, Position, Source, SourceMap},
    span::Span,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelStyle {
    Primary,
    Secondary,
}

#[derive(Debug, Clone)]
pub struct Label {
    pub span: Span,
    pub message: Option<String>,
    pub style: LabelStyle,
}

impl Label {
    pub fn primary(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: Some(message.into()),
            style: LabelStyle::Primary,
        }
    }

    pub fn secondary(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: Some(message.into()),
            style: LabelStyle::Secondary,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticKind {
    Error,
    Warning,
    Note,
    Help,
}

impl DiagnosticKind {
    pub fn color(&self) -> &str {
        match self {
            Self::Error => "\x1b[31m",   // red
            Self::Warning => "\x1b[33m", // yellow
            Self::Note => "\x1b[34m",    // blue
            Self::Help => "\x1b[32m",    // green
        }
    }
}

pub const RESET: &str = "\x1b[0m";

impl fmt::Display for DiagnosticKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Note => write!(f, "note"),
            Self::Help => write!(f, "help"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub labels: Vec<Label>,
}

impl Diagnostic {
    pub fn error<S: Into<String>>(message: S) -> Self {
        Self {
            kind: DiagnosticKind::Error,
            message: message.into(),
            labels: Vec::new(),
        }
    }

    pub fn warning<S: Into<String>>(message: S) -> Self {
        Self {
            kind: DiagnosticKind::Warning,
            message: message.into(),
            labels: Vec::new(),
        }
    }

    pub fn help<S: Into<String>>(message: S) -> Self {
        Self {
            kind: DiagnosticKind::Help,
            message: message.into(),
            labels: Vec::new(),
        }
    }

    pub fn note<S: Into<String>>(message: S) -> Self {
        Self {
            kind: DiagnosticKind::Note,
            message: message.into(),
            labels: Vec::new(),
        }
    }

    pub fn with_label(mut self, label: Label) -> Self {
        self.labels.push(label);
        self
    }

    pub fn with_labels<I: IntoIterator<Item = Label>>(mut self, labels: I) -> Self {
        self.labels.extend(labels);
        self
    }

    pub fn can_merge(&mut self, rhs: &Diagnostic) -> bool {
        self.kind == rhs.kind && self.message == rhs.message
    }

    pub fn merge(&mut self, rhs: Diagnostic) {
        self.labels.extend(rhs.labels);
        self.merge_labels();
    }

    fn merge_labels(&mut self) {
        self.labels.sort_by_key(|l| l.span.lo);

        let mut merged = Vec::<Label>::with_capacity(self.labels.len());

        for label in self.labels.drain(..) {
            if let Some(last) = merged.last_mut() {
                let can_merge = last.style == label.style
                    && last.message == label.message
                    && last.span.hi == label.span.lo;

                let is_duplicate = last.style == label.style && last.span == label.span;

                if is_duplicate {
                    continue;
                }

                if can_merge {
                    last.span += label.span;
                    continue;
                }
            }

            merged.push(label);
        }

        self.labels = merged;
    }
    /*

    pub fn render(&self, sm: &SourceMap) {
        let color = self.kind.color();
        eprintln!("{}{}:{}{}", color, self.kind, RESET, self.message);

        for label in self.labels.iter() {
            let source = &sm[label.span.id];
            let pos = source.get_pos(label.span);

            match pos {
                Position::Single(line, col) => {
                    self.print_line(source, line, col, label, label.span.len(), true)
                }
                Position::Multi { lines } => {
                    let last_idx = lines.len() - 1;
                    for (i, &pos) in lines.iter().enumerate() {
                        let is_last = i == last_idx;
                        self.print_line(source, pos.0, pos.1, label, 1, is_last);
                    }
                }
            }
        }
    }

    fn print_line(
        &self,
        src: &Source,
        line_no: usize,
        col_no: usize,
        label: &Label,
        len: usize,
        show_message: bool,
    ) {
        let line_idx = line_no - 1;
        let line_start = src.line_offsets[line_idx];
        let line_end = src
            .line_offsets
            .get(line_idx + 1)
            .copied()
            .unwrap_or(src.content.len());
        let line_content = &src.content[line_start..line_end];

        eprintln!("{:>4} | {}", line_no, line_content.trim_end());

        print!("     | {}", " ".repeat(col_no.saturating_sub(1)));

        let underline = "^".repeat(len.max(1));
        let color = match label.style {
            LabelStyle::Primary => "\x1b[91m",   // bright red
            LabelStyle::Secondary => "\x1b[94m", // bright blue
        };
        print!("{color}{}{RESET}", underline);

        // Optional message
        if show_message {
            if let Some(ref msg) = label.message {
                println!(" {msg}");
                return;
            }
        }
        println!();
    }
    */
    pub fn render(&self, sm: &SourceMap) {
        let color = self.kind.color();
        eprintln!("{}{}:{}{}", color, self.kind, RESET, self.message);

        let mut line_map: BTreeMap<(FileId, usize), Vec<&Label>> = BTreeMap::new();

        for label in &self.labels {
            let source = &sm[label.span.id];

            match source.get_pos(label.span) {
                Position::Single(line, _col) => {
                    line_map
                        .entry((label.span.id, line))
                        .or_default()
                        .push(label);
                }
                Position::Multi { lines } => {
                    for (line, _col) in lines {
                        line_map
                            .entry((label.span.id, line))
                            .or_default()
                            .push(label);
                    }
                }
            }
        }

        // For each line (sorted)
        for ((file_id, line_no), labels) in line_map {
            let source = &sm[file_id];
            self.print_labels_on_line(source, line_no, labels);
        }
    }

    fn print_labels_on_line(&self, src: &Source, line_no: usize, labels: Vec<&Label>) {
        let line_idx = line_no - 1;
        let line_start = src.line_offsets[line_idx];
        let line_end = src
            .line_offsets
            .get(line_idx + 1)
            .copied()
            .unwrap_or(src.content.len());
        let line_content = &src.content[line_start..line_end];

        // Print source line
        eprintln!("{:>4} | {}", line_no, line_content.trim_end());

        // Now print underline lines, one for each label
        for label in labels {
            // Column in this line
            let pos = src.get_pos(label.span);
            let col = match pos {
                Position::Single(_, c) => c,
                Position::Multi { ref lines } => {
                    // Try to find the line in question
                    if let Some((_, c)) = lines.iter().find(|(line, _)| *line == line_no) {
                        *c
                    } else {
                        continue;
                    }
                }
            };

            let len = label.span.len().max(1);

            print!("     | {}", " ".repeat(col.saturating_sub(1)));

            let underline = "^".repeat(len);
            let color = match label.style {
                LabelStyle::Primary => "\x1b[91m",   // bright red
                LabelStyle::Secondary => "\x1b[94m", // bright blue
            };
            print!("{color}{}{RESET}", underline);

            if let Some(ref msg) = label.message {
                print!(" {msg}");
            }

            println!();
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DiagnosticsBag(pub Vec<Diagnostic>);

impl DiagnosticsBag {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, diagnostic: Diagnostic) {
        if let Some(last) = self.0.last_mut() {
            if last.can_merge(&diagnostic) {
                last.merge(diagnostic);
                return;
            }
        }

        self.0.push(diagnostic);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn render_all(self, sm: &SourceMap) {
        for diag in self.0 {
            diag.render(sm);
        }
    }
}
