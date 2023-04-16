use std::collections::HashMap;

use crate::{lexer::{LibrettoTokenQueue, LibrettoLogicToken, LogicOrdinal}, lson::LsonType, parse_ast, compiler::{LibrettoCompileError, LibrettoCompiletime}};

use super::{logic_equality_expr::LogicEqualityExpr, LibrettoParsable, logic_expr::LogicExpr};

pub struct LogicAssignmentStatement {
    ident: String,
    value : LogicExpr
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicAssignmentStatement {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        queue.next_is(LogicOrdinal::Identifier) &&
        queue.next_is(LogicOrdinal::Equals) &&
        LogicExpr::raw_check(queue)
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
        let ident = if let Some(LibrettoLogicToken::Identifier(ident)) = queue.pop() {
            ident
        } else {
            return None
        };
        queue.pop();
        let value = parse_ast!(LogicExpr, queue, compile_time);
        Some(LogicAssignmentStatement{ident, value})
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
    fn check_assign_stmt() {
        check_expr::<LogicAssignmentStatement>("test = 2", 3);
        check_expr::<LogicAssignmentStatement>("test = layer ? true", 5);
        // check_expr("3.14");
        // check_expr("\"Hello World\"");
    }

    #[test]
    fn parse_assign_stmt() {
        let ast = parse_expr::<LogicAssignmentStatement>("test = 2");
        // check_expr("3.14");
    // check_expr("\"Hello World\"");
    }
}