use inkwell::support::LLVMString;
use crate::parser::action::Action;

/**
 * Generates LLVM IR from an AST.
 */
pub struct LLVMGenerator {
  ast: Vec<Action>,
}

impl LLVMGenerator {
  pub fn generate(ast: Vec<Action>) -> LLVMString {
    unimplemented!()
  }
}
