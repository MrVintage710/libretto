use std::collections::{btree_map::Values, HashMap};

use super::{
    util::ParseCommaSeparatedList, LibrettoCompileError, LibrettoCompileResult, LibrettoParsable,
    ParseResult,
};
use crate::{
    lexer::{self, LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal, Ordinal},
    logic::lson::Lson,
    parse_ast,
};

//==================================================================================================
//          Logic Value
//==================================================================================================

/// This Enum is part of the Libretto AST. It represents a value inside of the source code of Libretto. This is any of the following:
///
/// Int
/// ``` 1 ```
///
/// Float
/// ``` 3.1459 ```
///
/// String
/// ``` "Hello World" ```
///
/// Bool
/// ``` true ```
///
/// Arrays
/// ``` [1, 2, 3, 4] ```
///
/// Struct
/// ``` {foo : "bar"} ```
///
/// Functions (WIP)
/// ``` () : string { return "Hello World!" } ```
///
/// Evaluates to Lson obj containing these values
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
        //Check if the next token is one of the following:
        //String Literal, Bool Literal, Float Literal, Int Literal, Identifier
        //If it is, move the queue cursor forward by 1 and return true, else return false
        if Lson::raw_check(queue) {return true;}
        if queue.next_is(LogicOrdinal::Identifier) {return true;}
        false
    }

    ///Parse a LogicValue Object from a Token Queue
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> ParseResult<Self> {
        if let Some(lson) = Lson::checked_parse(queue) {
            return ParseResult::Parsed(LogicValue::Literal(lson.clone()));
        }

        if let Some(token) = queue.pop_if_next_is(LogicOrdinal::Identifier) {
            println!("{:?}", token);
            if let LibrettoLogicToken::Identifier(value) = token {
                return ParseResult::Parsed(LogicValue::Variable(value.clone()));
            }
        }

        ParseResult::Failure
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>) {
        todo!()
    }
}

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
            LogicOrdinal::Identifier,
        ]) {
            true
        } else {
            false
        }
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> ParseResult<Self> {
        if let Some(token) = queue.pop() {
            if token.check_ordinal(LogicOrdinal::LeftCurlyBracket) {
                let pairs = parse_ast!(
                    ParseCommaSeparatedList::<'a, LogicObjectKeyValue, LibrettoLogicToken>,
                    queue
                );
                if !queue.pop_and_check_if(LogicOrdinal::RightCurlyBracket) {
                    return ParseResult::Error("Could not find end of object.".to_string());
                }
                let data: HashMap<String, Lson> = pairs
                    .values()
                    .iter()
                    .map(|e| (e.key.clone(), e.value.clone()))
                    .collect();
                ParseResult::Parsed(Lson::Struct(data))
            } else if token.check_ordinal(LogicOrdinal::LeftBracket) {
                let pairs = parse_ast!(
                    ParseCommaSeparatedList::<'a, Lson, LibrettoLogicToken>,
                    queue
                );
                if !queue.pop_and_check_if(LogicOrdinal::RightBracket) {
                    return ParseResult::Error("Could not find end of object.".to_string());
                }
                let data: Vec<Lson> = pairs.values().iter().map(|e| e.clone()).collect();
                ParseResult::Parsed(Lson::Array(data))
            } else {
                match token {
                    LibrettoLogicToken::StringLiteral(value) => {
                        ParseResult::Parsed(Lson::String(value).into())
                    }
                    LibrettoLogicToken::BoolLiteral(value) => {
                        ParseResult::Parsed(Lson::Bool(value).into())
                    }
                    LibrettoLogicToken::FloatLiteral(value) => {
                        ParseResult::Parsed(Lson::Float(value).into())
                    }
                    LibrettoLogicToken::IntLiteral(value) => {
                        ParseResult::Parsed(Lson::Int(value).into())
                    }
                    _ => ParseResult::Failure,
                }
            }
        } else {
            ParseResult::Failure
        }
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>) {
        match self {
            Lson::None => errors.push(LibrettoCompileError::NullValueError),
            Lson::Array(values) => {
                for i in values.iter() {
                    i.validate(errors)
                }
            }
            Lson::Struct(pairs) => {
                for value in pairs.values() {
                    value.validate(errors)
                }
            }
            _ => {}
        }
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

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> ParseResult<Self> {
        queue.reset();
        let ident = queue.pop_if_next_is(LogicOrdinal::Identifier).unwrap();
        queue.pop_if_next_is(LogicOrdinal::Colon);
        let value = Lson::parse(queue).unwrap();

        if let LibrettoLogicToken::Identifier(key) = ident {
            ParseResult::Parsed(LogicObjectKeyValue { key, value })
        } else {
            ParseResult::Failure
        }
    }

    fn validate(&self, errors: &mut Vec<LibrettoCompileError>) {
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
        logic::lson::Lson,
        parse::{logic_value::LogicObjectKeyValue, LibrettoParsable},
    };

    use super::LogicValue;

    fn check_expr<'a, T: LibrettoParsable<'a, LibrettoLogicToken>>(
        source: &'a str,
        number_of_tokens: usize,
    ) {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let check = T::check(&mut queue);
        assert!(check);
        assert_eq!(queue.cursor(), 0);
        queue.reset();
        let check = T::raw_check(&mut queue);
        assert!(check);
        assert_eq!(queue.cursor(), number_of_tokens)
    }

    fn parse_expr<'a, T: LibrettoParsable<'a, LibrettoLogicToken>>(source: &'a str) -> T {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        println!("{:?}", queue);
        let result = T::checked_parse(&mut queue);
        assert!(result.is_some());
        result.unwrap()
    }

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

    #[test]
    fn check_logic_value() {
        check_expr::<LogicValue>("3", 1);
        check_expr::<LogicValue>("test", 1);
    }

    #[test]
    fn parse_logic_value() {
        let ast = parse_expr::<LogicValue>("ident ");
        println!("{:?}", ast);
        if let LogicValue::Variable(value) = ast {
            assert_eq!(value, "test".to_string());
        } else {
            assert!(false)
        }
        // check_expr("3.14");
        // check_expr("\"Hello World\"");
    }

    #[test]
    fn validate_logic_value() {}
}
