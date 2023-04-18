use std::collections::HashMap;

use crate::compiler::{LibrettoCompiletime, LibrettoCompileError};
use crate::lexer::{LibrettoLogicToken, LogicOrdinal, LibrettoTokenQueue };
use crate::lson::{LsonType, Lson};
use crate::runtime::LibrettoRuntime;
use super::{LibrettoEvaluator};
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
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if !LogicComparisonExpr::raw_check(queue) {return false}
        while queue.next_is([LogicOrdinal::Equality, LogicOrdinal::InverseEquality]) && LogicComparisonExpr::raw_check(queue) {}
        true
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
        let lhs = LogicComparisonExpr::parse(queue, compile_time).unwrap();
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
                let value = LogicComparisonExpr::parse(queue, compile_time);
                if value.is_some() {
                    rhs.push((operator, value.unwrap()));
                }
            } else {
                break;
            }
        }

        Some(LogicEqualityExpr { lhs, rhs })
    }

    fn validate(&self, compile_time : &mut LibrettoCompiletime) -> LsonType {
        let mut lhs_type = self.lhs.validate(compile_time);

        if !self.rhs.is_empty() {
            for (op, rhs) in &self.rhs {
                let rhs_type = rhs.validate(compile_time);
                if let LsonType::None = lhs_type.get_comparison_type(rhs_type) {
                    compile_time.push_error(LibrettoCompileError::InvalidOperationError(lhs_type.to_string(), op.to_string(), rhs_type.to_string()));
                    return LsonType::None
                }
                lhs_type = rhs_type;
            }
            LsonType::Bool
        } else {
            lhs_type
        }
    }
}

fn get_equality_type(lhs : &LsonType, op : &EqualityOperator, rhs : &LsonType) -> LsonType {
    lhs.get_equality_type(*rhs)
}

impl LibrettoEvaluator for LogicEqualityExpr {
    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> Lson {
        let mut cardnality = true;
        let mut v1 = self.lhs.evaluate(runtime);
        if !self.rhs.is_empty() {
            for (op, rhs) in &self.rhs {
                let v2 = rhs.evaluate(runtime);
                match op {
                    EqualityOperator::EqualTo => if !(v1 == v2) { cardnality = false },
                    EqualityOperator::NotEqualTo => if !(v1 != v2) { cardnality = false }
                };
                v1 = v2;
            }
            Lson::Bool(cardnality)
        } else {
            v1
        }
    }
}

//==================================================================================================
//          Comparison Expression Tests
//==================================================================================================

#[cfg(test)]
mod tests {

    use crate::{
        lson::LsonType,
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

    #[test]
    fn eval_term_expr() {
        evaluate_expr::<LogicComparisonExpr>("false", Lson::Bool(false));
        evaluate_expr::<LogicComparisonExpr>("2*2", Lson::Int(4));
        evaluate_expr::<LogicComparisonExpr>("2/2", Lson::Int(1));
        evaluate_expr::<LogicComparisonExpr>("5/2.5", Lson::Float(2.0));
        evaluate_expr::<LogicComparisonExpr>("2*2+2*2", Lson::Int(8));
        evaluate_expr::<LogicComparisonExpr>("10 < 15 < 20 > 15 > 10", Lson::Bool(true));
        evaluate_expr::<LogicComparisonExpr>("true != false", Lson::Bool(true));
    }
}