use std::{ops::ControlFlow, str::Chars};

use crate::{
    diagnostics::{Diagnostic, DiagnosticsBag, Reportable},
    source::{FileId, Source},
    span::Span,
    tokens::{Lexicable, Token, TokenKind},
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
                '.' => {
                    if self.first().is_ascii_digit() {
                        match self.float_suffix() {
                            ControlFlow::Continue(_) => continue,
                            ControlFlow::Break(t) => t,
                        }
                    } else if self.accept('.') {
                        if self.accept('.') {
                            TokenKind::DotDotDot
                        } else if self.accept('=') {
                            TokenKind::DotDotEq
                        } else {
                            TokenKind::DotDot
                        }
                    } else {
                        TokenKind::Dot
                    }
                }
                ',' => TokenKind::Comma,
                ':' => {
                    if self.accept(':') {
                        TokenKind::ColonColon
                    } else {
                        TokenKind::Colon
                    }
                }
                ';' => TokenKind::Semi,
                '?' => TokenKind::Question,

                '(' => TokenKind::LParen,
                ')' => TokenKind::RParen,
                '{' => TokenKind::LBrace,
                '}' => TokenKind::RBrace,
                '[' => TokenKind::LBracket,
                ']' => TokenKind::RBracket,

                '=' => {
                    if self.accept('=') {
                        TokenKind::EqEq
                    } else {
                        TokenKind::Eq
                    }
                }
                '!' => {
                    if self.accept('=') {
                        TokenKind::BangEq
                    } else {
                        TokenKind::Bang
                    }
                }

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
                '0'..='9' => match self.numeric_literals(char) {
                    ControlFlow::Continue(_) => continue,
                    ControlFlow::Break(t) => t,
                },
                '\'' => match self.character_literal() {
                    ControlFlow::Continue(_) => continue,
                    ControlFlow::Break(t) => t,
                },
                '"' => match self.string_literal() {
                    ControlFlow::Continue(_) => continue,
                    ControlFlow::Break(t) => t,
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.bump_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'));

                    TokenKind::correspond(&self.content[self.start..self.position()])
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

    fn bump_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while !self.is_eof() && predicate(self.first()) {
            self.bump();
        }
    }

    fn skip_ws(&mut self) {
        self.bump_while(|c| matches!(c, ' ' | '\r' | '\t' | '\n'));
    }

    fn check(&self, expected: char) -> bool {
        self.first() == expected
    }

    fn accept(&mut self, expected: char) -> bool {
        if self.first() == expected {
            self.bump();
            true
        } else {
            false
        }
    }

    fn float_suffix(&mut self) -> ControlFlow<TokenKind> {
        self.bump_while(|x| x.is_ascii_digit());

        if matches!(self.first(), 'e' | 'E') {
            self.bump();
            if matches!(self.first(), '+' | '-') {
                self.bump();
            }

            if !self.first().is_ascii_digit() {
                self.bag.push(
                    Diagnostic::error("syntax error")
                        .with_label(self.span().primary("invalid float literal"))
                        .with_label(
                            Span::new(self.id, self.position(), self.position() + 1)
                                .secondary("expected digit after float literal exponent"),
                        ),
                );
                return ControlFlow::Continue(());
            }

            self.bump_while(|x| x.is_ascii_digit());
        }

        ControlFlow::Break(TokenKind::FloatLiteral)
    }

    fn numeric_literals(&mut self, cur: char) -> ControlFlow<TokenKind> {
        let mut base = 10;

        if cur == '0' {
            match self.first().to_ascii_lowercase() {
                'x' => {
                    self.bump();
                    base = 16;
                }
                'b' => {
                    self.bump();
                    base = 2;
                }
                x if x.is_digit(8) => {
                    base = 8;
                }
                _ => (),
            }
        }

        self.bump_while(|c| c.is_digit(base as u32));

        if matches!(self.first(), |'e'| 'E') || self.accept('.') {
            self.float_suffix()
        } else {
            ControlFlow::Break(TokenKind::IntLiteral)
        }
    }

    fn character_literal(&mut self) -> ControlFlow<TokenKind> {
        self.start = self.position();
        match self.first() {
            '\'' => {
                self.bump();
                self.bag.push(
                    Diagnostic::error("syntax error")
                        .with_label(self.span().primary("char literal cannot be empty")),
                );
            }
            '\n' => {
                self.bag.push(
                    Diagnostic::error("syntax error")
                        .with_label(self.span().primary("char literal may not contain new line")),
                );
            }
            '\\' => {
                self.bump();
                if self.bump().normalize().is_none() {
                    self.bag.push(
                        Diagnostic::error("syntax error")
                            .with_label(self.span().primary("invalid char literal escape '{}'")),
                    );
                }
            }
            _ => {
                self.bump();
            }
        }

        if !self.accept('\'') {
            self.bag.push(
                Diagnostic::error("syntax error")
                    .with_label(self.span().primary("expected closing char quote")),
            );
        }

        self.start -= 1;
        ControlFlow::Break(TokenKind::CharLiteral)
    }

    fn string_literal(&mut self) -> ControlFlow<TokenKind> {
        while !self.is_eof() && !matches!(self.first(), '"') {
            match self.first() {
                '\n' => {
                    self.bag.push(
                        Diagnostic::error("syntax error")
                            .with_label(self.span().primary("strings cannot contain new line")),
                    );
                    break;
                }
                '\\' => {
                    self.bump();
                    if self.first().normalize().is_none() {
                        self.bag.push(
                            Diagnostic::error("syntax error").with_label(
                                self.span().primary("invalid char literal escape '{}'"),
                            ),
                        );
                    }
                }
                _ => (),
            }
            self.bump();
        }

        self.bump_while(|c| !matches!(c, '"'));
        if self.check('\0') || !self.accept('"') {
            self.bag.push(
                Diagnostic::error("syntax error")
                    .with_label(self.span().primary("unexpected end of file within literal")),
            );
        }

        ControlFlow::Break(TokenKind::StringLiteral)
    }
}
