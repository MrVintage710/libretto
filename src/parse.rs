mod logic_let_stmt;
mod logic_assignment_stmt;
mod logic_value;
mod logic_expr;
mod logic_unary_expr;
mod logic_term_expr;
mod logic_factor_expr;
mod logic_comparison_expr;
mod logic_equality_expr;
mod logic_stmt;
mod util;

use logos::Logos;
use std::{
    fmt::Debug, collections::{HashMap, VecDeque, vec_deque::Iter},
};
use thiserror::Error;

use crate::{
    lson::{LsonType, Lson},
    lexer::{LibrettoTokenQueue, Ordinal, LibrettoLogicToken}, runtime::LibrettoRuntime, compiler::LibrettoCompiletime, queue::TokenQueue,
};

pub enum CheckResult<'a, T>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal + Debug
{
    Pass(TokenQueue<'a, T>, Box<dyn Parsable<'a, T>>),
    Fail
}

pub trait Checkable<'a, T>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal + Debug
{
    fn check(queue : &mut TokenQueue<'a, T>) -> CheckResult<'a, T>;
}

pub trait Parsable<'a, T>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal + Debug
{
    fn parse(tokens : TokenQueue<'a, T>) -> Option<Box<dyn Evaluator>>  {}
}

pub struct NullParsable {}

impl Parsable for NullParsable {}

pub struct GroupParsable {
    parables : Vec<Box<dyn Parsable>>
}

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

    fn parse(queue: &mut LibrettoTokenQueue<'a, T>, compile_time : &mut LibrettoCompiletime) -> Option<Self>;

    fn validate(&self, compile_time : &mut LibrettoCompiletime) -> LsonType;

    fn check(queue: &mut LibrettoTokenQueue<'a, T>) -> bool {
        queue.mark();
        if Self::raw_check(queue) {
            queue.advance();
            true
        } else {
            queue.rewind();
            false
        }
    }

    fn checked_parse(queue: &mut LibrettoTokenQueue<'a, T>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
        queue.reset();
        if Self::check(queue) {
            Self::parse(queue, compile_time)
        } else {
            None
        }
    }
}

#[macro_export]
macro_rules! parse_ast {
    ($type:ty, $queue:expr, $compile_time:expr) => {
        {
            let result = <$type>::parse($queue, $compile_time);
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

    use crate::compiler::LibrettoCompiletime;
    use crate::lson::{LsonType, Lson};
    use crate::lexer::{LibrettoLogicToken, LibrettoTokenQueue};
    use crate::runtime::{LibrettoRuntime, LibrettoEvaluator};

    use super::{LibrettoParsable};

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
        let mut compile_time = LibrettoCompiletime::with_data([
            (String::from("foo"), LsonType::Float),
            (String::from("bar"), LsonType::Bool),
        ]);
        let result = T::checked_parse(&mut queue, &mut compile_time);
        assert!(result.is_some());
        result.unwrap()
    }

    pub fn validate_expr<'a, T: LibrettoParsable<'a, LibrettoLogicToken>>(
        source: &'a str,
        number_of_errors: usize,
        static_type : LsonType
    ) {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let mut compile_time = LibrettoCompiletime::with_data([
            (String::from("foo"), LsonType::Float),
            (String::from("bar"), LsonType::Bool),
        ]);
        let ast = T::checked_parse(&mut queue, &mut compile_time);
        assert!(ast.is_some());
        let ast = ast.unwrap();
        let ast_type = ast.validate(&mut compile_time);
        assert_eq!(compile_time.error_count(), number_of_errors);
        assert_eq!(static_type, ast_type);
    }

    pub fn evaluate_expr<'a, T : LibrettoParsable<'a, LibrettoLogicToken> + LibrettoEvaluator>(
        source: &'a str,
        lson : Lson
    ) -> LibrettoRuntime {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let mut compile_time = LibrettoCompiletime::with_data([
            (String::from("foo"), LsonType::Float),
            (String::from("bar"), LsonType::Bool),
            ]);
        let mut runtime = LibrettoRuntime::with_data([
            (String::from("foo"), Lson::Float(2.0)),
            (String::from("bar"), Lson::Bool(true)),
        ]);
        let ast = T::checked_parse(&mut queue, &mut compile_time);
        assert!(ast.is_some());
        let ast = ast.unwrap();
        let ast_type = ast.validate(&mut compile_time);
        let result = ast.evaluate(&mut runtime).unwrap();
        assert_eq!(ast_type, result.get_type());
        assert_eq!(result, lson);
        runtime
    }
}