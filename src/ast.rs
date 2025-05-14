use crate::{interner::Symbol, span::Span, ty::Ty};

pub struct Script {
    pub root: Section,
}

pub struct Section {
    pub name: Spanned<Symbol>,
    pub items: Vec<Item>,
}

pub enum Item {
    Fn(FnItem),
    Struct(StructItem),
    Enum(EnumItem),
    Section(SectionItem),
    Const(ConstItem),
    Define(DefineItem),
    // tbc
}

pub struct FnItem {
    pub name: Spanned<Symbol>,
    pub params: Vec<Spanned<Field>>,
    pub ret: Option<Spanned<Ty>>,
    pub body: BlockExpr,
}

pub struct Field {
    pub name: Spanned<Symbol>,
    pub ty: Spanned<Ty>,
}

pub struct StructItem {
    pub name: Spanned<Symbol>,
    pub fields: Vec<Spanned<Field>>,
}

pub struct EnumItem {
    pub name: Spanned<Symbol>,
    pub variants: Vec<Spanned<EnumVariants>>,
}

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

pub struct SectionItem {
    pub sections: Spanned<Section>,
}

pub struct ConstItem {
    pub name: Spanned<Symbol>,
    pub ty: Spanned<Ty>,
    pub value: Spanned<Expr>,
}

pub enum DefineItem {
    Fn(FnItem),
    Const(ConstItem),
}

pub enum Expr {}
pub struct BlockExpr {}

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
