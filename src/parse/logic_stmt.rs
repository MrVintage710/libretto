use crate::{lexer::{LibrettoLogicToken, LogicOrdinal, LibrettoTokenQueue}, compiler::LibrettoCompiletime};
use super::{LibrettoParsable, logic_let_stmt::LogicLetStatement, logic_assignment_stmt::LogicAssignmentStatement, logic_expr::LogicExpr};

pub enum LogicStmt {
    ExprStmt(LogicExpr),
    LetStmt(LogicLetStatement),
    AssignmentStmt(LogicAssignmentStatement)
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicStmt {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        LogicExpr::raw_check(queue) && queue.next_is(LogicOrdinal::Semicolon) ||
        LogicAssignmentStatement::raw_check(queue) ||
        LogicLetStatement::raw_check(queue)
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
        
        if LogicExpr::

        None
    }

    fn validate(&self, compile_time : &mut LibrettoCompiletime) -> crate::lson::LsonType {
        todo!()
    }
}