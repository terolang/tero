mod llvm;

use inkwell::support::LLVMString;
use super::parser::action::Action;
use super::error::Error;

use llvm::LLVMGenerator;

pub enum Target {
  LLVM,
}

pub enum Output {
  String(String),
  LLVM(LLVMString)
}

/**
 * Matches a target to a generator and passes to AST for code generation.
 */
pub fn generate(target: Target, ast: Vec<Action>) -> Result<Output, Error> {
  match target {
    Target::LLVM => Ok(Output::LLVM(LLVMGenerator::generate(ast))),
  }
}
