#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    /// =, +=, -=, etc
    Assignment,
    /// 1..2, 1..=3, [1..], [..2], [2..3]
    Range,
    /// ||
    LogicalOr,
    /// &&
    LogicalAnd,
    /// |
    BitwiseOr,
    /// ^
    BitwiseXor,
    /// &
    BitwiseAnd,
    /// ==, !=
    Equality,
    /// <, >, <=, >=
    Comparison,
    /// <<, >>
    Shift,
    /// +, -
    Addition,
    /// * /
    Multiplication,
    /// as
    As,
    /// : Type
    Cast,
    /// -x, !x, *x, &x
    Prefix,
    /// --, ++
    Postfix,
    /// expr.field, expr.method()
    FieldAccess,
    /// expr()
    Call,
    /// expr[]
    Index,
    /// open section::type
    Path,
    /// literals, idents
    Primary,
}
