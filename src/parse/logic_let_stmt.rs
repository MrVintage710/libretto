use std::collections::HashMap;

use crate::{lexer::{LibrettoTokenQueue, LibrettoLogicToken, LogicOrdinal}, lson::{LsonType, Lson}, compiler::{LibrettoCompiletime, LibrettoCompileError}, runtime::{LibrettoEvaluator, LibrettoRuntime, LibrettoRuntimeResult}};
use crate::parse::util::TypedIdentifier;
use super::{logic_equality_expr::LogicEqualityExpr, LibrettoParsable, logic_assignment_stmt::LogicAssignmentStatement, util::KeyValuePair, logic_expr::LogicExpr};

pub struct LogicLetStatement {
    identifier: TypedIdentifier,
    value : Option<LogicExpr>
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicLetStatement {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        let first = queue.next_is(LogicOrdinal::Let) && TypedIdentifier::raw_check(queue);
        if queue.next_is(LogicOrdinal::Semicolon) {
            return true;
        }

        if queue.next_is(LogicOrdinal::Equals) && LogicExpr::raw_check(queue) && queue.next_is(LogicOrdinal::Semicolon) {
            return true;
        }

        false
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
        queue.pop();
        if let Some(identifier) = TypedIdentifier::parse(queue, compile_time) {
            let next = queue.pop();
            if let Some(LibrettoLogicToken::Semicolon) = &next {
                return Some(LogicLetStatement { identifier, value: None })
            };
            if let Some(LibrettoLogicToken::Equals) = &next {
                let value = LogicExpr::parse(queue, compile_time);
                return Some(LogicLetStatement { identifier, value })
            }
        }

        None
    }

    fn validate(&self, compile_time : &mut LibrettoCompiletime) -> LsonType {
        let ident = self.identifier.ident();
        let declared_type = self.identifier.validate(compile_time);
        let rhs_type = if let Some(rhs) = &self.value {
            rhs.validate(compile_time)
        } else {
            LsonType::None
        };

        match (declared_type, rhs_type) {
            (LsonType::None, LsonType::None) => compile_time.push_error(LibrettoCompileError::TypeNotExplicit(ident.to_string())),
            (_, LsonType::None) => compile_time.insert_variable_type(ident, declared_type),
            (LsonType::None, _) => compile_time.insert_variable_type(ident, rhs_type),
            _ => {
                if declared_type != rhs_type {
                    compile_time.push_error(LibrettoCompileError::AssignmentStatementTypeMismatch(declared_type.to_string(), rhs_type.to_string()));
                }
            }
        }

        LsonType::None
    }
}

impl LibrettoEvaluator for LogicLetStatement {
    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> LibrettoRuntimeResult {
        if let Some(lhs) = &self.value {
            let ident = self.identifier.ident();
            let value = lhs.evaluate(runtime)?;
            runtime.insert_data(ident, value);
        }

        Ok(Lson::None)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        lson::{Lson, LsonType},
        parse::test_util::*,
    };

    use super::*;

    #[test]
    fn check_let_stmt() {
        check_expr::<LogicLetStatement>("let test : bool;", 5);
        check_expr::<LogicLetStatement>("let test : bool = false;", 7);
        check_expr::<LogicLetStatement>("let test = false;", 5);
    }

    #[test]
    fn parse_let_stmt() {
        let ast = parse_expr::<LogicLetStatement>("let test : bool;");
        assert_eq!(ast.value, None);
        assert_eq!(ast.identifier, parse_expr::<TypedIdentifier>("test : bool"))
    }

    #[test]
    fn validate_let_stmt() {
        validate_expr::<LogicLetStatement>("let test;", 1, LsonType::None);
        validate_expr::<LogicLetStatement>("let test : bool = 2.0;", 1, LsonType::None);
        validate_expr::<LogicLetStatement>("let test : bool;", 0, LsonType::None);
        validate_expr::<LogicLetStatement>("let test = false;", 0, LsonType::None);
    }

    #[test]
    fn evaluate_let_stmt() {
        let rt = evaluate_expr::<LogicLetStatement>("let test : float = 2.0;", Lson::None);
        assert!(rt.has_data("test"));
        assert_eq!(rt.get_data("test"), Lson::Float(2.0));
    }
}