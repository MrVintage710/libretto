use std::collections::btree_map::Range;

use crate::{logic::lson::Lson, lexer::{LibrettoLogicToken, LibrettoTokenQueue}};

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
    fn parse(queue : &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> Option<Self> {
      if queue.next_is(token)
      
      let lex = lexer.peek();
      if lex.is_none() { return None }
      let (token, _) = lexer.next().unwrap();

      match token {
        LibrettoLogicToken::BoolLiteral(value) => return Some(Lson::Bool(value).into()),
        LibrettoLogicToken::StringLiteral(value) => return Some(Lson::String(value.clone()).into()),
        LibrettoLogicToken::IntLiteral(value) => return Some(Lson::Int(value).into()),
        LibrettoLogicToken::FloatLiteral(value) => return Some(Lson::Float(value).into()),
        LibrettoLogicToken::Identifier(value) => return Some(LogicValue::Variable(value.clone())),
        _ => None
      }
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::{lexer::LibrettoLogicToken, parse::LibrettoParsable};

    use super::LogicValue;


  #[test]
  fn parse_logic_literal() {
    let mut lexer = LibrettoLogicToken::lexer("x").spanned().peekable();
    let ast = LogicValue::parse(&mut lexer).unwrap();
    println!("{:?}", ast)
  }
}