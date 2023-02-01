use crate::{logic::lson::Lson, lexer::{LibrettoLogicToken, LibrettoTokenQueue}};

use super::LibrettoParsable;

#[derive(Debug, PartialEq)]
pub struct LogicLiteral {
  value : Lson
}

impl From<Lson> for LogicLiteral {
    fn from(value: Lson) -> Self {
        LogicLiteral { value }
    }
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicLiteral {
    fn parse(lexer : &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> Option<Self> {
        if let Some((token, range)) = lexer.peek() {
          match token {
              LibrettoLogicToken::BoolLiteral(value) => return Some(Lson::Bool(*value).into()),
              LibrettoLogicToken::StringLiteral(value) => return Some(Lson::String(value.clone()).into()),
              LibrettoLogicToken::IntLiteral(value) => return Some(Lson::Int(*value).into()),
              LibrettoLogicToken::FloatLiteral(value) => return Some(Lson::Float(*value).into()),
              _ => {}
          };
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use logos::Logos;

    use crate::{lexer::LibrettoLogicToken, parse::LibrettoParsable, logic::lson::Lson};

    use super::LogicLiteral;


  #[test]
  fn parse_logic_literal() {
    let mut lexer = LibrettoLogicToken::lexer("false").spanned().peekable();
    let ast = LogicLiteral::parse(&mut lexer).unwrap();
    assert_eq!(LogicLiteral{value : Lson::Bool(false)}, ast)
  }
}