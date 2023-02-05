use super::{LibrettoParsable, ParseResult};
use crate::{
    lexer::{self, LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal},
    logic::lson::Lson,
};

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
#[derive(Debug)]
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
    
    ///Parse a LogicValue Object from a Token Queue
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> ParseResult<Self> {

        //First we check if the next token is one of the following:
        //String Literal, Bool Literal, Float Literal, Int Literal, Identifier
        //If is isn't any of those, return a failure.
        if !queue.next_is([
            LogicOrdinal::StringLiteral,
            LogicOrdinal::BoolLiteral,
            LogicOrdinal::FloatLiteral,
            LogicOrdinal::IntLiteral,
            LogicOrdinal::Identifier])   
        {
            return ParseResult::Failure;
        }

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
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::{
        lexer::{LibrettoLogicToken, LibrettoTokenQueue},
        parse::LibrettoParsable,
    };

    use super::LogicValue;

    #[test]
    fn parse_logic_literal() {
        let mut lex = LibrettoLogicToken::lexer("true");
        let mut queue = LibrettoTokenQueue::from(lex);
        let ast = LogicValue::parse(&mut queue).unwrap();
        println!("{:?}", ast)
    }
}
