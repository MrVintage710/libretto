use crate::lexer::{LibrettoLogicToken, LogicOrdinal};
use super::{LibrettoParsable, logic_let_stmt::LogicLetStatement, logic_assignment_stmt::LogicAssignmentStatement, logic_expr::LogicExpr};

pub enum LogicStmt {
    ExprStmt(LogicExpr),
    LetStmt(LogicLetStatement),
    AssignmentStmt(LogicAssignmentStatement)
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicStmt {
    fn raw_check(queue: &mut crate::lexer::LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        LogicExpr::raw_check(queue) && queue.next_is(LogicOrdinal::Semicolon) ||
        LogicAssignmentStatement::raw_check(queue) ||
        LogicLetStatement::raw_check(queue)
    }

    fn parse(queue: &mut crate::lexer::LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut crate::compiler::LibrettoCompiletime) -> Option<Self> {
//        if LogicExpr::raw_check(queue) && queue{
//            let
//        }

        None
    }

    fn validate(&self, compile_time : &mut crate::compiler::LibrettoCompiletime) -> crate::lson::LsonType {
        todo!()
    }
}