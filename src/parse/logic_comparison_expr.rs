use std::collections::HashMap;

use crate::lexer::{LibrettoLogicToken, LogicOrdinal };
use crate::lson::{LsonType, Lson};
use super::{LibrettoCompileError, LibrettoEvaluator};
use super::{logic_term_expr::LogicTermExpr, LibrettoParsable};

#[derive(Debug, PartialEq)]
pub struct LogicComparisonExpr {
    lhs : LogicTermExpr,
    rhs : Vec<(ComparisonOperator, LogicTermExpr)>
}

#[derive(PartialEq, Debug, Eq, Clone, Copy)]
pub enum ComparisonOperator {
    LessThan,
    GreaterThan,
    LessThanEqualTo,
    GreaterThanEqualTo
}

impl ToString for ComparisonOperator {
    fn to_string(&self) -> String {
        match self {
            ComparisonOperator::LessThan => "<".to_string(),
            ComparisonOperator::GreaterThan => ">".to_string(),
            ComparisonOperator::LessThanEqualTo => "<=".to_string(),
            ComparisonOperator::GreaterThanEqualTo => ">=".to_string(),
        }
    }
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicComparisonExpr {
    fn raw_check(queue: &mut crate::lexer::LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if !LogicTermExpr::raw_check(queue) {return false}
        while queue.next_is([LogicOrdinal::LessThan, LogicOrdinal::GreaterThan, LogicOrdinal::LessThanEquality, LogicOrdinal::GreaterThanEquality]) && LogicTermExpr::raw_check(queue) {}
        true
    }

    fn parse(queue: &mut crate::lexer::LibrettoTokenQueue<'a, LibrettoLogicToken>, errors: &mut Vec<super::LibrettoCompileError>) -> Option<Self> {
        let lhs = LogicTermExpr::parse(queue, errors).unwrap();
        let mut rhs = Vec::new();
        
        loop {
            queue.reset();
            if queue.next_is([LogicOrdinal::LessThan, LogicOrdinal::GreaterThan, LogicOrdinal::LessThanEquality, LogicOrdinal::GreaterThanEquality]) && LogicTermExpr::raw_check(queue) {
                let operator = {
                    let token = queue.pop();
                    match token {
                        Some(LibrettoLogicToken::LessThan) => ComparisonOperator::LessThan,
                        Some(LibrettoLogicToken::GreaterThan) => ComparisonOperator::GreaterThan,
                        Some(LibrettoLogicToken::LessThanEquality) => ComparisonOperator::LessThanEqualTo,
                        Some(LibrettoLogicToken::GreaterThanEquality) => ComparisonOperator::GreaterThanEqualTo,
                        _ => ComparisonOperator::LessThan
                    }
                };
                let value = LogicTermExpr::parse(queue, errors);
                if value.is_some() {
                    rhs.push((operator, value.unwrap()));
                }
            } else {
                break;
            }
        }
        
        Some(LogicComparisonExpr { lhs, rhs })
    }

    fn validate(&self, errors: &mut Vec<super::LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> LsonType {
        let mut lhs_type = self.lhs.validate(errors, type_map);

        if !self.rhs.is_empty() {
            for (op, rhs) in &self.rhs {
                let rhs_type = rhs.validate(errors, type_map);
                if let LsonType::None = lhs_type.get_comparison_type(rhs_type) {
                    errors.push(LibrettoCompileError::InvalidOperationError(lhs_type.to_string(), op.to_string(), rhs_type.to_string()));
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

fn get_comaprison_type(lhs : &LsonType, op : &ComparisonOperator, rhs : &LsonType) -> LsonType {
    lhs.get_comparison_type(*rhs)
}

impl LibrettoEvaluator for LogicComparisonExpr {
    fn evaluate(&self, runtime: &mut crate::runtime::LibrettoRuntime) -> crate::lson::Lson {
        let mut cardnality = true;
        let mut v1 = self.lhs.evaluate(runtime);
        if !self.rhs.is_empty() {
            for (op, rhs) in &self.rhs {
                let v2 = rhs.evaluate(runtime);
                match op {
                    ComparisonOperator::LessThan => if !(v1 < v2) { cardnality = false },
                    ComparisonOperator::GreaterThan => if !(v1 > v2) { cardnality = false },
                    ComparisonOperator::LessThanEqualTo => if !(v1 <= v2) { cardnality = false },
                    ComparisonOperator::GreaterThanEqualTo => if !(v1 >= v2) { cardnality = false },
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
    fn check_factor_expr() {
        check_expr::<LogicComparisonExpr>("!false", 2);
        check_expr::<LogicComparisonExpr>("-12", 2);
        check_expr::<LogicComparisonExpr>("3.14", 1);
        check_expr::<LogicComparisonExpr>("2", 1);
        check_expr::<LogicComparisonExpr>("2*4*6*8", 7);
        check_expr::<LogicComparisonExpr>("10 < 15 < 20", 5);
    }

    #[test]
    fn parse_factor_expr() {
        let ast = parse_expr::<LogicComparisonExpr>("2+4*6 >= 5");
        assert_eq!(ast.lhs, parse_expr::<LogicTermExpr>("2+4*6"));
        assert_eq!(ast.rhs, vec![(ComparisonOperator::GreaterThanEqualTo, parse_expr::<LogicTermExpr>("5"))]);

        let ast = parse_expr::<LogicComparisonExpr>("10 < 15 < 20");
        assert_eq!(ast.rhs.len(), 2);
    }

    #[test]
    fn validate_factor_expr() {
        validate_expr::<LogicComparisonExpr>("!false", 0, LsonType::Bool);
        validate_expr::<LogicComparisonExpr>("2 * 2", 0, LsonType::Int);
        validate_expr::<LogicComparisonExpr>("10 < 15 < 20", 0, LsonType::Bool);
    }

    #[test]
    fn eval_term_expr() {
        evaluate_expr::<LogicComparisonExpr>("false", Lson::Bool(false));
        evaluate_expr::<LogicComparisonExpr>("2*2", Lson::Int(4));
        evaluate_expr::<LogicComparisonExpr>("2/2", Lson::Int(1));
        evaluate_expr::<LogicComparisonExpr>("5/2.5", Lson::Float(2.0));
        evaluate_expr::<LogicComparisonExpr>("2*2+2*2", Lson::Int(8));
        evaluate_expr::<LogicComparisonExpr>("10 < 15 < 20 > 15 > 10", Lson::Bool(true));
    }
}