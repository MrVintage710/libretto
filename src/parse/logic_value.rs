use std::collections::HashMap;

use super::{
    util::{CommaSeparatedList, KeyValuePair}, LibrettoParsable, logic_equality_expr::LogicEqualityExpr,
};
use crate::{
    lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal, Ordinal},
    lson::{Lson, LsonType},
    parse_ast, runtime::{LibrettoRuntime, LibrettoEvaluator, LibrettoRuntimeResult}, compiler::LibrettoCompiletime,
};

#[derive(Debug, PartialEq)]
pub enum LogicValue {
    Literal(Lson),
    Variable(String),
}
impl From<Lson> for LogicValue {
    fn from(value: Lson) -> Self {
        LogicValue::Literal(value)
    }
}

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicValue {

    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        queue.next_is(LogicOrdinal::Identifier) || Lson::raw_check(queue)
    }
    
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
        if queue.next_is(LogicOrdinal::Identifier) {
            if let Some(LibrettoLogicToken::Identifier(value)) = queue.pop() {
                Some(Self::Variable(value))
            } else {
                None
            }
        } else {
            if let Some(lson) = Lson::parse(queue, compile_time) {
                Some(Self::Literal(lson))
            } else {
                None
            }
        }
    }

    fn validate(&self, compile_time : &mut LibrettoCompiletime) -> LsonType {
        match self {
            LogicValue::Literal(lson) => lson.validate(compile_time),
            LogicValue::Variable(value) => {
                return compile_time.get_data(value);
            },
        }
    }
}

impl LibrettoEvaluator for LogicValue {
    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> LibrettoRuntimeResult {
        match self {
            LogicValue::Literal(value) => Ok(value.clone()),
            LogicValue::Variable(ident) => Ok(runtime.get_data(ident).clone()),
        }
    }
}

//==================================================================================================
//          Lson Parsable
//==================================================================================================

type ObjectTerm<'a> = CommaSeparatedList<'a, KeyValuePair<'a, Lson, LibrettoLogicToken>, LibrettoLogicToken>;
type FunctionParams<'a> = CommaSeparatedList<'a, KeyValuePair<'a, LsonType, LibrettoLogicToken>, LibrettoLogicToken>;
type ArrayTerm<'a> = CommaSeparatedList<'a, Lson, LibrettoLogicToken>;

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for Lson {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if queue.next_is(LogicOrdinal::LeftCurlyBracket) {
            ObjectTerm::<'a>::raw_check(queue) &&
            queue.next_is(LogicOrdinal::RightCurlyBracket)
        } else if queue.next_is(LogicOrdinal::LeftBracket) {
            ArrayTerm::<'a>::raw_check(queue) &&
            queue.next_is(LogicOrdinal::RightBracket)
        } else if queue.next_is(LogicOrdinal::LeftParen) {
            LogicEqualityExpr::raw_check(queue) && queue.next_is(LogicOrdinal::RightParen)
//            if ArrayTerm::<'a>::raw_check(queue) &&
//            queue.next_is(LogicOrdinal::RightParen){
//                queue.next_is(LogicOrdinal::Arrow);
//                queue.next_is(LogicOrdinal::Type);
//                return true;
//            }
        } else if queue.next_is([
            LogicOrdinal::StringLiteral,
            LogicOrdinal::BoolLiteral,
            LogicOrdinal::FloatLiteral,
            LogicOrdinal::IntLiteral,
            LogicOrdinal::NoneLiteral
        ]) {
            true
        } else {
            false
        }
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
        if let Some(token) = queue.pop() {
            if token.check_ordinal(LogicOrdinal::LeftCurlyBracket) {
                let pairs = parse_ast!(
                    ObjectTerm::<'a>,
                    queue,
                    compile_time
                );
                if !queue.pop_and_check_if(LogicOrdinal::RightCurlyBracket) {
                    // Maybe handle error
                }
                let data: HashMap<String, Lson> = pairs
                    .values()
                    .iter()
                    .map(|e| (e.key().to_string(), e.value().clone()))
                    .collect();
                Some(Lson::Struct(data))
            } else if token.check_ordinal(LogicOrdinal::LeftBracket) {
                let pairs = parse_ast!(
                    ArrayTerm::<'a>,
                    queue,
                    compile_time
                );
                if !queue.pop_and_check_if(LogicOrdinal::RightBracket) {
                    //handle
                }
                let data: Vec<Lson> = pairs.values().iter().map(|e| e.clone()).collect();
                Some(Lson::Array(data))
            } else {
                match token {
                    LibrettoLogicToken::StringLiteral(value) => {
                        Some(Lson::String(value))
                    }
                    LibrettoLogicToken::BoolLiteral(value) => {
                        Some(Lson::Bool(value))
                    }
                    LibrettoLogicToken::FloatLiteral(value) => {
                        Some(Lson::Float(value))
                    }
                    LibrettoLogicToken::IntLiteral(value) => {
                        Some(Lson::Int(value))
                    }
                    LibrettoLogicToken::NoneLiteral => {
                        Some(Lson::None)
                    }
                    _ => None,
                }
            }
        } else {
            None
        }
    }

    fn validate(&self, compile_time : &mut LibrettoCompiletime) -> LsonType {
        match self {
            Lson::Array(values) => {
                for i in values.iter() {
                    i.validate(compile_time);
                };
            }
            Lson::Struct(pairs) => {
                for value in pairs.values() {
                    value.validate(compile_time);
                }
            }
            _ => {}
        }

        self.into()
    }
}

