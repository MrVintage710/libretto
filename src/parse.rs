mod logic_expr;
mod logic_let;
mod logic_value;
mod util;

use logos::Logos;
use std::{
    fmt::{Debug, Display},
    process::Output,
};
use thiserror::Error;

use crate::{
    lexer::{LibrettoTokenQueue, Ordinal},
    runtime::LibrettoRuntime,
};

pub enum ParseResult<T> {
    Parsed(T),
    Error(String),
    Failure,
}

impl<T> Debug for ParseResult<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parsed(arg0) => f.debug_tuple("Parsed").field(arg0).finish(),
            Self::Error(arg0) => f.debug_tuple("Error").field(arg0).finish(),
            Self::Failure => write!(f, "Failure"),
        }
    }
}

impl<T> Display for ParseResult<T>
where
    T: Display,
{
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
    T: Logos<'a> + PartialEq + Ordinal + Clone + Debug + 'a,
    T::Extras: Clone,
    Self: Sized,
{
    ///This function will check the token queue
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, T>) -> bool;

    fn parse(queue: &mut LibrettoTokenQueue<'a, T>) -> ParseResult<Self>;

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>);

    fn check(queue: &mut LibrettoTokenQueue<'a, T>) -> bool {
        if Self::raw_check(queue) {
            queue.mark();
            true
        } else {
            queue.rewind();
            false
        }
    }

    fn checked_parse(queue: &mut LibrettoTokenQueue<'a, T>) -> Option<Self> {
        queue.reset();
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

pub type LibrettoCompileResult<T> = Result<T, LibrettoCompileError>;

#[derive(Error, Debug)]
pub enum LibrettoCompileError {
    #[error("Values are not allowed to be set to null.")]
    NullValueError,
}

// #[macro_export]
// macro_rules! validate_ast {
//     ($ast:expr) => {
//         {
//             let result = $ast.validate();
//             match result {
//                 Err(e) => {return Err(e)},
//                 _ => {}
//             };
//         }
//     };
// }

/// This macro will parse a token from the Queue given a type.
/// If there is a error, it returns the error.
/// If there is a failure, it resturns the failure.
/// If it parses successfully, the value is passed back.
#[macro_export]
macro_rules! parse_ast {
    ($ast:path, $queue:expr) => {{
        let result = <$ast>::parse($queue);
        match result {
            ParseResult::Parsed(value) => value,
            ParseResult::Error(err) => return ParseResult::Error(err),
            ParseResult::Failure => return ParseResult::Failure,
        }
    }};
}
