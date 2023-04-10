mod logic_let_stmt;
mod logic_assignment_stmt;
mod logic_value;
mod logic_expr;
mod logic_unary_expr;
mod logic_term_expr;
mod logic_factor_expr;
mod logic_comparison_expr;
mod logic_equality_expr;
mod util;

use logos::Logos;
use std::{
    fmt::Debug, collections::HashMap,
};
use thiserror::Error;

use crate::{
    lson::{LsonType, Lson},
    lexer::{LibrettoTokenQueue, Ordinal}, runtime::LibrettoRuntime,
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

    fn parse(queue: &mut LibrettoTokenQueue<'a, T>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self>;

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> LsonType;

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

    #[error("When parsing an expression with type {0}, there was a default supplied with type {1}. These types must be the same.")]
    ExprDefaultTypeMissmatch(String, String),
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

pub mod test_util {

    use std::collections::HashMap;

    use logos::Logos;

    use crate::lson::{LsonType, Lson};
    use crate::lexer::{LibrettoLogicToken, LibrettoTokenQueue};
    use crate::parse::LibrettoCompileError;
    use crate::runtime::LibrettoRuntime;

    use super::{LibrettoParsable, LibrettoEvaluator};

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
        let result = T::checked_parse(&mut queue, &mut Vec::new());
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
            (String::from("bar"), LsonType::Bool),
        ]);
        let ast = T::checked_parse(&mut queue, &mut errors);
        assert!(ast.is_some());
        let ast = ast.unwrap();
        let ast_type = ast.validate(&mut errors, &mut types);
        for error in errors.iter() {
            println!("{:?}", error)
        }
        assert_eq!(errors.len(), number_of_errors);
        assert_eq!(static_type, ast_type);
        errors
    }

    pub fn evaluate_expr<'a, T : LibrettoParsable<'a, LibrettoLogicToken> + LibrettoEvaluator>(
        source: &'a str,
        lson : Lson
    ) {
        let mut runtime = LibrettoRuntime::with_data([
            (String::from("foo"), Lson::Float(2.0)),
            (String::from("bar"), Lson::Bool(true)),
        ]);
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let mut compile_time_errors = Vec::new();
        let mut types = HashMap::from([
            (String::from("foo"), LsonType::Float),
            (String::from("bar"), LsonType::Bool),
        ]);
        let ast = T::checked_parse(&mut queue, &mut compile_time_errors);
        assert!(ast.is_some());
        let ast = ast.unwrap();
        let ast_type = ast.validate(&mut compile_time_errors, &mut types);
        let result = ast.evaluate(&mut runtime);
        assert_eq!(ast_type, result.get_type());
        assert_eq!(result, lson);
    }
}