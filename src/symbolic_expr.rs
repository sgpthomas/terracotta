use std::{fmt::Display, ops};

#[derive(Clone, Debug)]
pub enum SymbolicExpr {
    Int(i64),
    Bool(bool),
    Var(String),
    // int operations
    Add(Box<SymbolicExpr>, Box<SymbolicExpr>),
    Subtract(Box<SymbolicExpr>, Box<SymbolicExpr>),
    Times(Box<SymbolicExpr>, Box<SymbolicExpr>),
    Div(Box<SymbolicExpr>, Box<SymbolicExpr>),
    // bool operations
    And(Box<SymbolicExpr>, Box<SymbolicExpr>),
    Or(Box<SymbolicExpr>, Box<SymbolicExpr>),
    Not(Box<SymbolicExpr>),
}

impl Display for SymbolicExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolicExpr::Int(i) => write!(f, "{i}"),
            SymbolicExpr::Bool(b) => write!(f, "{b}"),
            SymbolicExpr::Var(s) => write!(f, "{s}"),
            SymbolicExpr::Add(x, y) => write!(f, "{x} + {y}"),
            SymbolicExpr::Subtract(x, y) => write!(f, "{x} - {y}"),
            SymbolicExpr::Times(x, y) => write!(f, "{x} * {y}"),
            SymbolicExpr::Div(x, y) => write!(f, "{x} / {y}"),

            SymbolicExpr::And(x, y) => write!(f, "{x} && {y}"),
            SymbolicExpr::Or(x, y) => write!(f, "{x} || {y}"),
            SymbolicExpr::Not(x) => write!(f, "!{x}"),
        }
    }
}

impl SymbolicExpr {
    fn var<S>(s: S) -> Self
    where
        S: ToString,
    {
        Self::Var(s.to_string())
    }

    fn add(self, rhs: Self) -> Self {
        Self::Add(Box::new(self), Box::new(rhs))
    }

    fn subtract(self, rhs: Self) -> Self {
        Self::Subtract(Box::new(self), Box::new(rhs))
    }

    fn times(self, rhs: Self) -> Self {
        Self::Times(Box::new(self), Box::new(rhs))
    }

    fn div(self, rhs: Self) -> Self {
        Self::Div(Box::new(self), Box::new(rhs))
    }

    fn and(self, rhs: Self) -> Self {
        Self::And(Box::new(self), Box::new(rhs))
    }

    fn or(self, rhs: Self) -> Self {
        Self::Or(Box::new(self), Box::new(rhs))
    }

    fn not(self) -> Self {
        Self::Not(Box::new(self))
    }
}

impl From<&str> for SymbolicExpr {
    fn from(v: &str) -> Self {
        SymbolicExpr::var(v)
    }
}

impl From<i64> for SymbolicExpr {
    fn from(v: i64) -> Self {
        SymbolicExpr::Int(v)
    }
}

impl<S> ops::Add<S> for SymbolicExpr
where
    S: Into<SymbolicExpr>,
{
    type Output = Self;

    fn add(self, rhs: S) -> Self::Output {
        self.add(rhs.into())
    }
}

impl<S> ops::Sub<S> for SymbolicExpr
where
    S: Into<SymbolicExpr>,
{
    type Output = Self;

    fn sub(self, rhs: S) -> Self::Output {
        self.subtract(rhs.into())
    }
}

impl<S> ops::Mul<S> for SymbolicExpr
where
    S: Into<SymbolicExpr>,
{
    type Output = Self;

    fn mul(self, rhs: S) -> Self::Output {
        self.times(rhs.into())
    }
}

impl<S> ops::Div<S> for SymbolicExpr
where
    S: Into<SymbolicExpr>,
{
    type Output = Self;

    fn div(self, rhs: S) -> Self::Output {
        self.div(rhs.into())
    }
}

impl<S> ops::BitAnd<S> for SymbolicExpr
where
    S: Into<SymbolicExpr>,
{
    type Output = Self;

    fn bitand(self, rhs: S) -> Self::Output {
        self.and(rhs.into())
    }
}

impl<S> ops::BitOr<S> for SymbolicExpr
where
    S: Into<SymbolicExpr>,
{
    type Output = Self;

    fn bitor(self, rhs: S) -> Self::Output {
        self.or(rhs.into())
    }
}

impl ops::Not for SymbolicExpr {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.not()
    }
}
