use std::ops::Add;

use crate::{
    lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal, Ordinal}, parse_ast, logic::lson::Lson,
};

use super::{logic_value::LogicValue, LibrettoCompileError, LibrettoParsable, ParseResult};

//==================================================================================================
//          Logic Unary Expression
//==================================================================================================

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Negative,
    Bang,
}

#[derive(Debug, PartialEq)]
pub struct LogicUnaryExpr {
    operator: Option<UnaryOperator>,
    value: LogicValue,
}

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicUnaryExpr {
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self> {
        let operator = queue.pop_if_next_is([LogicOrdinal::Bang, LogicOrdinal::Sub]);
        let operator = if let Some(token) = operator {
            match token {
                LibrettoLogicToken::Sub => Some(UnaryOperator::Negative),
                LibrettoLogicToken::Bang => Some(UnaryOperator::Bang),
                _ => None
            }
        } else {
            None
        };
        let value = parse_ast!(LogicValue, queue, errors);

        Some(LogicUnaryExpr {
            operator,
            value,
        })
    }

    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        queue.next_is([LogicOrdinal::Bang, LogicOrdinal::Sub]);
        if LogicValue::raw_check(queue) {
            return true;
        } else {
            false
        }
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>) {
        self.value.validate(errors);
        if let Some(op) = &self.operator {
            match op {
                UnaryOperator::Negative => {
                    if let LogicValue::Literal(value) = &self.value {
                        match value {
                            Lson::None => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "null".to_string())),
                            Lson::String(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "string".to_string())),
                            Lson::Bool(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "bool".to_string())),
                            Lson::Array(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "array".to_string())),
                            Lson::Struct(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "struct".to_string())),
                            Lson::Function(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "function".to_string())),
                            _ => {}
                        }
                    }
                },
                UnaryOperator::Bang => {
                    if let LogicValue::Literal(value) = &self.value {
                        match value {
                            Lson::None => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "null".to_string())),
                            Lson::String(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "string".to_string())),
                            Lson::Float(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "float".to_string())),
                            Lson::Int(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "int".to_string())),
                            Lson::Array(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "array".to_string())),
                            Lson::Struct(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "struct".to_string())),
                            Lson::Function(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "function".to_string())),
                            _ => {}
                        }
                    }
                },
            }
        }
    }
}

//==================================================================================================
//          Additive Expression
//==================================================================================================

#[derive(Debug, PartialEq)]
pub enum AdditionOperator {
    Plus,
    Minus
}

