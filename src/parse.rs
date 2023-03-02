mod logic_expr;
mod logic_let;
mod logic_value;
mod util;

use logos::Logos;
use std::{
    fmt::{Debug}, collections::HashMap,
};
use thiserror::Error;

use crate::{
    logic::lson::{LsonType, Lson},
    lexer::{LibrettoTokenQueue, Ordinal},
    runtime::LibrettoRuntime,
};

//==================================================================================================
//          LibrettoParsable
//==================================================================================================

pub trait LibrettoParsable<'a, T>
where
    T: Logos<'a> + PartialEq + Ordinal + Clone + Debug + 'a,
    T::Extras: Clone,
    Self: Sized,
{
    ///This function will check the token queue
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, T>) -> bool;

    fn parse(queue: &mut LibrettoTokenQueue<'a, T>, errors: &mut Vec<LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> Option<Self>;

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>, type_map : &HashMap<String, LsonType>) -> LsonType;

    fn check(queue: &mut LibrettoTokenQueue<'a, T>) -> bool {
        if Self::raw_check(queue) {
            queue.mark();
            true
        } else {
            queue.rewind();
            false
        }
    }

    fn checked_parse(queue: &mut LibrettoTokenQueue<'a, T>, errors: &mut Vec<LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> Option<Self> {
        queue.reset();
        if Self::check(queue) {
            Self::parse(queue, errors, type_map)
        } else {
            None
        }
    }
}

//==================================================================================================
//          Evaluator
//==================================================================================================

pub trait LibrettoEvaluator {
    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> Lson;
}

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
    ($type:ty, $queue:expr, $errors:expr, $type_map:expr) => {
        {
            let result = <$type>::parse($queue, $errors, $type_map);
            match result {
                Some(value) => value,
                None => return None
            }
        }
    };
}

pub mod test_util {

    use std::collections::HashMap;

    use logos::Logos;

    use crate::logic::lson::{LsonType, Lson};
    use crate::lexer::{LibrettoLogicToken, LibrettoTokenQueue};
    use crate::parse::LibrettoCompileError;

    use super::LibrettoParsable;

    pub fn check_expr<'a, T: LibrettoParsable<'a, LibrettoLogicToken>>(
        source: &'a str,
        number_of_tokens: usize,
    ) {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let check = T::check(&mut queue);
        assert!(check);
        assert_eq!(queue.cursor(), 0);
        queue.reset();
        let check = T::raw_check(&mut queue);
        assert!(check);
        assert_eq!(queue.cursor(), number_of_tokens)
    }

    pub fn parse_expr<'a, T: LibrettoParsable<'a, LibrettoLogicToken>>(source: &'a str) -> T {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let mut types = HashMap::from([
            (String::from("foo"), LsonType::Float),
            (String::from("bar"), LsonType::String),
        ]);
        let result = T::checked_parse(&mut queue, &mut Vec::new(), &mut types);
        assert!(result.is_some());
        result.unwrap()
    }

    pub fn validate_expr<'a, T: LibrettoParsable<'a, LibrettoLogicToken>>(
        source: &'a str,
        number_of_errors: usize,
        static_type : LsonType
    ) -> Vec<LibrettoCompileError> {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let mut errors = Vec::new();
        let mut types = HashMap::from([
            (String::from("foo"), LsonType::Float),
            (String::from("bar"), LsonType::String),
        ]);
        let ast = T::checked_parse(&mut queue, &mut errors, &mut types);
        assert!(ast.is_some());
        let ast = ast.unwrap();
        let ast_type = ast.validate(&mut errors);
        for error in errors.iter() {
            println!("{:?}", error)
        }
        assert_eq!(errors.len(), number_of_errors);
        assert_eq!(static_type, ast_type);
        errors
    }
}