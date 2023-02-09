use crate::{lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal, Ordinal}, parse_ast};

use super::{logic_value::LogicValue, LibrettoParsable, ParseResult, LibrettoCompileError};

//==================================================================================================
//          Logic Unary Expression
//==================================================================================================

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Negative,
    Bang
}

#[derive(Debug)]
pub struct LogicUnaryExpr {
    operator : Option<UnaryOperator>,
    value: LogicValue,
}

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicUnaryExpr {
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> ParseResult<Self> {

        //for now
        queue.reset();
        let option_operator = queue.pop_if_next_is([LogicOrdinal::Sub, LogicOrdinal::Bang]);
        let result = LogicValue::parse(queue);

        match result {
            ParseResult::Parsed(value) => {
                let operator = if option_operator.is_some() {
                    match option_operator.unwrap() {
                        LibrettoLogicToken::Bang => Some(UnaryOperator::Bang),
                        LibrettoLogicToken::Sub => Some(UnaryOperator::Negative),
                        _ => None
                    }
                } else {
                    None
                };
                ParseResult::Parsed(LogicUnaryExpr{operator, value})
            },
            ParseResult::Error(err) => ParseResult::Error(err),
            ParseResult::Failure => ParseResult::Failure,
        }
    }

    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        let mut operator_space = 0;
        queue.next_is([LogicOrdinal::Bang, LogicOrdinal::Sub]);
        if LogicValue::raw_check(queue) { return true } 
        else {false}
    }

    fn validate(&self, errors : &mut Vec<LibrettoCompileError>) {
        todo!()
    }
}

//==================================================================================================
//          Additive Expression
//==================================================================================================

#[derive(Debug)]
pub struct LogicAdditiveExpr {
    lhs : LogicUnaryExpr,
    rhs : Option<LogicUnaryExpr>,
    is_adding : bool
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicAdditiveExpr {
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> ParseResult<Self> {
        //This needs work
        
        queue.reset();
        let lhs = parse_ast!(LogicUnaryExpr, queue);
        let operator = queue.pop_if_next_is([LogicOrdinal::Add, LogicOrdinal::Sub]);
        let rhs = parse_ast!(LogicUnaryExpr, queue);

        if let Some(operator) = operator {
            let is_adding = match operator {
                LibrettoLogicToken::Add => true,
                LibrettoLogicToken::Sub => false,
                _ => return ParseResult::Error("Not a valid operator".to_owned())
            };

            return ParseResult::Parsed(LogicAdditiveExpr { lhs, rhs: Some(rhs), is_adding});
        }

        ParseResult::Failure
    }

    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if !LogicValue::raw_check(queue) {return false};
        if !queue.next_is([LogicOrdinal::Add, LogicOrdinal::Sub]) {return false;};
        LogicValue::raw_check(queue);
        queue.mark();
        true
    }

    fn validate(&self, errors : &mut Vec<LibrettoCompileError>) {
        todo!()
    }
}

//==================================================================================================
//          Tests
//==================================================================================================

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::{
        lexer::{LibrettoLogicToken, LibrettoTokenQueue},
        parse::{LibrettoParsable, self, ParseResult}, logic::lson::Lson,
    };

    use super::{LogicValue, LogicUnaryExpr, UnaryOperator};

    fn check_expr(source : &str) {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let check = LogicUnaryExpr::raw_check(&mut queue);
        assert!(check);
    }

    fn check_expr_inv(source : &str) {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let check = LogicUnaryExpr::raw_check(&mut queue);
        assert!(!check);
    }

    #[test]
    fn check_unary_expr() {
        check_expr("-3.14");
        check_expr("!false");
        check_expr("3.14");
        check_expr("false");

        check_expr_inv("!!false");
        check_expr_inv("function");
    }

    #[test]
    fn parse_unary_expr() {
        fn parse_check(source : &str, operator : Option<UnaryOperator>, value : Lson) {
            let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
            let parse = LogicUnaryExpr::parse(&mut queue);
            if let ParseResult::Parsed(expr) = parse {
                assert_eq!(expr.operator, operator);
                assert_eq!(expr.value, LogicValue::Literal(value))
            } else {
                assert!(false)
            }
        }
        
        parse_check("-3.14", Some(UnaryOperator::Negative), Lson::Float(3.14));
        parse_check("!false", Some(UnaryOperator::Bang), Lson::Bool(false));
        parse_check("3.14", None, Lson::Float(3.14));
        parse_check("true", None, Lson::Bool(true));
    }

    #[test]
    fn check_binary_expr() {
        check_expr("1 + 1");
        check_expr("false + true");
        check_expr("\"Hello\" + \"world!\"");
        check_expr("1");
        check_expr("false");
    }
}
