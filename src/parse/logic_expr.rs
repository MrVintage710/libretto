use std::ops::Add;

use crate::{
    lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal, Ordinal}, parse_ast, logic::lson::{Lson, LsonType},
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
pub enum TermOperator {
    Plus,
    Minus
}

impl ToString for TermOperator {
    fn to_string(&self) -> String {
        match self {
            TermOperator::Plus => String::from("+"),
            TermOperator::Minus => String::from("-"),
        }
    }
}

#[derive(Debug)]
pub struct LogicTermExpr {
    lhs: LogicUnaryExpr,
    operator : Option<TermOperator>,
    rhs: Option<LogicUnaryExpr>,
}

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicTermExpr {
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self> {
        
        let lhs = parse_ast!(LogicUnaryExpr, queue, errors);
        let operator = {
            if let Some(op) = queue.pop_if_next_is([LogicOrdinal::Add, LogicOrdinal::Sub]) {
                match op {
                    LibrettoLogicToken::Add => Some(TermOperator::Plus),
                    LibrettoLogicToken::Sub => Some(TermOperator::Minus),
                    _ => {
                        errors.push(LibrettoCompileError::ParseCheckNotThoroughError("LogicAdditiveExpr".to_string()));
                        None
                    }
                }
            } else {
                None
            }
        };
        let rhs = LogicUnaryExpr::checked_parse(queue, errors);

        Some(LogicTermExpr {
            lhs,
            rhs,
            operator,
        })
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
        if let (Some(op), Some(rhs)) = (&self.operator, &self.rhs) {
            if let (LogicValue::Literal(lhs_value), LogicValue::Literal(rhs_value)) = (&self.lhs.value, &rhs.value){
                let lhs_type : LsonType = lhs_value.into();
                let rhs_type : LsonType = rhs_value.into();
                match (lhs_type, rhs_type) {
                    (LsonType::Float, LsonType::Float) |
                    (LsonType::Int, LsonType::Int) |
                    (LsonType::Int, LsonType::Float) |
                    (LsonType::Float, LsonType::Int) |
                    (LsonType::String, LsonType::Float) |
                    (LsonType::String, LsonType::Int)|
                    (LsonType::Float, LsonType::String) |
                    (LsonType::Int, LsonType::String) => {},
                    _ => errors.push(LibrettoCompileError::InvalidOperationError(op.to_string(), lhs_type.to_string(), rhs_type.to_string()))
                }
            }
        }
    }
}

//==================================================================================================
//          Multiplicative Expression
//==================================================================================================

#[derive(Debug, PartialEq)]
pub enum FactorOperator {
    Mult,
    Div
}

#[derive(Debug)]
pub struct LogicFactorExpr {
    lhs : LogicUnaryExpr,
    rhs : Vec<(FactorOperator, LogicUnaryExpr)>
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicFactorExpr {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if !LogicUnaryExpr::raw_check(queue) {
            return false;
        };
        while queue.next_is([LogicOrdinal::Mult, LogicOrdinal::Div]) && LogicUnaryExpr::raw_check(queue) {}
        true
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self> {
        let lhs = parse_ast!(LogicUnaryExpr, queue, errors);
        let mut rhs = Vec::new();

        loop {
            queue.reset();
            if queue.next_is([LogicOrdinal::Mult, LogicOrdinal::Div]) && LogicUnaryExpr::raw_check(queue) {
                let operator = {
                    let token = queue.pop();
                    if let Some(LibrettoLogicToken::Div) = token {
                        FactorOperator::Div
                    } else {
                        FactorOperator::Mult
                    }
                };
                let value = LogicUnaryExpr::parse(queue, errors);
                if value.is_some() {
                    rhs.push((operator, value.unwrap()));
                }
            } else {
                break;
            }
        }

        Some(LogicFactorExpr { lhs, rhs })
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
        parse::{self, LibrettoParsable, ParseResult, logic_expr::{TermOperator, FactorOperator}},
    };

    use super::{LogicUnaryExpr, LogicValue, UnaryOperator, LogicTermExpr, LogicFactorExpr};

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
        check_expr::<LogicTermExpr>("!false", 2);
        check_expr::<LogicTermExpr>("-12", 2);
        check_expr::<LogicTermExpr>("3.14", 1);
        check_expr::<LogicTermExpr>("2+2", 3);
    }

    #[test]
    fn parse_additive_expr() {
        let ast = parse_expr::<LogicTermExpr>("!false");
        assert_eq!(ast.operator, None);
        assert_eq!(ast.lhs, LogicUnaryExpr{ operator: Some(UnaryOperator::Bang), value: LogicValue::Literal(Lson::Bool(false)) });

        let ast = parse_expr::<LogicTermExpr>("2+2");
        assert_eq!(ast.operator, Some(TermOperator::Plus));
        assert_eq!(ast.lhs, LogicUnaryExpr{ operator: None, value: LogicValue::Literal(Lson::Int(2)) });
        assert_eq!(ast.rhs, Some(LogicUnaryExpr{ operator: None, value: LogicValue::Literal(Lson::Int(2)) }));

        let ast = parse_expr::<LogicTermExpr>("2-2");
        assert_eq!(ast.operator, Some(TermOperator::Minus));
        assert_eq!(ast.lhs, LogicUnaryExpr{ operator: None, value: LogicValue::Literal(Lson::Int(2)) });
        assert_eq!(ast.rhs, Some(LogicUnaryExpr{ operator: None, value: LogicValue::Literal(Lson::Int(2)) }));
    }

    #[test]
    fn validate_additive_expr() {
        validate_expr::<LogicTermExpr>("!false", 0);
        validate_expr::<LogicTermExpr>("2 + 2", 0);
        validate_expr::<LogicTermExpr>("false + 3", 1);
    }

    #[test]
    fn check_factor_expr() {
        check_expr::<LogicFactorExpr>("!false", 2);
        check_expr::<LogicFactorExpr>("-12", 2);
        check_expr::<LogicFactorExpr>("3.14", 1);
        check_expr::<LogicFactorExpr>("2", 1);
        check_expr::<LogicFactorExpr>("2*4*6*8", 7);
    }

    #[test]
    fn parse_factor_expr() {
        let ast = parse_expr::<LogicFactorExpr>("2*4*6");
        assert_eq!(ast.lhs, parse_expr::<LogicUnaryExpr>("2"));
        assert_eq!(ast.rhs, vec![(FactorOperator::Mult, parse_expr::<LogicUnaryExpr>("4")), (FactorOperator::Mult, parse_expr::<LogicUnaryExpr>("6"))]);
    }
}
