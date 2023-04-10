use std::collections::HashMap;

use crate::{lson::{Lson, LsonType}, lexer::{LibrettoLogicToken, LogicOrdinal, LibrettoTokenQueue}, parse_ast, parse::LibrettoCompileError::ExprDefaultTypeMissmatch};
use super::{logic_equality_expr::LogicEqualityExpr, LibrettoParsable, LibrettoCompileError};

pub struct LogicExpr {
    expr : LogicEqualityExpr,
    default : Option<Lson>
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicExpr {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        let mut check = LogicEqualityExpr::raw_check(queue);
        if queue.next_is(LogicOrdinal::Question) {
            check &= Lson::raw_check(queue);
        }
        check
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self> {
        let expr = parse_ast!(LogicEqualityExpr, queue, errors);
        let default = if let Some(_) = queue.pop_if_next_is(LogicOrdinal::Question) {
            Some(parse_ast!(Lson, queue, errors))
        } else {
            None
        };
        Some(LogicExpr{expr, default})
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> LsonType {
        let expected_type = self.expr.validate(errors, type_map);
        if let Some(lson) = &self.default {
            let default_type = lson.validate(errors, type_map);
            if expected_type != default_type {
                errors.push(ExprDefaultTypeMissmatch(expected_type.to_string(), default_type.to_string()))
            }
        };
        expected_type
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        lson::{Lson, LsonType},
        parse::test_util::*,
        parse::logic_equality_expr::LogicEqualityExpr
    };

    use super::*;

    #[test]
    fn check_logic_expr() {
        check_expr::<LogicExpr>("foo ? true", 3);
        check_expr::<LogicExpr>("foo", 1);
        // check_expr("3.14");
        // check_expr("\"Hello World\"");
    }

    #[test]
    fn parse_logic_expr() {
        let ast = parse_expr::<LogicExpr>("foo ? true");
        assert_eq!(ast.expr, parse_expr::<LogicEqualityExpr>("foo"));
        assert!(ast.default.is_some());
        assert_eq!(ast.default.unwrap(), parse_expr::<Lson>("true"));
        
        let ast = parse_expr::<LogicExpr>("bar");
        assert_eq!(ast.expr, parse_expr::<LogicEqualityExpr>("bar"));
        assert!(ast.default.is_none());
    }

    #[test]
    fn validate_logic_expr() {
        validate_expr::<LogicExpr>("10 < 15 ? false", 0, LsonType::Bool);
        validate_expr::<LogicExpr>("\"test\" ? true", 1, LsonType::String)
    }
}

