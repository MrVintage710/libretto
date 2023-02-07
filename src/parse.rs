mod logic_let;
mod logic_expr;
mod logic_value;

use logos::{Lexer, Logos};
use std::{process::Output, fmt::{Debug, Display}};

use crate::{
    lexer::{LibrettoTokenQueue, Ordinal},
    runtime::LibrettoRuntime,
};

pub enum ParseResult<T> {
    Parsed(T),
    Error(String),
    Failure,
}

impl <T> Debug for ParseResult<T> where T : Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parsed(arg0) => f.debug_tuple("Parsed").field(arg0).finish(),
            Self::Error(arg0) => f.debug_tuple("Error").field(arg0).finish(),
            Self::Failure => write!(f, "Failure"),
        }
    }
}

impl <T> Display for ParseResult<T> where T : Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parsed(arg0) => f.debug_tuple("Parsed").finish(),
            Self::Error(arg0) => f.debug_tuple("Error").field(arg0).finish(),
            Self::Failure => write!(f, "Failure"),
        }
    }
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
    fn check(queue: &mut LibrettoTokenQueue<'a, T>) -> bool;

    fn parse(queue: &mut LibrettoTokenQueue<'a, T>) -> ParseResult<Self>;

    fn checked_parse(queue: &mut LibrettoTokenQueue<'a, T>) -> Option<Self> {
        if Self::check(queue) {
            let result = Self::parse(queue);
            match result {
                ParseResult::Parsed(value) => Some(value),
                ParseResult::Error(err) => panic!("Error durring parse: {}", err),
                ParseResult::Failure => None,
            }    
        } else {
            None
        }
    }
}

pub trait LibrettoEvaluator {
    type Output;

    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> Output;
}
