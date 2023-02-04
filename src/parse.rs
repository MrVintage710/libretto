mod logic_let;
mod logic_unary;
mod logic_value;

use logos::{Lexer, Logos};
use std::process::Output;

use crate::{
    lexer::{LibrettoTokenQueue, Ordinal},
    runtime::LibrettoRuntime,
};

pub enum ParseResult<T> {
    Parsed(T),
    Error(String),
    Failure,
}

impl<T> ParseResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Parsed(value) => value,
            Self::Error(err) => panic!("Unwrapped ParseResult on an Error Variant: {}", err),
            Self::Failure => {
                panic!("Unwrapped ParseResult on a Failure Variant. The parsing did not match.")
            }
        }
    }
}

pub trait LibrettoParsable<'a, T>
where
    T: Logos<'a> + PartialEq + Ordinal + Clone,
    T::Extras: Clone,
    Self: Sized,
{
    fn parse(queue: &mut LibrettoTokenQueue<'a, T>) -> ParseResult<Self>;
}

pub trait LibrettoEvaluator {
    type Output;

    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> Output;
}
