use std::collections::{btree_map::Values, HashMap};

use super::{
    util::ParseCommaSeparatedList, LibrettoCompileError, LibrettoCompileResult, LibrettoParsable,
};
use crate::{
    lexer::{self, LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal, Ordinal},
    logic::lson::{Lson, LsonType},
    parse_ast,
};

//==================================================================================================
//          Lson Parsable
//==================================================================================================

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for Lson {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if queue.next_is(LogicOrdinal::LeftCurlyBracket) {
            if !ParseCommaSeparatedList::<'a, LogicObjectKeyValue, LibrettoLogicToken>::raw_check(
                queue,
            ) {
                return false;
            }
            if !queue.next_is(LogicOrdinal::RightCurlyBracket) {
                return false;
            }
            true
        } else if queue.next_is(LogicOrdinal::LeftBracket) {
            if !ParseCommaSeparatedList::<'a, Lson, LibrettoLogicToken>::raw_check(queue) {
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
            LogicOrdinal::IntLiteral,
            LogicOrdinal::Identifier
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
                    ParseCommaSeparatedList::<'a, LogicObjectKeyValue, LibrettoLogicToken>,
                    queue,
                    errors
                );
                if !queue.pop_and_check_if(LogicOrdinal::RightCurlyBracket) {
                    // Maybe handle error
                }
                let data: HashMap<String, Lson> = pairs
                    .values()
                    .iter()
                    .map(|e| (e.key.clone(), e.value.clone()))
                    .collect();
                Some(Lson::Struct(data))
            } else if token.check_ordinal(LogicOrdinal::LeftBracket) {
                let pairs = parse_ast!(
                    ParseCommaSeparatedList::<'a, Lson, LibrettoLogicToken>,
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

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>) -> LsonType {
        match self {
            Lson::None => errors.push(LibrettoCompileError::NullValueError),
            Lson::Array(values) => {
                for i in values.iter() {
                    i.validate(errors);
                };
            }
            Lson::Struct(pairs) => {
                for value in pairs.values() {
                    value.validate(errors);
                }
            }
            _ => {}
        }

        self.into()
    }
}

//==================================================================================================
//          LogicKeyValue
//==================================================================================================

#[derive(Debug, PartialEq)]
pub struct LogicObjectKeyValue {
    key: String,
    value: Lson,
}

impl LogicObjectKeyValue {
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &Lson {
        &self.value
    }
}

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicObjectKeyValue {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if !queue.next_is(LogicOrdinal::Identifier) {
            return false;
        }
        if !queue.next_is(LogicOrdinal::Colon) {
            return false;
        }
        if !Lson::raw_check(queue) {
            return false;
        }
        true
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, errors: &mut Vec<LibrettoCompileError>) -> Option<Self> {
        queue.reset();
        let ident = queue.pop_if_next_is(LogicOrdinal::Identifier).unwrap();
        queue.pop_if_next_is(LogicOrdinal::Colon);
        let value = parse_ast!(Lson, queue, errors);

        if let LibrettoLogicToken::Identifier(key) = ident {
            Some(LogicObjectKeyValue { key, value })
        } else {
            None
        }
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>) -> LsonType {
        self.value.validate(errors)
    }
}

//==================================================================================================
//          Tests
//==================================================================================================

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::{
        lexer::{LibrettoLogicToken, LibrettoTokenQueue},
        logic::lson::{Lson, LsonType},
        parse::{logic_value::LogicObjectKeyValue, LibrettoParsable, test_util::*},
    };

    #[test]
    fn check_key_value_pairs() {
        check_expr::<LogicObjectKeyValue>("key : \"value\"", 3);
        check_expr::<LogicObjectKeyValue>("key : false", 3);
        check_expr::<LogicObjectKeyValue>("key : 3.14", 3);
        check_expr::<LogicObjectKeyValue>("key : 3", 3);
    }

    #[test]
    fn parse_key_value_pairs() {
        let ast = parse_expr::<LogicObjectKeyValue>("key : \"value\"");
        assert_eq!(ast.key, "key");
        assert_eq!(ast.value, Lson::String("value".to_string()))
    }

    #[test]
    fn validate_key_value_pairs() {
        validate_expr::<LogicObjectKeyValue>("key : \"value\"", 0, LsonType::String);
        validate_expr::<LogicObjectKeyValue>("key : false", 0, LsonType::Bool);
        validate_expr::<LogicObjectKeyValue>("key : 3.14", 0, LsonType::Float);
        validate_expr::<LogicObjectKeyValue>("key : 2", 0, LsonType::Int);
    }

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
        println!("{:?}", ast);

        let ast = parse_expr::<Lson>("[true, false]");
        println!("{:?}", ast)
    }
}
