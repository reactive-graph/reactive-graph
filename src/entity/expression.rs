use strum::EnumString;

/// Represents the an expression with a left hand side and a right hand side.
#[derive(Debug, PartialEq, EnumString)]
pub enum OperatorPosition {
    LHS,
    RHS,
}

pub type ExpressionValue<T> = (OperatorPosition, T);

/// Represents an expression with a left hand side and a right hand side.
#[derive(Copy, Clone, Debug)]
pub struct Expression<LHS, RHS> {
    /// The left hand side of the expression.
    pub lhs: LHS,

    /// The right hand side of the expression.
    pub rhs: RHS,
}

impl<LHS: Clone, RHS: Clone> Expression<LHS, RHS> {
    pub fn new(lhs: LHS, rhs: RHS) -> Self {
        Expression { lhs, rhs }
    }

    pub fn lhs(&self, lhs: LHS) -> Self {
        Expression::new(lhs, self.rhs.clone())
    }

    pub fn rhs(&self, rhs: RHS) -> Self {
        Expression::new(self.lhs.clone(), rhs)
    }
}

#[derive(Clone, Debug)]
pub struct ExpressionResult<T, LHS, RHS> {
    /// Textual representation of the operator, for example "&&" for an AND-operator
    pub symbol: String,

    // The expression
    pub expression: Expression<LHS, RHS>,

    // The calculated result
    pub result: T,
}
