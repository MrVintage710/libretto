use std::collections::HashMap;

use crate::{lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal}, parse_ast, lson::Lson, runtime::LibrettoRuntime, compiler::{LibrettoCompiletime, LibrettoCompileError}};
use crate::lson::LsonType;
use super::{logic_value::LogicValue, LibrettoParsable, LibrettoEvaluator};

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

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
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
        let value = parse_ast!(LogicValue, queue, compile_time);

        Some(LogicUnaryExpr {
            operator,
            value,
        })
    }

    fn validate(&self, compile_time : &mut LibrettoCompiletime) -> LsonType {
        let lson_type = self.value.validate(compile_time);

        if let Some(op) = &self.operator {
            match op {
                UnaryOperator::Negative => {
                    match lson_type {
                        LsonType::None => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "null".to_string())),
                        LsonType::String => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "string".to_string())),
                        LsonType::Bool => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "bool".to_string())),
                        LsonType::Array => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "array".to_string())),
                        LsonType::Struct => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "struct".to_string())),
                        LsonType::Function => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "function".to_string())),
                        _ => {}
                    }
                },
                UnaryOperator::Bang => {
                    match lson_type {
                        LsonType::None => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "null".to_string())),
                        LsonType::Int => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "int".to_string())),
                        LsonType::Float => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "float".to_string())),
                        LsonType::String => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "string".to_string())),
                        LsonType::Array => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "array".to_string())),
                        LsonType::Struct => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "struct".to_string())),
                        LsonType::Function => compile_time.push_error(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "function".to_string())),
                        _ => {}
                    }
                },
            }
        }
        lson_type
    }
}

impl LibrettoEvaluator for LogicUnaryExpr {
    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> Lson {
        let value = self.value.evaluate(runtime);
        if let Some(op) = &self.operator {
            match op {
                UnaryOperator::Negative => -value,
                UnaryOperator::Bang => !value,
            }
        } else {
            value
        }

    }
}

//==================================================================================================
//          Logic Unary Tests
//==================================================================================================

#[cfg(test)]
mod tests {

    use crate::{
        lson::{Lson, LsonType},
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

    #[test]
    fn evaluate_unary_expr() {
        evaluate_expr::<LogicUnaryExpr>("!false", true.into());
        evaluate_expr::<LogicUnaryExpr>("bar", true.into());
    }
}