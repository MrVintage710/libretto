mod logic_value;
mod logic_let;
mod logic_unary;

use std::process::Output;
use logos::{Logos, Lexer};

use crate::{runtime::LibrettoRuntime, lexer::LibrettoTokenQueue};

pub enum ParseResult {
  Parsed,
  
}

pub trait LibrettoParsable<'a, T : Logos<'a> + PartialEq> where Self : Sized {
  fn parse(queue : &mut LibrettoTokenQueue<'a, T>) -> Option<Self>;
}

pub trait LibrettoEvaluator {
  type Output;

  fn evaluate(&self, runtime : &mut LibrettoRuntime) -> Output;
}