#[derive(Debug)]
pub struct LogicAdditiveExpr {
    lhs: LogicUnaryExpr,
    rhs: Option<LogicUnaryExpr>,
    operator : Option<AdditionOperator>,
}

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicAdditiveExpr {
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self> {
        
        let lhs = parse_ast!(LogicUnaryExpr, queue, errors);
        let operator = {
            if let Some(op) = queue.pop_if_next_is([LogicOrdinal::Add, LogicOrdinal::Sub]) {
                match op {
                    LibrettoLogicToken::Add => Some(AdditionOperator::Plus),
                    LibrettoLogicToken::Sub => Some(AdditionOperator::Minus),
                    _ => {
                        errors.push(LibrettoCompileError::ParseCheckNotThorough("LogicAdditiveExpr".to_string()));
                        None
                    }
                }
            } else {
                None
            }
        };
        let rhs = LogicUnaryExpr::checked_parse(queue, errors);

        Some(LogicAdditiveExpr {
            lhs,
            rhs,
            operator,
        })
        //This needs work

        // queue.reset();
        // let lhs = LogicUnaryExpr::parse(queue, errors);
        // let operator = queue.pop_if_next_is([LogicOrdinal::Add, LogicOrdinal::Sub]);
        // let rhs = parse_ast!(LogicUnaryExpr, queue);

        // if let Some(operator) = operator {
        //     let is_adding = match operator {
        //         LibrettoLogicToken::Add => true,
        //         LibrettoLogicToken::Sub => false,
        //         _ => return ParseResult::Error("Not a valid operator".to_owned()),
        //     };

        //     return ParseResult::Parsed(LogicAdditiveExpr {
        //         lhs,
        //         rhs: Some(rhs),
        //         is_adding,
        //     });
        // }

        // ParseResult::Failure
    }

    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if !LogicUnaryExpr::raw_check(queue) {
            return false;
        };
        queue.next_is([LogicOrdinal::Add, LogicOrdinal::Sub]);
        LogicUnaryExpr::raw_check(queue);
        true
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>) {
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
        logic::lson::Lson,
        parse::{self, LibrettoParsable, ParseResult, logic_expr::AdditionOperator},
    };

    use super::{LogicUnaryExpr, LogicValue, UnaryOperator, LogicAdditiveExpr};

    fn check_expr<'a, T: LibrettoParsable<'a, LibrettoLogicToken>>(
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

    fn parse_expr<'a, T: LibrettoParsable<'a, LibrettoLogicToken>>(source: &'a str) -> T {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let result = T::checked_parse(&mut queue, &mut Vec::new());
        assert!(result.is_some());
        result.unwrap()
    }

    fn validate_expr<'a, T: LibrettoParsable<'a, LibrettoLogicToken>>(
        source: &'a str,
        number_of_errors: usize,
    ) -> Vec<parse::LibrettoCompileError> {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let mut errors = Vec::new();
        let ast = T::checked_parse(&mut queue, &mut errors);
        assert!(ast.is_some());
        let ast = ast.unwrap();
        ast.validate(&mut errors);
        assert_eq!(errors.len(), number_of_errors);
        errors
    }

    #[test]
    fn check_unary_expr() {
        check_expr::<LogicUnaryExpr>("!false", 2);
        check_expr::<LogicUnaryExpr>("-12", 2);
        check_expr::<LogicUnaryExpr>("3.14", 1);
    }

    #[test]
    fn parse_unary_expr() {
        let ast = parse_expr::<LogicUnaryExpr>("!false");
        assert_eq!(ast.operator, Some(UnaryOperator::Bang));
        assert_eq!(ast.value, LogicValue::Literal(Lson::Bool(false)));

        let ast = parse_expr::<LogicUnaryExpr>("-12");
        assert_eq!(ast.operator, Some(UnaryOperator::Negative));
        assert_eq!(ast.value, LogicValue::Literal(Lson::Int(12)));

        let ast = parse_expr::<LogicUnaryExpr>("3.14");
        assert_eq!(ast.operator, None);
        assert_eq!(ast.value, LogicValue::Literal(Lson::Float(3.14)));
    }

    #[test]
    fn validate_unary_expr() {
        validate_expr::<LogicUnaryExpr>("!false", 0);
        validate_expr::<LogicUnaryExpr>("-1", 0);
        validate_expr::<LogicUnaryExpr>("-false", 1);
    }

    #[test]
    fn check_additive_expr() {
        check_expr::<LogicAdditiveExpr>("!false", 2);
        check_expr::<LogicAdditiveExpr>("-12", 2);
        check_expr::<LogicAdditiveExpr>("3.14", 1);
        check_expr::<LogicAdditiveExpr>("2+2", 3);
    }

    #[test]
    fn parse_additive_expr() {
        let ast = parse_expr::<LogicAdditiveExpr>("!false");
        assert_eq!(ast.operator, None);
        assert_eq!(ast.lhs, LogicUnaryExpr{ operator: Some(UnaryOperator::Bang), value: LogicValue::Literal(Lson::Bool(false)) });

        let ast = parse_expr::<LogicAdditiveExpr>("2+2");
        assert_eq!(ast.operator, Some(AdditionOperator::Plus));
        assert_eq!(ast.lhs, LogicUnaryExpr{ operator: None, value: LogicValue::Literal(Lson::Int(2)) });
        assert_eq!(ast.rhs, Some(LogicUnaryExpr{ operator: None, value: LogicValue::Literal(Lson::Int(2)) }));
    }
}
