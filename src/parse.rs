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
    logic::lson::{LsonType},
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

    fn parse(queue: &mut LibrettoTokenQueue<'a, T>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self>;

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

    fn checked_parse(queue: &mut LibrettoTokenQueue<'a, T>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self> {
        queue.reset();
        if Self::check(queue) {
            Self::parse(queue, errors)
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

    #[error("The operator {0} is not supported for type {1}")]
    OperationNotSupportedError(String, String),
    
    #[error("The operation {0} is not supported for types {1} and {2}")]
    InvalidOperationError(String, String, String),

    #[error("When parsing '{0}', the pre parse check passed event though the pattern doesn't match.")]
    ParseCheckNotThoroughError(String),

}

#[macro_export]
macro_rules! parse_ast {
    ($type:ty, $queue:expr, $errors:expr) => {
        {
            let result = <$type>::parse($queue, $errors);
            match result {
                Some(value) => value,
                None => return None
            }
        }
    };
}

pub trait StaticTyped {
    fn get_static_type(&self) -> Option<LsonType>;
}