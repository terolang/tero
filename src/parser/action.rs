use super::CodeLocation;

pub enum Expression {
  Null
}

pub struct Action {
  pub expr: Expression,
  pub loc: CodeLocation,
}

impl Action {
  pub fn new(expr: Expression, loc: CodeLocation) -> Action {
    Action { expr, loc }
  }
}
