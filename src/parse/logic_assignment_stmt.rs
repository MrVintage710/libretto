use std::collections::HashMap;

use crate::{lexer::{LibrettoTokenQueue, LibrettoLogicToken, LogicOrdinal}, lson::{LsonType, Lson}, parse_ast, compiler::{LibrettoCompileError, LibrettoCompiletime}, runtime::{LibrettoRuntime, LibrettoEvaluator, LibrettoRuntimeResult}};

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
        let desired_type = compile_time.get_variable_type(&self.ident);
        let value_type = self.value.validate(compile_time);

        if desired_type == LsonType::None {
            compile_time.push_error(LibrettoCompileError::AssignmentWithUndeclaredVariable(self.ident.clone()))
        }

        if desired_type != value_type {
            compile_time.push_error(LibrettoCompileError::AssignmentWithInvalidType(self.ident.clone()));
        }

        desired_type
    }
}

impl LibrettoEvaluator for LogicAssignmentStatement {
    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> LibrettoRuntimeResult {
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
    }

    #[test]
    fn parse_assign_stmt() {
        let ast = parse_expr::<LogicAssignmentStatement>("test = 2");
    }

    #[test]
    fn validate_assign_stmt() {
        validate_expr::<LogicAssignmentStatement>("test = 2", 2, LsonType::None);
        validate_expr::<LogicAssignmentStatement>("bar = 2", 1, LsonType::Bool);
        validate_expr::<LogicAssignmentStatement>("foo = 2.0", 0, LsonType::Float);
    }
}