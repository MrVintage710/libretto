use super::{LibrettoParsable, ParseResult, LibrettoCompileError, LibrettoCompileResult};
use crate::{
    lexer::{self, LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal},
    logic::lson::Lson,
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
/// Arrays (WIP)
/// ``` [1, 2, 3, 4] ```
/// 
/// Struct (WIP)
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
        if queue.next_is([
            LogicOrdinal::StringLiteral,
            LogicOrdinal::BoolLiteral,
            LogicOrdinal::FloatLiteral,
            LogicOrdinal::IntLiteral,
            LogicOrdinal::Identifier])
        {
            true
        } else {
            false
        }
    }
    
    ///Parse a LogicValue Object from a Token Queue
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> ParseResult<Self> {
        
        //Now we pop the token, and return a Parsed(LogicValue) with the token type inside
        //Later on this method will also hold values for objects and arrays, however 
        //parsing for those structures has not been implemented yet.
        if let Some(token) = queue.pop() {
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
                LibrettoLogicToken::Identifier(value) => {
                    ParseResult::Parsed(LogicValue::Variable(value))
                }
                _ => ParseResult::Failure,
            }
        } else {
            ParseResult::Failure
        }
    }

    fn validate(&self, errors : &mut Vec<LibrettoCompileError>) {
        todo!()
    }
}

//==================================================================================================
//          Lson Parsable
//==================================================================================================

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for Lson {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if queue.next_is(LogicOrdinal::LeftCurlyBracket) {


            false
        } else if queue.next_is([
            LogicOrdinal::StringLiteral,
            LogicOrdinal::BoolLiteral,
            LogicOrdinal::FloatLiteral,
            LogicOrdinal::IntLiteral,
            LogicOrdinal::Identifier])
        {
            true
        } else {
            false
        }
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> ParseResult<Self> {
        if let Some(token) = queue.pop() {
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
        } else {
            ParseResult::Failure
        }
    }

    fn validate(&self, errors : &mut Vec<LibrettoCompileError>) {
        todo!()
    }
}

//==================================================================================================
//          LogicKeyValue
//==================================================================================================

#[derive(Debug, PartialEq)]
pub struct LogicObjectKeyValue {
    key : String,
    value : Lson
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicObjectKeyValue {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if !queue.next_is(LogicOrdinal::Identifier) {return false}
        if !queue.next_is(LogicOrdinal::Colon) {return false}
        if !Lson::raw_check(queue) {return false;}
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

    fn validate(&self, errors : &mut Vec<LibrettoCompileError>) {
        todo!()
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
        parse::{LibrettoParsable, logic_value::LogicObjectKeyValue}, logic::lson::Lson,
    };

    use super::LogicValue;

    fn check_expr<'a, T : LibrettoParsable<'a, LibrettoLogicToken>>(source : &str, number_of_tokens : usize) {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let check = LogicObjectKeyValue::check(&mut queue);
        assert!(check);
        assert_eq!(queue.cursor(), 0);
        queue.reset();
        let check = LogicObjectKeyValue::raw_check(&mut queue);
        assert!(check);
        assert_eq!(queue.cursor(), number_of_tokens)
    }

    fn check_expr_inv(source : &str) {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let check = LogicValue::raw_check(&mut queue);
        assert!(!check);
    }

    fn validate_expr(source : &str) {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
    }

    #[test]
    fn check_key_value_pairs() {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer("key : \"value\""));
        let key_value = LogicObjectKeyValue::checked_parse(&mut queue);
        if let Some(value) = key_value {
            assert_eq!(value, LogicObjectKeyValue{ key: "key".to_owned(), value : Lson::String("value".to_owned())})
        } else {
            assert!(false)
        }
    }

    #[test]
    fn parse_key_value_pairs() {
        check_expr::<LogicObjectKeyValue>("key : \"value\"", 3);
        check_expr::<LogicObjectKeyValue>("key : false", 3);
        check_expr::<LogicObjectKeyValue>("key : 3.14", 3);
        check_expr::<LogicObjectKeyValue>("key : 3", 3);
    }

    #[test]
    fn check_logic_value() {
        // check_expr("true");
        // check_expr("3");
        // check_expr("3.14");
        // check_expr("\"Hello World\"");
    }

    #[test]
    fn parse_logic_value() {
        let mut lex = LibrettoLogicToken::lexer("true");
        let mut queue = LibrettoTokenQueue::from(lex);
        let ast = LogicValue::checked_parse(&mut queue).unwrap();
        println!("{:?}", ast)
    }

    #[test]
    fn validate_logic_value() {
        
    }
}
