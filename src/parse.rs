mod logic_literal;

use std::process::Output;
use logos::{Logos, Lexer};

use crate::runtime::LibrettoRuntime;

pub trait LibrettoParsable<'a, T : Logos<'a>> where Self : Sized {
  fn parse(lexer : &mut Lexer<'a, T>) -> Option<Self>;
}

pub trait LibrettoEvaluator {
  type Output;

  fn evaluate(&self, runtime : &mut LibrettoRuntime) -> Output;
}