use std::collections::HashMap;

use crate::{lexer::{LibrettoTokenQueue, LibrettoLogicToken, LogicOrdinal}, lson::LsonType};

use super::{logic_equality_expr::LogicEqualityExpr, LibrettoParsable, LibrettoCompileError, logic_expr::LogicExpr};

pub struct LogicAssignmentStatement {
    identifier: String,
    value : LogicExpr
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicAssignmentStatement {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        queue.next_is(LogicOrdinal::Identifier) &&
        queue.next_is(LogicOrdinal::Equals) &&
        LogicExpr::raw_check(queue)
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self> {
        todo!()
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> LsonType {
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
        check_expr::<LogicAssignmentStatement>("test = 2", 3);
        check_expr::<LogicAssignmentStatement>("test = layer ? true", 5);
        // check_expr("3.14");
        // check_expr("\"Hello World\"");
    }
}