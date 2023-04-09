use std::collections::HashMap;

use crate::{lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal}, parse_ast, lson::Lson, runtime::LibrettoRuntime};
use crate::lson::LsonType;
use super::{logic_unary_expr::LogicUnaryExpr, LibrettoParsable, LibrettoCompileError, LibrettoEvaluator};

//==================================================================================================
//          Factor Expression
//==================================================================================================

#[derive(Debug, PartialEq)]
pub enum FactorOperator {
    Mult,
    Div
}

impl ToString for FactorOperator {
    fn to_string(&self) -> String {
        match self {
            FactorOperator::Mult => "*".to_string(),
            FactorOperator::Div => "/".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
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

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> LsonType {
        let lhs = self.lhs.validate(errors, type_map);

        if let Some((op, rhs)) = self.rhs.first() {
            let rhs = rhs.validate(errors, type_map);
            let mut expected_type = get_factor_type(&lhs, op, &rhs);

            if expected_type == LsonType::None {
                errors.push(LibrettoCompileError::InvalidOperationError(lhs.to_string(), op.to_string(), rhs.to_string()));
                return LsonType::None;
            }

            for i in 1..self.rhs.len() {
                let (inner_op, inner)= &self.rhs[i];
                let inner_type = inner.validate(errors, type_map);
                let op_type = get_factor_type(&expected_type, inner_op, &inner_type);
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

fn get_factor_type(lhs : &LsonType, op : &FactorOperator, rhs : &LsonType) -> LsonType {
    match op {
        FactorOperator::Mult => lhs.get_product_type(*rhs),
        FactorOperator::Div => lhs.get_quotient_type(*rhs),
    }
}

impl LibrettoEvaluator for LogicFactorExpr {
    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> Lson {
        let mut v1 = self.lhs.evaluate(runtime);
        for (op, rhs) in &self.rhs {
            let v2 = rhs.evaluate(runtime);
            match op {
                FactorOperator::Mult => v1 = v1 * v2,
                FactorOperator::Div => v1 = v1 / v2,
            };
        }
        v1
    }
}

//==================================================================================================
//          Factor Expression
//==================================================================================================

#[cfg(test)]
mod tests {

    use crate::{
        lson::{Lson, LsonType},
        parse::test_util::*,
    };

    use super::{LogicUnaryExpr, LogicFactorExpr, FactorOperator};

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

    #[test]
    fn validate_factor_expr() {
        validate_expr::<LogicFactorExpr>("!false", 0, LsonType::Bool);
        validate_expr::<LogicFactorExpr>("2 * 2", 0, LsonType::Int);
        validate_expr::<LogicFactorExpr>("false * 3", 1, LsonType::None);
    }

    #[test]
    fn eval_factor_expr() {
        evaluate_expr::<LogicFactorExpr>("false", Lson::Bool(false));
        evaluate_expr::<LogicFactorExpr>("2*2", Lson::Int(4));
    }
}