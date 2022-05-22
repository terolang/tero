pub mod action;

use action::Action;
use super::error::Error;

/**
 * Points to a specific character in a source file
 */
pub struct CodeLocation {
  pub pos: u32,
  pub x: u16,
  pub y: u16,
}

impl CodeLocation {
  pub fn new(pos: u32, x: u16, y: u16) -> CodeLocation {
    CodeLocation { pos, x, y }
  }
}

/**
 * The parser takes a source file and parses it into an AST
 */
pub struct Parser {
  src: Vec<char>,
  ast: Vec<Action>,
  pos: u32,
  x: u16,
  y: u16,
}

impl Parser {
  /**
   * Parse a source file into an AST
   */
  pub fn parse(src: String) -> Result<Vec<Action>, Error> {
    let parser = Parser {
      src: src.chars().collect(),
      ast: vec![],
      pos: 0,
      x: 0,
      y: 0,
    };
    Ok(parser.ast)
  }

  /**
   * Get the next character in the source file
   */
  fn next(&mut self) -> Option<char> {
    self.pos += 1;
    self.x += 1;
    let c = self.src.get(self.pos as usize).cloned();
    if c == Some('\n') {
      self.x = 0;
      self.y += 1;
    }
    c
  }

  /**
   * Get the next character in the source file ignoring characters in `except`
   */
  fn next_except(&mut self, except: String) -> Option<char> {
    let c = self.next();
    if c.is_none() || except.contains(c.unwrap()) {
      return c;
    }
    self.next_except(except)
  }

  /**
   * Peek at the next character without incrementing the position
   */
  fn peek(&mut self) -> Option<char> {
    self.src.get((self.pos + 1) as usize).cloned()
  }
}
