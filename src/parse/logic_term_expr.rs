use std::collections::HashMap;

use crate::compiler::{LibrettoCompiletime, LibrettoCompileError};
use crate::lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal};
use crate::lson::{LsonType, Lson};
use crate::parse_ast;
use crate::runtime::{LibrettoRuntime, LibrettoEvaluator, LibrettoRuntimeResult};
use super::logic_factor_expr::LogicFactorExpr;
use super::LibrettoParsable;

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

#[derive(Debug, PartialEq)]
pub struct LogicTermExpr {
    lhs: LogicFactorExpr,
    rhs: Vec<(TermOperator, LogicFactorExpr)>,
}

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicTermExpr {

    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if !LogicFactorExpr::raw_check(queue) {
            return false;
        };
        while queue.next_is([LogicOrdinal::Add, LogicOrdinal::Sub]) && LogicFactorExpr::raw_check(queue) {}
        true
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
        let lhs = parse_ast!(LogicFactorExpr, queue, compile_time);
        let mut rhs = Vec::new();

        loop {
            queue.reset();
            if queue.next_is([LogicOrdinal::Add, LogicOrdinal::Sub]) && LogicFactorExpr::raw_check(queue) {
                let operator = {
                    let token = queue.pop();
                    if let Some(LibrettoLogicToken::Sub) = token {
                        TermOperator::Minus
                    } else {
                        TermOperator::Plus
                    }
                };
                let value = LogicFactorExpr::parse(queue, compile_time);
                if value.is_some() {
                    rhs.push((operator, value.unwrap()));
                }
            } else {
                break;
            }
        }

        Some(LogicTermExpr { lhs, rhs })
    }

    fn validate(&self, compile_time : &mut LibrettoCompiletime) -> LsonType {
        let lhs = self.lhs.validate(compile_time);

        if let Some((op, rhs)) = self.rhs.first() {
            let rhs = rhs.validate(compile_time);
            let mut expected_type = get_term_type(&lhs, op, &rhs);

            if expected_type == LsonType::None {
                compile_time.push_error(LibrettoCompileError::InvalidOperationError(lhs.to_string(), op.to_string(), rhs.to_string()));
                return LsonType::None;
            }

            for i in 1..self.rhs.len() {
                let (inner_op, inner)= &self.rhs[i];
                let inner_type = inner.validate(compile_time);
                let op_type = get_term_type(&expected_type, inner_op, &inner_type);
                if op_type == LsonType::None{
                    compile_time.push_error(LibrettoCompileError::InvalidOperationError(expected_type.to_string(), op.to_string(), inner_type.to_string()));
                    return LsonType::None;
                }
            }

            expected_type
        } else {
            lhs
        }
    }
}

fn get_term_type(lhs : &LsonType, op : &TermOperator, rhs : &LsonType) -> LsonType {
    match op {
        TermOperator::Plus => lhs.get_sum_type(*rhs),
        TermOperator::Minus => lhs.get_difference_type(*rhs),
    }
}

impl LibrettoEvaluator for LogicTermExpr {
    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> LibrettoRuntimeResult {
        let mut v1 = self.lhs.evaluate(runtime)?;
        for (op, rhs) in &self.rhs {
            let v2 = rhs.evaluate(runtime)?;
            match op {
                TermOperator::Plus => v1 = v1 + v2,
                TermOperator::Minus => v1 = v1 - v2,
            };
        }
        Ok(v1)
    }
}

//==================================================================================================
//          Additive Expression Tests
//==================================================================================================

#[cfg(test)]
mod tests {

    use crate::{
        lson::{Lson, LsonType},
        parse::{test_util::*, logic_value::LogicValue, logic_factor_expr::LogicFactorExpr},
    };

    use super::{LogicTermExpr, TermOperator};

    #[test]
    fn check_term_expr() {
        check_expr::<LogicTermExpr>("!false", 2);
        check_expr::<LogicTermExpr>("-12", 2);
        check_expr::<LogicTermExpr>("3.14", 1);
        check_expr::<LogicTermExpr>("2+2+2+4*8", 9);
    }

    #[test]
    fn parse_term_expr() {
        let ast = parse_expr::<LogicTermExpr>("2+3");
        assert_eq!(ast.lhs, parse_expr::<LogicFactorExpr>("2"));
        assert_eq!(ast.rhs[0], (TermOperator::Plus, parse_expr::<LogicFactorExpr>("3")));

        let ast = parse_expr::<LogicTermExpr>("2+2+2");
        assert_eq!(ast.lhs, parse_expr::<LogicFactorExpr>("2"));
        assert_eq!(ast.rhs[0], (TermOperator::Plus, parse_expr::<LogicFactorExpr>("2")));
        assert_eq!(ast.rhs[1], (TermOperator::Plus, parse_expr::<LogicFactorExpr>("2")));
    }

    #[test]
    fn validate_term_expr() {
        validate_expr::<LogicTermExpr>("!false", 0, LsonType::Bool);
        validate_expr::<LogicTermExpr>("2 + 2 * 3", 0, LsonType::Int);
        validate_expr::<LogicTermExpr>("2 + foo", 0, LsonType::Float);
        validate_expr::<LogicTermExpr>("false + 3", 1, LsonType::None);
    }

    #[test]
    fn eval_term_expr() {
        evaluate_expr::<LogicTermExpr>("false", Lson::Bool(false));
        evaluate_expr::<LogicTermExpr>("2*2", Lson::Int(4));
        evaluate_expr::<LogicTermExpr>("2/2", Lson::Int(1));
        evaluate_expr::<LogicTermExpr>("5/2.5", Lson::Float(2.0));
        evaluate_expr::<LogicTermExpr>("2*2+2*2", Lson::Int(8));
    }
}