use crate::lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal};

use super::{logic_value::LogicValue, LibrettoParsable, ParseResult};

//==================================================================================================
//          Logic Unary Expression
//==================================================================================================

#[derive(Debug)]
pub struct LogicUnaryExpr {
    is_negative: bool,
    value: LogicValue,
}

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicUnaryExpr {
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> ParseResult<Self> {
        let is_negative = queue.pop_if(LogicOrdinal::Sub).is_some();
        let result = LogicValue::parse(queue);

        match result {
            ParseResult::Parsed(value) => ParseResult::Parsed(LogicUnaryExpr{is_negative, value}),
            ParseResult::Error(err) => ParseResult::Error(err),
            ParseResult::Failure => ParseResult::Failure,
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

        // let operator = 

        ParseResult::Failure
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
        parse::LibrettoParsable, logic::lson::Lson,
    };

    use super::{LogicValue, LogicUnaryExpr};

    #[test]
    fn parse_unary_expr() {
        let mut lex = LibrettoLogicToken::lexer("-3.14");
        let mut queue = LibrettoTokenQueue::from(lex);
        let ast = LogicUnaryExpr::parse(&mut queue).unwrap();
        assert_eq!(ast.is_negative, true);
        if let LogicValue::Literal(Lson::Float(value)) = ast.value {
            assert_eq!(value, 3.14)
        } else {
            assert!(false)
        }
    }
}
