use crate::{interner::Symbol, span::Span, ty::Ty};

#[derive(Debug, Clone)]
pub struct Script {
    pub root: Section,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: Spanned<Symbol>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Fn(FnItem),
    Struct(StructItem),
    Enum(EnumItem),
    Section(SectionItem),
    Const(ConstItem),
    Define(DefineItem),
    Open(OpenItem),
    // tbc
}
#[derive(Debug, Clone)]
pub struct OpenItem {
    pub path: Vec<Spanned<Symbol>>,
    pub alias: Option<Spanned<Symbol>>,
    pub imports: Vec<Spanned<ImportItem>>,
}

#[derive(Debug, Clone)]
pub enum ImportItem {
    Star,
    SelfImport,
    Ident(Symbol),
}

#[derive(Debug, Clone)]
pub struct FnItem {
    pub name: Spanned<Symbol>,
    pub params: Vec<Spanned<Field>>,
    pub ret: Option<Spanned<Ty>>,
    pub body: BlockExpr,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: Spanned<Symbol>,
    pub ty: Spanned<Ty>,
}

#[derive(Debug, Clone)]
pub struct StructItem {
    pub name: Spanned<Symbol>,
    pub fields: Vec<Spanned<Field>>,
}

#[derive(Debug, Clone)]
pub struct EnumItem {
    pub name: Spanned<Symbol>,
    pub variants: Vec<Spanned<EnumVariants>>,
}

#[derive(Debug, Clone)]
pub enum EnumVariants {
    Tuple {
        name: Spanned<Symbol>,
        types: Vec<Spanned<Ty>>,
    },
    Struct {
        name: Spanned<Symbol>,
        fields: Vec<Spanned<Field>>,
    },
}

#[derive(Debug, Clone)]
pub struct SectionItem {
    pub sections: Spanned<Section>,
}

#[derive(Debug, Clone)]
pub struct ConstItem {
    pub name: Spanned<Symbol>,
    pub ty: Spanned<Ty>,
    pub value: Spanned<Expr>,
}

#[derive(Debug, Clone)]
pub enum DefineItem {
    Fn(FnItem),
    Const(ConstItem),
}

pub type SpannedBox<T> = Spanned<Box<T>>;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(Symbol),
    Range {
        start: Option<SpannedBox<Expr>>,
        end: Option<SpannedBox<Expr>>,
        inclusive: bool,
    },
    Binary(SpannedBox<Expr>, Spanned<BinaryOp>, SpannedBox<Expr>),
    Postfix(SpannedBox<Expr>, Spanned<PostfixOp>),
    Prefix(Spanned<PrefixOp>, SpannedBox<Expr>),
    Call(SpannedBox<Expr>, Vec<Spanned<Expr>>),
    Field(SpannedBox<Expr>, Spanned<Symbol>),
    MethodCall {
        receiver: SpannedBox<Expr>,
        method: Spanned<Symbol>,
        args: Vec<Spanned<Expr>>,
    },
    Index(SpannedBox<Expr>, SpannedBox<Expr>),
    If {
        cond: SpannedBox<Expr>,
        then: SpannedBox<Expr>,
        else_: Option<SpannedBox<Expr>>,
    },
    Loop(Spanned<BlockExpr>),
    While {
        cond: SpannedBox<Expr>,
        body: BlockExpr,
    },
    Break(Option<SpannedBox<Expr>>),
    Continue,
    Return(Option<SpannedBox<Expr>>),
    Block(Spanned<BlockExpr>),
    Assign {
        target: SpannedBox<Expr>,
        value: SpannedBox<Expr>,
    },
    AssignEq {
        op: Spanned<BinaryOp>,
        target: SpannedBox<Expr>,
        value: SpannedBox<Expr>,
    },
    StructInit {
        name: Spanned<Symbol>,
        fields: Vec<Spanned<Field>>,
    },
    TupleInit {
        name: Spanned<Symbol>,
        tys: Vec<Spanned<Ty>>,
    },
    Match {
        scrutinee: SpannedBox<Expr>,
        arms: Vec<MatchArm>,
    },
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Spanned<Pattern>,
    pub body: Expr,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Wildcard,
    Literal(Spanned<Literal>),
    Variable(Spanned<Symbol>),
    Tuple {
        name: Spanned<Symbol>,
        tys: Vec<Spanned<Pattern>>,
    },
    Struct {
        name: Spanned<Symbol>,
        fields: Vec<Spanned<Field>>,
    },
    // tbc
}

#[derive(Debug, Clone)]
pub struct BlockExpr {
    pub stmts: Vec<Stmt>,
    pub expr: Option<SpannedBox<Expr>>,
}

#[derive(Debug, Clone)]
pub enum Literal {
    UInt(u64),
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(Symbol),
    None,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {}
#[derive(Debug, Clone)]
pub enum PrefixOp {}
#[derive(Debug, Clone)]
pub enum PostfixOp {}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: Spanned<Symbol>,
        ty: Option<Spanned<Ty>>,
        value: Option<Expr>,
    },
    Open(OpenItem),
    Expr(Expr),
    Semi(Expr),
}

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

pub trait Spannable: Sized {
    fn spanned(self, span: Span) -> Spanned<Self>;
}

impl<T: Sized> Spannable for T {
    fn spanned(self, span: Span) -> Spanned<Self> {
        Spanned { node: self, span }
    }
}
