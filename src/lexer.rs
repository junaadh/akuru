use std::str::Chars;

use crate::{
    diagnostics::{Diagnostic, DiagnosticsBag, Reportable},
    source::{FileId, Source},
    span::Span,
    tokens::{Token, TokenKind},
};

#[derive(Debug, Clone)]
pub struct Lexer<'src> {
    pub id: FileId,
    pub content: &'src str,
    pub chars: Chars<'src>,
    pub start: usize,
    pub bag: DiagnosticsBag,
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
            bag: DiagnosticsBag::new(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        let kind;
        loop {
            self.skip_ws();
            self.start = self.position();

            let char = self.bump();

            kind = match char {
                '\0' => TokenKind::Eof,
                // '.' =>
                ',' => TokenKind::Comma,
                ':' => TokenKind::Colon,
                ';' => TokenKind::Semi,
                '?' => TokenKind::Question,

                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,
                '{' => TokenKind::LBrace,
                '}' => TokenKind::RBrace,
                '[' => TokenKind::LBracket,
                ']' => TokenKind::RBracket,

                '+' => {
                    if self.accept('+') {
                        TokenKind::PlusPlus
                    } else if self.accept('=') {
                        TokenKind::PlusEq
                    } else {
                        TokenKind::Plus
                    }
                }
                '-' => {
                    if self.accept('-') {
                        TokenKind::MinusMinus
                    } else if self.accept('=') {
                        TokenKind::MinusEq
                    } else {
                        TokenKind::Minus
                    }
                }
                '*' => {
                    if self.accept('=') {
                        TokenKind::StarEq
                    } else {
                        TokenKind::Star
                    }
                }
                '/' => {
                    if self.accept('=') {
                        TokenKind::SlashEq
                    } else if self.accept('/') {
                        self.bump_while(|c| matches!(c, '\n'));
                        continue;
                    } else {
                        TokenKind::Slash
                    }
                }
                '<' => {
                    if self.accept('<') {
                        if self.accept('=') {
                            TokenKind::LShiftEq
                        } else {
                            TokenKind::LShift
                        }
                    } else if self.accept('=') {
                        TokenKind::LtEq
                    } else {
                        TokenKind::Lt
                    }
                }
                '>' => {
                    if self.accept('>') {
                        if self.accept('=') {
                            TokenKind::RShiftEq
                        } else {
                            TokenKind::RShift
                        }
                    } else if self.accept('=') {
                        TokenKind::GtEq
                    } else {
                        TokenKind::Gt
                    }
                }
                '|' => {
                    if self.accept('|') {
                        TokenKind::PipePipe
                    } else if self.accept('=') {
                        TokenKind::PipeEq
                    } else {
                        TokenKind::Pipe
                    }
                }
                '&' => {
                    if self.accept('&') {
                        TokenKind::AndAnd
                    } else if self.accept('=') {
                        TokenKind::AndEq
                    } else {
                        TokenKind::And
                    }
                }
                '^' => {
                    if self.accept('=') {
                        TokenKind::CaretEq
                    } else {
                        TokenKind::Caret
                    }
                }

                _ => {
                    self.bag.push(
                        Diagnostic::error("syntax error")
                            .with_label(self.span().primary("unexpected token '{}'")),
                    );
                    continue;
                }
            };
            break;
        }

        Token::new(kind, self.span())
    }

    fn bump(&mut self) -> char {
        self.chars.next().unwrap_or('\0')
    }

    fn first(&self) -> char {
        self.chars.clone().next().unwrap_or('\0')
    }

    // fn second(&self) -> char {
    //     let mut cloned = self.chars.clone();
    //     cloned.next();
    //     cloned.next().unwrap_or('\0')
    // }
    // fn third(&self) -> char {
    //     let mut cloned = self.chars.clone();
    //     cloned.next();
    //     cloned.next();
    //     cloned.next().unwrap_or('\0')
    // }

    fn position(&self) -> usize {
        self.content.len() - self.chars.as_str().len()
    }

    #[inline]
    fn span(&self) -> Span {
        Span::new(self.id, self.start, self.position())
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn bump_while(&mut self, predicate: fn(char) -> bool) {
        while !self.is_eof() && predicate(self.first()) {
            self.bump();
        }
    }

    fn skip_ws(&mut self) {
        self.bump_while(|c| matches!(c, ' ' | '\r' | '\t' | '\n'));
    }

    // fn check(&self, expected: char) -> bool {
    //     self.first() == expected
    // }

    fn accept(&mut self, expected: char) -> bool {
        if self.first() == expected {
            self.bump();
            true
        } else {
            false
        }
    }

    // fn consume(&mut self, expected: char) -> bool {
    //     if !self.accept(expected) {
    //         self.bag.push(Diagnostic::error(format!(
    //             "expected char '{}', found '{{}}'",
    //             expected
    //         )));
    //         false
    //     } else {
    //         true
    //     }
    // }
}
