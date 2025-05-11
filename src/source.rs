use std::{fs, ops::Index};

use crate::span::Span;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Source {
    pub name: String,
    pub content: String,
    pub line_offsets: Vec<usize>,
}

impl Source {
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        let content = fs::read_to_string(&name).expect("failed to read file");
        let line_offsets = std::iter::once(0)
            .chain(content.as_str().match_indices('\n').map(|(i, _)| i + 1))
            .collect();

        Self {
            name,
            content,
            line_offsets,
        }
    }

    pub fn with_content(name: impl Into<String>, content: impl Into<String>) -> Self {
        let name = name.into();
        let content = content.into();
        let line_offsets = std::iter::once(0)
            .chain(content.as_str().match_indices('\n').map(|(i, _)| i + 1))
            .collect();

        Self {
            name,
            content,
            line_offsets,
        }
    }

    pub fn get_pos(&self, span: Span) -> Position {
        let start_line = self.get_offset_line(span.lo);
        let end_line = self.get_offset_line(span.hi);

        if start_line == end_line {
            let col = span.lo - self.line_offsets[start_line];
            Position::Single(start_line + 1, col + 1)
        } else {
            let lines = (start_line..=end_line)
                .map(|line| {
                    let line_start = self.line_offsets[line];
                    let col = match line {
                        l if l == start_line => span.lo - line_start,
                        l if l == end_line => span.hi - line_start,
                        _ => 0,
                    };
                    (line + 1, col + 1)
                })
                .collect();
            Position::Multi { lines }
        }
    }

    fn get_offset_line(&self, offset: usize) -> usize {
        match self.line_offsets.binary_search(&offset) {
            Ok(idx) => idx,
            Err(idx) => idx - 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileId(pub u32);

pub struct SourceMap {
    pub source: Vec<Source>,
}

impl SourceMap {
    pub fn fresh() -> Self {
        Self { source: Vec::new() }
    }

    pub fn insert(&mut self, name: impl Into<String>) -> FileId {
        let source = Source::new(name);
        self.source.push(source);
        FileId(self.source.len() as u32 - 1)
    }

    pub fn with_content(&mut self, name: impl Into<String>, content: impl Into<String>) -> FileId {
        let source = Source::with_content(name, content);
        self.source.push(source);
        FileId(self.source.len() as u32 - 1)
    }
}

impl Index<FileId> for SourceMap {
    type Output = Source;

    fn index(&self, index: FileId) -> &Self::Output {
        &self.source[index.0 as usize]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Position {
    Single(usize, usize),
    Multi { lines: Vec<(usize, usize)> },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_offsets_single_line() {
        let content = "hello world";
        let source = Source::with_content("test.txt", content);

        assert_eq!(source.line_offsets, vec![0]);
    }

    #[test]
    fn test_line_offsets_multi_line() {
        let content = "first line\nsecond line\nthird line\n";
        let source = Source::with_content("multi.txt", content);

        assert_eq!(source.line_offsets, vec![0, 11, 23, 34]);
    }

    #[test]
    fn test_get_pos_single_line() {
        let content = "hello world\nsecond line\n";
        let source = Source::with_content("pos.txt", content);
        let span = Span {
            id: FileId(0),
            lo: 6,
            hi: 9,
        };

        let pos = source.get_pos(span);
        assert_eq!(pos, Position::Single(1, 7));
    }

    #[test]
    fn test_get_pos_multi_line() {
        let content = "line 1\nline 2\nline 3\n";
        let source = Source::with_content("pos_multi.txt", content);
        // span from "1\nline 2\nli"
        let span = Span {
            id: FileId(0),
            lo: 5,
            hi: 18,
        };

        let pos = source.get_pos(span);

        assert_eq!(
            pos,
            Position::Multi {
                lines: vec![(1, 6), (2, 1), (3, 5)]
            }
        );
    }

    #[test]
    fn test_source_map_insert() {
        let mut map = SourceMap::fresh();
        let id = map.with_content("a.rs", "fn main() {}\nlet x = 5;");

        let source = &map[id];
        assert_eq!(source.name, "a.rs");
        assert_eq!(source.content, "fn main() {}\nlet x = 5;");
        assert_eq!(source.line_offsets, vec![0, 13]);
    }

    #[test]
    fn test_get_offset_line_binary_search() {
        let content = "a\nb\nc\nd\n";
        let source = Source::with_content("offset.rs", content);
        // line offsets should be [0, 2, 4, 6, 8]

        assert_eq!(source.get_offset_line(0), 0);
        assert_eq!(source.get_offset_line(1), 0);
        assert_eq!(source.get_offset_line(2), 1);
        assert_eq!(source.get_offset_line(3), 1);
        assert_eq!(source.get_offset_line(7), 3);
    }

    #[test]
    fn test_get_pos_exact_line_boundaries() {
        let content = "a\nb\nc\nd\n";
        let source = Source::with_content("pos.rs", content);
        let span = Span {
            id: FileId(0),
            lo: 2,
            hi: 4,
        }; // starts at 'b', ends at 'c'

        let pos = source.get_pos(span);
        assert_eq!(
            pos,
            Position::Multi {
                lines: vec![(2, 1), (3, 1)]
            }
        );
    }

    #[test]
    fn test_empty_source() {
        let source = Source::with_content("empty.rs", "");
        assert_eq!(source.line_offsets, vec![0]);
    }
}
