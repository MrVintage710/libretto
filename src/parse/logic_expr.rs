use std::ops::Add;

use crate::{
    lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal, Ordinal}, parse_ast, logic::lson::{Lson, LsonType},
};

use super::{LibrettoCompileError, LibrettoParsable, ParseResult};

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
    value: Lson,
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
        let value = parse_ast!(Lson, queue, errors);

        Some(LogicUnaryExpr {
            operator,
            value,
        })
    }

    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        queue.next_is([LogicOrdinal::Bang, LogicOrdinal::Sub]);
        if Lson::raw_check(queue) {
            return true;
        } else {
            false
        }
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>) -> LsonType {
        if let Some(op) = &self.operator {
            match op {
                UnaryOperator::Negative => {
                    match self.value {
                        Lson::None => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "null".to_string())),
                        Lson::String(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "string".to_string())),
                        Lson::Bool(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "bool".to_string())),
                        Lson::Array(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "array".to_string())),
                        Lson::Struct(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "struct".to_string())),
                        Lson::Function(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("-".to_string(), "function".to_string())),
                        _ => {}
                    }
                },
                UnaryOperator::Bang => {
                    match self.value {
                        Lson::None => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "null".to_string())),
                        Lson::String(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "string".to_string())),
                        Lson::Float(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "float".to_string())),
                        Lson::Int(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "int".to_string())),
                        Lson::Array(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "array".to_string())),
                        Lson::Struct(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "struct".to_string())),
                        Lson::Function(_) => errors.push(LibrettoCompileError::OperationNotSupportedError("!".to_string(), "function".to_string())),
                        _ => {}
                    }
                },
            }
        }
        self.value.validate(errors)
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
    lhs: LogicFactorExpr,
    rhs: Vec<(TermOperator, LogicFactorExpr)>,
}

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicTermExpr {
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

    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if !LogicFactorExpr::raw_check(queue) {
            return false;
        };
        while queue.next_is([LogicOrdinal::Add, LogicOrdinal::Sub]) && LogicFactorExpr::raw_check(queue) {}
        true
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>) -> LsonType {
        let lhs = self.lhs.validate(errors);

        if let Some((op, rhs)) = self.rhs.first() {
            let rhs = rhs.validate(errors);
            let mut expected_type = get_term_type(&lhs, op, &rhs);

            if expected_type == LsonType::None {
                errors.push(LibrettoCompileError::InvalidOperationError(lhs.to_string(), op.to_string(), rhs.to_string()));
                return LsonType::None;
            }

            for i in 1..self.rhs.len() {
                let (inner_op, inner)= &self.rhs[i];
                let inner_type = inner.validate(errors);
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
        TermOperator::Plus => lhs.get_product_type(*rhs),
        TermOperator::Minus => lhs.get_quotient_type(*rhs),
    }
}

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

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>) -> LsonType {
        let lhs = self.lhs.validate(errors);

        if let Some((op, rhs)) = self.rhs.first() {
            let rhs = rhs.validate(errors);
            let mut expected_type = get_factor_type(&lhs, op, &rhs);

            if expected_type == LsonType::None {
                errors.push(LibrettoCompileError::InvalidOperationError(lhs.to_string(), op.to_string(), rhs.to_string()));
                return LsonType::None;
            }

            for i in 1..self.rhs.len() {
                let (inner_op, inner)= &self.rhs[i];
                let inner_type = inner.validate(errors);
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
    

//==================================================================================================
//          Tests
//==================================================================================================

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::{
        lexer::{LibrettoLogicToken, LibrettoTokenQueue},
        logic::lson::{Lson, LsonType},
        parse::{self, LibrettoParsable, ParseResult, logic_expr::{TermOperator, FactorOperator}, test_util::*},
    };

    use super::{LogicUnaryExpr, UnaryOperator, LogicTermExpr, LogicFactorExpr};

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
        assert_eq!(ast.value, Lson::Bool(false));

        let ast = parse_expr::<LogicUnaryExpr>("-12");
        assert_eq!(ast.operator, Some(UnaryOperator::Negative));
        assert_eq!(ast.value, Lson::Int(12));

        let ast = parse_expr::<LogicUnaryExpr>("3.14");
        assert_eq!(ast.operator, None);
        assert_eq!(ast.value, Lson::Float(3.14));
        
        let ast = parse_expr::<LogicUnaryExpr>("foo");
        assert_eq!(ast.operator, None);
        assert_eq!(ast.value, Lson::Ident("foo".to_string()));
    }

    #[test]
    fn validate_unary_expr() {
        validate_expr::<LogicUnaryExpr>("!false", 0, LsonType::Bool);
        validate_expr::<LogicUnaryExpr>("-1", 0, LsonType::Int);
        validate_expr::<LogicUnaryExpr>("-false", 1, LsonType::Bool);
    }

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
    }

    #[test]
    fn validate_term_expr() {
        validate_expr::<LogicTermExpr>("!false", 0, LsonType::Bool);
        validate_expr::<LogicTermExpr>("2 + 2 * 3", 0, LsonType::Int);
        validate_expr::<LogicTermExpr>("2 + test", 0, LsonType::None);
        validate_expr::<LogicTermExpr>("false + 3", 1, LsonType::None);
    }

    #[test]
    fn validate_factor_expr() {
        validate_expr::<LogicFactorExpr>("!false", 0, LsonType::Bool);
        validate_expr::<LogicFactorExpr>("2 * 2", 0, LsonType::Int);
        validate_expr::<LogicFactorExpr>("false * 3", 1, LsonType::None);
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
