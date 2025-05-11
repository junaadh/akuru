use std::str::Chars;

use crate::{
    source::{FileId, Source},
    tokens::TokenKind,
};

#[derive(Debug, Clone)]
pub struct Lexer<'src> {
    pub id: FileId,
    pub content: &'src str,
    pub chars: Chars<'src>,
    pub start: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(id: FileId, source: &'src Source) -> Self {
        let content = source.content.as_str();
        let chars = content.chars();

        Self {
            id,
            content,
            chars,
            start: 0,
        }
    }

    fn next_kind(&mut self) -> TokenKind {
        self.skip_ws();
        self.start = self.position();

        let char = self.bump();

        todo!()
    }

    fn bump(&mut self) -> char {
        self.chars.next().unwrap_or('\0')
    }

    fn first(&self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    fn second(&self) -> char {
        let mut cloned = self.chars.clone();
        cloned.next();
        cloned.next().unwrap_or('\0')
    }

    fn third(&self) -> char {
        let mut cloned = self.chars.clone();
        cloned.next();
        cloned.next();
        cloned.next().unwrap_or('\0')
    }

    fn position(&self) -> usize {
        self.content.len() - self.chars.as_str().len()
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn bump_while(&mut self, predicate: fn(char) -> bool) {
        while self.is_eof() && predicate(self.first()) {
            self.bump();
        }
    }

    fn skip_ws(&mut self) {
        self.bump_while(|c| matches!(c, ' ' | '\r' | '\t' | '\n'));
    }
}
