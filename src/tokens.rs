use crate::span::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    Eof, // 0

    // Hash,     // #
    // Dollar,   // $
    Dot,      // .
    Comma,    // ,
    Colon,    // :
    Semi,     // ;
    Question, // ?

    LParen,   // (
    RParen,   // )
    LBrace,   // {
    RBrace,   // }
    LBracket, // [
    RBracket, // ]

    Plus,     // +
    Minus,    // -
    Star,     // *
    Slash,    // /
    Bang,     // !
    LShift,   // <<
    RShift,   // >>
    Pipe,     // |
    And,      // &
    Caret,    // ^
    PipePipe, // ||
    AndAnd,   // &&
    PlusPlus,
    MinusMinus,

    Eq,       // =
    PlusEq,   // +=
    MinusEq,  // -=
    StarEq,   // *=
    SlashEq,  // /=
    LShiftEq, // <<=
    RShiftEq, // >>=
    PipeEq,   // |=
    AndEq,    // &=
    CaretEq,  // ^=

    Lt,     // <
    Gt,     // >
    LtEq,   // <=
    GtEq,   // >=
    EqEq,   // ==
    BangEq, // !=

    If,
    Else,
    While,
    For,
    Loop,
    Fn,
    Return,
    Let,
    Const,
    Continue,
    True,
    False,
    Struct,
    Enum,
    Match,
    Break,

    IntLiteral,
    FloatLiteral,
    CharLiteral,
    StringLiteral,

    Ident, // identifier
}

impl TokenKind {
    #[inline]
    pub fn is_assignment(&self) -> bool {
        (Self::Eq..=Self::CaretEq).contains(self)
    }

    #[inline]
    pub fn is_comparitive(&self) -> bool {
        (Self::Lt..=Self::BangEq).contains(self)
    }

    #[inline]
    pub fn is_keyword(&self) -> bool {
        (Self::If..=Self::Break).contains(self)
    }

    #[inline]
    pub fn is_literal(&self) -> bool {
        (Self::IntLiteral..=Self::StringLiteral).contains(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    #[inline]
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn is_keyword(&self) -> bool {
        self.kind.is_keyword()
    }

    pub fn is_literal(&self) -> bool {
        self.kind.is_literal()
    }

    pub fn is_eof(&self) -> bool {
        self.kind == TokenKind::Eof
    }
}

pub trait Lexicable {
    fn normalize(&self) -> Option<char>;
}

impl Lexicable for char {
    fn normalize(&self) -> Option<char> {
        match self {
            'n' => Some('\n'),
            'r' => Some('\r'),
            't' => Some('\t'),
            'v' => Some('\x0b'),
            'b' => Some('\x08'),
            'a' => Some('\x07'),
            '0' => Some('\0'),
            _ => None,
        }
    }
}
