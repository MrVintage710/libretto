use crate::lexer::{LibrettoLogicToken, LogicOrdinal };
use crate::logic::lson::{LsonType};
use super::LibrettoCompileError;
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

    fn validate(&self, errors: &mut Vec<super::LibrettoCompileError>, type_map : &mut std::collections::HashMap<String, crate::logic::lson::LsonType>) -> crate::logic::lson::LsonType {
        let lhs = self.lhs.validate(errors, type_map);

        if let Some((op, rhs)) = self.rhs.first() {
            let rhs = rhs.validate(errors, type_map);
            let expected_type = get_comaprison_type(&lhs, op, &rhs);

            if expected_type == LsonType::None {
                errors.push(LibrettoCompileError::InvalidOperationError(lhs.to_string(), op.to_string(), rhs.to_string()));
                return LsonType::None;
            }

            for i in 1..self.rhs.len() {
                let (inner_op, inner)= &self.rhs[i];
                let inner_type = inner.validate(errors, type_map);
                let op_type = get_comaprison_type(&expected_type, inner_op, &inner_type);
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

fn get_comaprison_type(lhs : &LsonType, op : &ComparisonOperator, rhs : &LsonType) -> LsonType {
    lhs.get_comparison_type(*rhs)
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
    fn check_factor_expr() {
        check_expr::<LogicComparisonExpr>("!false", 2);
        check_expr::<LogicComparisonExpr>("-12", 2);
        check_expr::<LogicComparisonExpr>("3.14", 1);
        check_expr::<LogicComparisonExpr>("2", 1);
        check_expr::<LogicComparisonExpr>("2*4*6*8", 7);
    }

    #[test]
    fn parse_factor_expr() {
        let ast = parse_expr::<LogicComparisonExpr>("2+4*6 >= 5");
        assert_eq!(ast.lhs, parse_expr::<LogicTermExpr>("2+4*6"));
        assert_eq!(ast.rhs, vec![(ComparisonOperator::GreaterThanEqualTo, parse_expr::<LogicTermExpr>("5"))]);
    }

    #[test]
    fn validate_factor_expr() {
        validate_expr::<LogicComparisonExpr>("!false", 0, LsonType::Bool);
        validate_expr::<LogicComparisonExpr>("2 * 2", 0, LsonType::Int);
    }
}