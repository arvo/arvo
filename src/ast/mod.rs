//! # Abstract Syntax Tree
//!
//! The abstract syntax tree (AST) is an intermediate representation of the
//! raw input after it has been tokenised and parsed. The AST may only be
//! partially complete with respect to the raw input, because there may be
//! errors in the raw input.

use super::identifier::Identifier;
use super::lexer::{Span, Spanned};

///
#[derive(Clone, Debug, PartialEq)]
pub enum Ast {
    Expr(Expr),
    Nil,
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    BinaryOperator(Box<BinaryOperatorExpr>),
    Literal(Box<LiteralExpr>),
    PrefixOperator(Box<PrefixOperatorExpr>),
    SuffixOperator(Box<SuffixOperatorExpr>),
    Void(Box<VoidExpr>),
}

impl Spanned for Expr {
    fn span(&self) -> &Span {
        match *self {
            Expr::BinaryOperator(ref expr, ..) => expr.span(),
            Expr::Literal(ref expr, ..) => expr.span(),
            Expr::PrefixOperator(ref expr, ..) => expr.span(),
            Expr::SuffixOperator(ref expr, ..) => expr.span(),
            Expr::Void(ref expr, ..) => expr.span(),
            _ => unimplemented!(),
        }
    }

    fn span_mut(&mut self) -> &mut Span {
        match *self {
            Expr::BinaryOperator(ref mut expr, ..) => expr.span_mut(),
            Expr::Literal(ref mut expr, ..) => expr.span_mut(),
            Expr::PrefixOperator(ref mut expr, ..) => expr.span_mut(),
            Expr::SuffixOperator(ref mut expr, ..) => expr.span_mut(),
            Expr::Void(ref mut expr, ..) => expr.span_mut(),
            _ => unimplemented!(),
        }
    }
}

/// An `Expr` can be created from a `BinaryOperatorExpr`.
impl From<BinaryOperatorExpr> for Expr {
    fn from(expr: BinaryOperatorExpr) -> Expr {
        Expr::BinaryOperator(expr.into())
    }
}

/// An `Expr` can be created from a `Box<BinaryOperatorExpr>`.
impl From<Box<BinaryOperatorExpr>> for Expr {
    fn from(expr: Box<BinaryOperatorExpr>) -> Expr {
        Expr::BinaryOperator(expr)
    }
}

/// An `Expr` can be created from a `LiteralExpr`.
impl From<LiteralExpr> for Expr {
    fn from(expr: LiteralExpr) -> Expr {
        Expr::Literal(expr.into())
    }
}

/// An `Expr` can be created from a `Box<LiteralExpr>`.
impl From<Box<LiteralExpr>> for Expr {
    fn from(expr: Box<LiteralExpr>) -> Expr {
        Expr::Literal(expr)
    }
}

/// An `Expr` can be created from a `PrefixOperatorExpr`.
impl From<PrefixOperatorExpr> for Expr {
    fn from(expr: PrefixOperatorExpr) -> Expr {
        Expr::PrefixOperator(expr.into())
    }
}

/// An `Expr` can be created from a `Box<PrefixOperatorExpr>`.
impl From<Box<PrefixOperatorExpr>> for Expr {
    fn from(expr: Box<PrefixOperatorExpr>) -> Expr {
        Expr::PrefixOperator(expr)
    }
}

/// An `Expr` can be created from a `SuffixOperatorExpr`.
impl From<SuffixOperatorExpr> for Expr {
    fn from(expr: SuffixOperatorExpr) -> Expr {
        Expr::SuffixOperator(expr.into())
    }
}

/// An `Expr` can be created from a `Box<SuffixOperatorExpr>`.
impl From<Box<SuffixOperatorExpr>> for Expr {
    fn from(expr: Box<SuffixOperatorExpr>) -> Expr {
        Expr::SuffixOperator(expr)
    }
}

/// An `Expr` can be created from a `VoidExpr`.
impl From<VoidExpr> for Expr {
    fn from(expr: VoidExpr) -> Expr {
        Expr::Void(expr.into())
    }
}

/// An `Expr` can be created from a `Box<VoidExpr>`.
impl From<Box<VoidExpr>> for Expr {
    fn from(expr: Box<VoidExpr>) -> Expr {
        Expr::Void(expr)
    }
}

///
pub type Exprs = Vec<Expr>;

///
#[derive(Clone, Debug, PartialEq)]
pub enum LiteralExpr {
    Bool(bool, Span),
    Char(char, Span),
    Float(f64, Span),
    Int(i64, Span),
    Str(String, Span),
}

impl Spanned for LiteralExpr {
    fn span(&self) -> &Span {
        use self::LiteralExpr::*;

        match *self {
            Bool(_, ref span, ..) => span,
            Char(_, ref span, ..) => span,
            Float(_, ref span, ..) => span,
            Int(_, ref span, ..) => span,
            Str(_, ref span, ..) => span,
        }
    }

