use std::collections::HashMap;

use crate::{lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal}, parse_ast};
use crate::logic::lson::LsonType;
use super::{logic_value::LogicValue, LibrettoParsable, LibrettoCompileError};

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
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        queue.next_is([LogicOrdinal::Bang, LogicOrdinal::Sub]);
        if LogicValue::raw_check(queue) {
            return true;
        } else {
            false
        }
    }

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

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> LsonType {
        let lson_type = self.value.validate(errors, type_map);

        if let Some(op) = &self.operator {
            match op {
                UnaryOperator::Negative => {
                    match lson_type {
                        LsonType::None => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "null".to_string())),
                        LsonType::String => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "string".to_string())),
                        LsonType::Bool => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "bool".to_string())),
                        LsonType::Array => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "array".to_string())),
                        LsonType::Struct => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "struct".to_string())),
                        LsonType::Function => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "function".to_string())),
                        _ => {}
                    }
                },
                UnaryOperator::Bang => {
                    match lson_type {
                        LsonType::None => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "null".to_string())),
                        LsonType::Int => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "int".to_string())),
                        LsonType::Float => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "float".to_string())),
                        LsonType::String => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "string".to_string())),
                        LsonType::Array => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "array".to_string())),
                        LsonType::Struct => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "struct".to_string())),
                        LsonType::Function => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "function".to_string())),
                        _ => {}
                    }
                },
            }
        }
        lson_type
    }
}

//==================================================================================================
//          Logic Unary Tests
//==================================================================================================

#[cfg(test)]
mod tests {

    use crate::{
        logic::lson::{Lson, LsonType},
        parse::{test_util::*, logic_value::LogicValue},
    };

    use super::{LogicUnaryExpr, UnaryOperator};

    #[test]
    fn check_unary_expr() {
        check_expr::<LogicUnaryExpr>("!false", 2);
        check_expr::<LogicUnaryExpr>("-12", 2);
        check_expr::<LogicUnaryExpr>("3.14", 1);
        check_expr::<LogicUnaryExpr>("foo", 1);
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
        validate_expr::<LogicUnaryExpr>("!false", 0, LsonType::Bool);
        validate_expr::<LogicUnaryExpr>("-1", 0, LsonType::Int);
        validate_expr::<LogicUnaryExpr>("-false", 1, LsonType::Bool);
    }
}