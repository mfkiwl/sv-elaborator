//! This module represent elaborated and type-checked expressions.

use std::ops;
use std::fmt;
use source::Span;
use number::LogicVec;
use super::ty::Ty;
use syntax::ast::{IncDec, UnaryOp, BinaryOp, Spanned};

use syntax::ast::{Scope, HierId, DataType, AssignPattern, Ident};

/// A typed value with concrete data type.
#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Type(Ty),
    Int(LogicVec),
    FixStr(String),
    Real(f64),
    String, // TODO
    Chandle, // TODO
    Event, // TODO
}

/// Select expression after type checking
#[derive(Debug, Clone)]
pub enum DimKind {
    /// Represent bit-select of type `[ expression ]`.
    Value(Box<Expr>),
    /// Represent bit-select of type `[ expression : expression ]`
    Range(Box<Expr>, Box<Expr>),
    /// Represent bit-select of type `[ expression +: expression ]`
    PlusRange(Box<Expr>, Box<Expr>),
    /// Represent bit-select of type `[ expression -: expression ]`
    MinusRange(Box<Expr>, Box<Expr>),
}

/// Should be boxed when nested in other AST structure.
pub type Dim = Spanned<DimKind>;

/// Represent expression after type checking.
#[derive(Debug, Clone)]
pub enum ExprKind {
    Const(Val),
    
    /// A hierachical name
    HierName(Option<Scope>, Spanned<HierId>),

    /// Empty queue initializer ("{}")
    EmptyQueue,

    /// Concatenation
    Concat(Vec<Expr>),

    /// Multiple concatenation
    MultConcat(Box<Expr>, Box<Expr>),

    /// Assignment pattern expression
    AssignPattern(Option<Box<DataType>>, AssignPattern),

    /// Element select
    Select(Box<Expr>, Dim),

    /// Member access
    Member(Box<Expr>, Ident),

    // Subroutine calls
    /// Call to system task
    SysTfCall(Box<Spanned<String>>, Vec<Option<Expr>>),

    // Casts
    ConstCast(Box<Expr>),

    /// Cast to a type
    TypeCast(Box<Ty>, Box<Expr>),
    /// Cast to a specified sign,
    SignCast(bool, Box<Expr>),
    /// Cast to a specified width
    WidthCast(usize, Box<Expr>),

    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    PrefixIncDec(IncDec, Box<Expr>),
    PostfixIncDec(Box<Expr>, IncDec),

    /// Assignment
    Assign(Box<Expr>, Box<Expr>),
    BinaryAssign(Box<Expr>, BinaryOp, Box<Expr>),

    /// Parenthesised expression
    Paren(Box<Expr>),

    /// Min-typ-max expression
    MinTypMax(Box<Expr>, Box<Expr>, Box<Expr>),

    /// Conditional expression
    Cond(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Clone)]
pub struct Expr {
    pub value: ExprKind,
    pub span: Span,
    pub ty: Ty,
}

impl ops::Deref for Expr {
    type Target = ExprKind;

    fn deref(&self) -> &ExprKind {
        &self.value
    }
}

impl ops::DerefMut for Expr {
    fn deref_mut(&mut self) -> &mut ExprKind {
        &mut self.value
    }
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}