    fn span_mut(&mut self) -> &mut Span {
        use self::LiteralExpr::*;

        match *self {
            Bool(_, ref mut span, ..) => span,
            Char(_, ref mut span, ..) => span,
            Float(_, ref mut span, ..) => span,
            Int(_, ref mut span, ..) => span,
            Str(_, ref mut span, ..) => span,
        }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct BinaryOperatorExpr {
    operator: Operator,
    lhs: Expr,
    rhs: Expr,
    span: Span,
}

impl BinaryOperatorExpr {
    pub fn new<Op, LHS, RHS, S>(operator: Op, lhs: LHS, rhs: RHS, span: S) -> BinaryOperatorExpr
        where Op: Into<Operator>,
              LHS: Into<Expr>,
              RHS: Into<Expr>,
              S: Into<Span>
    {
        BinaryOperatorExpr {
            operator: operator.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
            span: span.into()
        }
    }
}

impl Spanned for BinaryOperatorExpr {
    fn span(&self) -> &Span {
        &self.span
    }

    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct PrefixOperatorExpr {
    operator: Operator,
    rhs: Expr,
    span: Span,
}

impl PrefixOperatorExpr {
    pub fn new<Op, RHS, S>(operator: Op, rhs: RHS, span: S) -> PrefixOperatorExpr
        where Op: Into<Operator>,
              RHS: Into<Expr>,
              S: Into<Span>
    {
        PrefixOperatorExpr {
            operator: operator.into(),
            rhs: rhs.into(),
            span: span.into()
        }
    }
}

impl Spanned for PrefixOperatorExpr {
    fn span(&self) -> &Span {
        &self.span
    }

    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct SuffixOperatorExpr {
    operator: Operator,
    lhs: Expr,
    span: Span,
}

impl SuffixOperatorExpr {
    pub fn new<Op, LHS, S>(operator: Op, lhs: LHS, span: S) -> SuffixOperatorExpr
        where Op: Into<Operator>,
              LHS: Into<Expr>,
              S: Into<Span>
    {
        SuffixOperatorExpr {
            operator: operator.into(),
            lhs: lhs.into(),
            span: span.into()
        }
    }
}

impl Spanned for SuffixOperatorExpr {
    fn span(&self) -> &Span {
        &self.span
    }

    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add(Span),
    AddEq(Span),
    And(Span),
    Assign(Span),
    Div(Span),
    DivEq(Span),
    Equal(Span),
    GreaterThan(Span),
    GreaterThanEq(Span),
    LessThan(Span),
    LessThanEq(Span),
    Mul(Span),
    MulEq(Span),
    Or(Span),
    PushPop(Span),
    Sub(Span),
    SubEq(Span),
}

impl Spanned for Operator {
    fn span(&self) -> &Span {
        use self::Operator::*;

        match *self {
            Add(ref span, ..) => span,
            AddEq(ref span, ..) => span,
            And(ref span, ..) => span,
            Assign(ref span, ..) => span,
            Div(ref span, ..) => span,
            DivEq(ref span, ..) => span,
            Equal(ref span, ..) => span,
            GreaterThan(ref span, ..) => span,
            GreaterThanEq(ref span, ..) => span,
            LessThan(ref span, ..) => span,
            LessThanEq(ref span, ..) => span,
            Mul(ref span, ..) => span,
            MulEq(ref span, ..) => span,
            Or(ref span, ..) => span,
            PushPop(ref span, ..) => span,
            Sub(ref span, ..) => span,
            SubEq(ref span, ..) => span,
        }
    }

    fn span_mut(&mut self) -> &mut Span {
        use self::Operator::*;

        match *self {
            Add(ref mut span, ..) => span,
            AddEq(ref mut span, ..) => span,
            And(ref mut span, ..) => span,
            Assign(ref mut span, ..) => span,
            Div(ref mut span, ..) => span,
            DivEq(ref mut span, ..) => span,
            Equal(ref mut span, ..) => span,
            GreaterThan(ref mut span, ..) => span,
            GreaterThanEq(ref mut span, ..) => span,
            LessThan(ref mut span, ..) => span,
            LessThanEq(ref mut span, ..) => span,
            Mul(ref mut span, ..) => span,
            MulEq(ref mut span, ..) => span,
            Or(ref mut span, ..) => span,
            PushPop(ref mut span, ..) => span,
            Sub(ref mut span, ..) => span,
            SubEq(ref mut span, ..) => span,
        }
    }
}

///
#[derive(Clone, Debug, PartialEq)]
pub struct VoidExpr {
    span: Span,
}

impl VoidExpr {
    pub fn new<S>(span: S) -> VoidExpr
        where S: Into<Span>
    {
        VoidExpr { span: span.into() }
    }
}

impl Spanned for VoidExpr {
    fn span(&self) -> &Span {
        &self.span
    }

    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}