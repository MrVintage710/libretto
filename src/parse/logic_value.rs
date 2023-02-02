use crate::{logic::lson::Lson, lexer::LibrettoLogicToken};

use super::LibrettoParsable;

#[derive(Debug)]
pub enum LogicValue {
  Literal(Lson),
  Variable(String)
}

impl From<Lson> for LogicValue {
    fn from(value: Lson) -> Self {
        LogicValue::Literal(value)
    }
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicValue {
    fn parse(lexer : &mut logos::Lexer<'a, LibrettoLogicToken>) -> Option<Self> {
        if let Some(token) = lexer.peekable().peek() {
          match token {
              LibrettoLogicToken::BoolLiteral(value) => return Some(Lson::Bool(*value).into()),
              LibrettoLogicToken::StringLiteral(value) => return Some(Lson::String(value.clone()).into()),
              LibrettoLogicToken::IntLiteral(value) => return Some(Lson::Int(*value).into()),
              LibrettoLogicToken::FloatLiteral(value) => return Some(Lson::Float(*value).into()),
              LibrettoLogicToken::Identifier(value) => return Some(LogicValue::Variable(value.clone())),
              _ => {}
          };
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::{lexer::LibrettoLogicToken, parse::LibrettoParsable};

    use super::LogicValue;


  #[test]
  fn parse_logic_literal() {
    let mut lexer = LibrettoLogicToken::lexer("x");
    let ast = LogicValue::parse(&mut lexer).unwrap();
    println!("{:?}", ast)
  }
}