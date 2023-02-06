use crate::lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal, Ordinal};

use super::{logic_value::LogicValue, LibrettoParsable, ParseResult};

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
        queue.reset_cursor();
        let option_operator = queue.pop_if_next_is(LogicOrdinal::Sub);
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

    fn check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        let mut operator_space = 0;
        if queue.next_is([LogicOrdinal::Bang, LogicOrdinal::Sub]) { operator_space = 1; queue.forward(1); };
        if LogicValue::check(queue) { return true } 
        else {
            queue.backward(operator_space);
            false
        }
    }
}

//==================================================================================================
//          Additive Expression
//==================================================================================================

#[derive(Debug)]
pub struct LogicAdditiveExpr {
    lhs : LogicUnaryExpr,
    rhs : LogicUnaryExpr,
    is_adding : bool
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicAdditiveExpr {
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> ParseResult<Self> {
        let lhs = LogicUnaryExpr::checked_parse(queue);
        if lhs.is_none() {return ParseResult::Failure}

        if !queue.next_is([LogicOrdinal::Add, LogicOrdinal::Sub]) {}

        ParseResult::Failure
    }

    fn check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
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

    #[test]
    fn check_unary_expr() {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer("-3.14"));
        let check = LogicUnaryExpr::check(&mut queue);
        assert!(check);

        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer("!false"));
        let check = LogicUnaryExpr::check(&mut queue);
        assert!(check);

        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer("3.14"));
        let check = LogicUnaryExpr::check(&mut queue);
        assert!(check);

        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer("false"));
        let check = LogicUnaryExpr::check(&mut queue);
        assert!(check);

        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer("!!false"));
        let check = LogicUnaryExpr::check(&mut queue);
        assert!(!check);

        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer("function"));
        let check = LogicUnaryExpr::check(&mut queue);
        assert!(!check);
    }

    #[test]
    fn parse_unary_expr() {
        fn parse_check(source : &str, operator : Option<UnaryOperator>, ) {}
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer("-3.14"));
        let parse = LogicUnaryExpr::parse(&mut queue);
        if let ParseResult::Parsed(expr) = parse {
            assert!(expr.operator.is_some());
            assert_eq!(expr.operator.unwrap(), UnaryOperator::Negative);
            assert_eq!(expr.value, LogicValue::Literal(Lson::Float(3.14)))
        } else {
            assert!(false)
        }
    }
}
