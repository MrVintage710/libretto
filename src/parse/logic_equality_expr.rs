use std::collections::HashMap;

use crate::lexer::{LibrettoLogicToken, LogicOrdinal };
use crate::logic::lson::{LsonType};
use super::LibrettoCompileError;
use super::logic_comparison_expr::LogicComparisonExpr;
use super::{logic_term_expr::LogicTermExpr, LibrettoParsable};

#[derive(Debug, PartialEq)]
pub struct LogicEqualityExpr {
    lhs : LogicComparisonExpr,
    rhs : Vec<(EqualityOperator, LogicComparisonExpr)>
}

#[derive(PartialEq, Debug, Eq, Clone, Copy)]
pub enum EqualityOperator {
    EqualTo,
    NotEqualTo
}

impl ToString for EqualityOperator {
    fn to_string(&self) -> String {
        match self {
            EqualityOperator::EqualTo => "==".to_string(),
            EqualityOperator::NotEqualTo => "!=".to_string(),
        }
    }
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicEqualityExpr {
    fn raw_check(queue: &mut crate::lexer::LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if !LogicComparisonExpr::raw_check(queue) {return false}
        while queue.next_is([LogicOrdinal::Equality, LogicOrdinal::InverseEquality]) && LogicComparisonExpr::raw_check(queue) {}
        true
    }

    fn parse(queue: &mut crate::lexer::LibrettoTokenQueue<'a, LibrettoLogicToken>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self> {
        let lhs = LogicComparisonExpr::parse(queue, errors).unwrap();
        let mut rhs = Vec::new();

        loop {
            queue.reset();
            if queue.next_is([LogicOrdinal::Equality, LogicOrdinal::InverseEquality]) && LogicTermExpr::raw_check(queue) {
                let operator = {
                    let token = queue.pop();
                    match token {
                        Some(LibrettoLogicToken::Equality) => EqualityOperator::EqualTo,
                        Some(LibrettoLogicToken::InverseEquality) => EqualityOperator::NotEqualTo,
                        _ => EqualityOperator::NotEqualTo
                    }
                };
                let value = LogicComparisonExpr::parse(queue, errors);
                if value.is_some() {
                    rhs.push((operator, value.unwrap()));
                }
            } else {
                break;
            }
        }

        Some(LogicEqualityExpr { lhs, rhs })
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> LsonType {
        let lhs = self.lhs.validate(errors, type_map);

        if let Some((op, rhs)) = self.rhs.first() {
            let rhs = rhs.validate(errors, type_map);
            let expected_type = get_equality_type(&lhs, op, &rhs);

            if expected_type == LsonType::None {
                errors.push(LibrettoCompileError::InvalidOperationError(lhs.to_string(), op.to_string(), rhs.to_string()));
                return LsonType::None;
            }

            for i in 1..self.rhs.len() {
                let (inner_op, inner)= &self.rhs[i];
                let inner_type = inner.validate(errors, type_map);
                let op_type = get_equality_type(&expected_type, inner_op, &inner_type);
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

fn get_equality_type(lhs : &LsonType, op : &EqualityOperator, rhs : &LsonType) -> LsonType {
    lhs.get_equality_type(*rhs)
}

//==================================================================================================
//          Comparison Expression Tests
//==================================================================================================

#[cfg(test)]
mod tests {

    use crate::{
        logic::lson::LsonType,
        parse::test_util::*,
    };

    use super::*;

    #[test]
    fn check_equality_expr() {
        check_expr::<LogicEqualityExpr>("true != false", 3);
        check_expr::<LogicEqualityExpr>("true != 5.5 <= 10", 5);
    }

    #[test]
    fn parse_equality_expr() {
        let ast = parse_expr::<LogicEqualityExpr>("true != false");
        assert_eq!(ast.lhs, parse_expr::<LogicComparisonExpr>("true"));
        assert_eq!(ast.rhs, vec![(EqualityOperator::NotEqualTo, parse_expr::<LogicComparisonExpr>("false"))]);
    }

    #[test]
    fn validate_equality_expr() {
        validate_expr::<LogicEqualityExpr>("!false", 0, LsonType::Bool);
        validate_expr::<LogicEqualityExpr>("2 * 2", 0, LsonType::Int);
    }
}