use std::{fmt::Display, ops};

#[derive(Clone, Debug)]
pub enum SymbolicExpr {
    Int(i64),
    Bool(bool),
    Var(String),
    Add(Box<SymbolicExpr>, Box<SymbolicExpr>),
    Subtract(Box<SymbolicExpr>, Box<SymbolicExpr>),
    Times(Box<SymbolicExpr>, Box<SymbolicExpr>),
    Div(Box<SymbolicExpr>, Box<SymbolicExpr>),
    Lt(Box<SymbolicExpr>, Box<SymbolicExpr>),
    Ite {
        cond: Box<SymbolicExpr>,
        true_branch: Box<SymbolicExpr>,
        false_branch: Box<SymbolicExpr>,
    },
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
            SymbolicExpr::Lt(x, y) => write!(f, "{x} < {y}"),
            SymbolicExpr::Ite {
                cond,
                true_branch,
                false_branch,
            } => write!(f, "if {cond} {{ {true_branch} }} else {{ {false_branch} }}"),
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

    pub fn add(self, rhs: Self) -> Self {
        Self::Add(Box::new(self), Box::new(rhs))
    }

    pub fn subtract(self, rhs: Self) -> Self {
        Self::Subtract(Box::new(self), Box::new(rhs))
    }

    pub fn times(self, rhs: Self) -> Self {
        Self::Times(Box::new(self), Box::new(rhs))
    }

    pub fn div(self, rhs: Self) -> Self {
        Self::Div(Box::new(self), Box::new(rhs))
    }

    pub fn lt(self, rhs: Self) -> Self {
        Self::Lt(Box::new(self), Box::new(rhs))
    }

    pub fn ite<F, G>(cond: Self, true_branch: F, false_branch: G) -> Self
    where
        F: Fn() -> Self,
        G: Fn() -> Self,
    {
        Self::Ite {
            cond: Box::new(cond),
            true_branch: Box::new(true_branch()),
            false_branch: Box::new(false_branch()),
        }
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

impl From<bool> for SymbolicExpr {
    fn from(b: bool) -> Self {
        SymbolicExpr::Bool(b)
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