//==================================================================================================
//          Lson Parsable
//==================================================================================================

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LsonType {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        queue.next_is([LogicOrdinal::Type])
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
        let token = queue.pop();
        if let Some(LibrettoLogicToken::Type(t)) = token {
            return Some(t)
        }

        None
    }

    fn validate(&self, compile_time : &mut LibrettoCompiletime) -> LsonType {
        *self
    }
}

//==================================================================================================
//          Tests
//==================================================================================================

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        lson::{Lson, LsonType},
        parse::test_util::*,
    };

    use super::*;

    #[test]
    fn check_lson() {
        check_expr::<Lson>("3", 1);
        check_expr::<Lson>("none", 1);
        check_expr::<Lson>("[true, false]", 5);
        check_expr::<Lson>("{ key : false, test : false }", 9);
    }

    #[test]
    fn parse_lson() {
        let ast = parse_expr::<Lson>("none");
        assert_eq!(ast, Lson::None);

        let ast = parse_expr::<Lson>("{obj : {key : \"value\"}}");
        assert_eq!(ast, Lson::Struct(HashMap::from([("obj".to_string(), Lson::Struct(HashMap::from([("key".to_string(), "value".into())])))])));

        let ast = parse_expr::<Lson>("[true, false]");
        assert_eq!(ast, Lson::Array(vec![Lson::Bool(true), Lson::Bool(false)]));
    }

    #[test]
    fn validate_lson() {
        validate_expr::<Lson>("3", 0, LsonType::Int);
        validate_expr::<Lson>("none", 0, LsonType::None);
        validate_expr::<Lson>("[true, false]", 0, LsonType::Array);
        // check_expr("3.14");
        // check_expr("\"Hello World\"");
    }

    #[test]
    fn check_logic_value() {
        check_expr::<LogicValue>("3", 1);
        check_expr::<LogicValue>("[true, false]", 5);
        check_expr::<LogicValue>("{ key : false, test : false }", 9);
        check_expr::<LogicValue>("test", 1);
        check_expr::<LogicValue>("(2+2)", 5);

        // check_expr("3.14");
        // check_expr("\"Hello World\"");
    }

    #[test]
    fn parse_logic_value() {
        let ast = parse_expr::<LogicValue>("{obj : {key : \"value\"}}");
        println!("{:?}", ast);

        let ast = parse_expr::<Lson>("[true, false]");
        println!("{:?}", ast)
    }

    #[test]
    fn validate_logic_value() {
        validate_expr::<LogicValue>("3", 0, LsonType::Int);
        validate_expr::<LogicValue>("[true, false]", 0, LsonType::Array);
        validate_expr::<LogicValue>("foo", 0, LsonType::Float);
        validate_expr::<LogicValue>("test", 0, LsonType::None);
    }
}
