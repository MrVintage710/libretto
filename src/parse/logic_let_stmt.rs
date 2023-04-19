use std::collections::HashMap;

use crate::{lexer::{LibrettoTokenQueue, LibrettoLogicToken, LogicOrdinal}, lson::LsonType, compiler::LibrettoCompiletime};
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
        todo!()
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
}