//==================================================================================================
//          Additive Expression
//==================================================================================================

use std::collections::HashMap;

use crate::lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal};
use crate::logic::lson::LsonType;
use crate::parse_ast;
use super::LibrettoCompileError;
use super::logic_factor_expr::LogicFactorExpr;
use super::{LibrettoParsable};

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

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self> {
        let lhs = parse_ast!(LogicFactorExpr, queue, errors);
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
                let value = LogicFactorExpr::parse(queue, errors);
                if value.is_some() {
                    rhs.push((operator, value.unwrap()));
                }
            } else {
                break;
            }
        }

        Some(LogicTermExpr { lhs, rhs })
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> LsonType {
        let lhs = self.lhs.validate(errors, type_map);

        if let Some((op, rhs)) = self.rhs.first() {
            let rhs = rhs.validate(errors, type_map);
            let mut expected_type = get_term_type(&lhs, op, &rhs);

            if expected_type == LsonType::None {
                errors.push(LibrettoCompileError::InvalidOperationError(lhs.to_string(), op.to_string(), rhs.to_string()));
                return LsonType::None;
            }

            for i in 1..self.rhs.len() {
                let (inner_op, inner)= &self.rhs[i];
                let inner_type = inner.validate(errors, type_map);
                let op_type = get_term_type(&expected_type, inner_op, &inner_type);
                if op_type == LsonType::None{
                    errors.push(LibrettoCompileError::InvalidOperationError(expected_type.to_string(), op.to_string(), inner_type.to_string()));
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

//==================================================================================================
//          Additive Expression Tests
//==================================================================================================

#[cfg(test)]
mod tests {

    use crate::{
        logic::lson::{Lson, LsonType},
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
}