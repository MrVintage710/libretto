use std::collections::HashMap;

use super::{
    util::{CommaSeparatedList, KeyValuePair}, LibrettoCompileError, LibrettoParsable, LibrettoEvaluator,
};
use crate::{
    lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal, Ordinal},
    lson::{Lson, LsonType},
    parse_ast, runtime::LibrettoRuntime,
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
    
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self> {
        if queue.next_is(LogicOrdinal::Identifier) {
            if let Some(LibrettoLogicToken::Identifier(value)) = queue.pop() {
                Some(Self::Variable(value))
            } else {
                None
            }
        } else {
            if let Some(lson) = Lson::parse(queue, errors) {
                Some(Self::Literal(lson))
            } else {
                None
            }
        }
    }

    fn validate(&self, errors : &mut Vec<LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> LsonType {
        match self {
            LogicValue::Literal(lson) => lson.validate(errors, type_map),
            LogicValue::Variable(value) => {
                if type_map.contains_key(value) {
                    return type_map.get(value).unwrap().clone();
                } 
                errors.push(LibrettoCompileError::NullValueError);
                LsonType::None
            },
        }
    }
}

impl LibrettoEvaluator for LogicValue {
    fn evaluate(&self, runtime: &mut LibrettoRuntime) -> Lson {
        match self {
            LogicValue::Literal(lson) => {
                return lson.clone();
            },
            LogicValue::Variable(ident) => {
                return Lson::None;
            },
        }
    }
}

//==================================================================================================
//          Lson Parsable
//==================================================================================================

type ObjectTerm<'a> = CommaSeparatedList<'a, KeyValuePair<'a, Lson, LibrettoLogicToken>, LibrettoLogicToken>;
type ArrayTerm<'a> = CommaSeparatedList<'a, Lson, LibrettoLogicToken>;

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for Lson {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if queue.next_is(LogicOrdinal::LeftCurlyBracket) {
            if !ObjectTerm::<'a>::raw_check(
                queue,
            ) {
                return false;
            }
            if !queue.next_is(LogicOrdinal::RightCurlyBracket) {
                return false;
            }
            true
        } else if queue.next_is(LogicOrdinal::LeftBracket) {
            if !ArrayTerm::<'a>::raw_check(queue) {
                return false;
            }
            if !queue.next_is(LogicOrdinal::RightBracket) {
                return false;
            }
            true
        } else if queue.next_is([
            LogicOrdinal::StringLiteral,
            LogicOrdinal::BoolLiteral,
            LogicOrdinal::FloatLiteral,
            LogicOrdinal::IntLiteral
        ]) {
            true
        } else {
            false
        }
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self> {
        if let Some(token) = queue.pop() {
            if token.check_ordinal(LogicOrdinal::LeftCurlyBracket) {
                let pairs = parse_ast!(
                    ObjectTerm::<'a>,
                    queue,
                    errors
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
                    errors
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
                    _ => None,
                }
            }
        } else {
            None
        }
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> LsonType {
        match self {
            Lson::None => errors.push(LibrettoCompileError::NullValueError),
            Lson::Array(values) => {
                for i in values.iter() {
                    i.validate(errors, type_map);
                };
            }
            Lson::Struct(pairs) => {
                for value in pairs.values() {
                    value.validate(errors, type_map);
                }
            }
            _ => {}
        }

        self.into()
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
        check_expr::<Lson>("[true, false]", 5);
        check_expr::<Lson>("{ key : false, test : false }", 9);
        // check_expr("3.14");
        // check_expr("\"Hello World\"");
    }

    #[test]
    fn parse_lson() {
        let ast = parse_expr::<Lson>("{obj : {key : \"value\"}}");
        assert_eq!(ast, Lson::Struct(HashMap::from([("obj".to_string(), Lson::Struct(HashMap::from([("key".to_string(), "value".into())])))])));

        let ast = parse_expr::<Lson>("[true, false]");
        assert_eq!(ast, Lson::Array(vec![Lson::Bool(true), Lson::Bool(false)]));
    }

    #[test]
    fn validate_lson() {
        validate_expr::<Lson>("3", 0, LsonType::Int);
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
        validate_expr::<LogicValue>("test", 1, LsonType::None);
    }
}
