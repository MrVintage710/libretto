use std::collections::HashMap;

use crate::{lexer::{LibrettoTokenQueue, LibrettoLogicToken, LogicOrdinal}, lson::LsonType, compiler::LibrettoCompiletime};

use super::{logic_equality_expr::LogicEqualityExpr, LibrettoParsable, logic_assignment_stmt::LogicAssignmentStatement, util::KeyValuePair};

pub struct LogicLetStatement {
    identifier: String,
    implicit_type: Option<LsonType>,
    value : LogicEqualityExpr
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicLetStatement {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
//        let first = queue.next_is(LogicOrdinal::Let) &&
//        queue.next_is(LogicOrdinal::Identifier);
//        queue.mark();
//        if first && KeyValuePair::<LsonType, LibrettoLogicToken>::raw_check(queue) && queue.next_is(LogicOrdinal::Semicolon) {
//            return true;
//        }
//        queue.rewind();
//        if first && KeyValuePair::<LsonType, LibrettoLogicToken>::raw_check(queue) && queue.next_is(LogicOrdinal::Equals) &&
        false
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
        todo!()
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
        
        // check_expr("3.14");
        // check_expr("\"Hello World\"");
    }
}