use logos::{Logos, Lexer};

//==================================================================================================
//          Libretto Token - Top Level Lexing
//==================================================================================================

fn content_after_first<'a>(lex : &mut Lexer<'a, LibrettoToken<'a>>) -> String {
  lex.slice()[1..].to_string()
}

fn content_except_first_and_last<'a>(lex : &mut Lexer<'a, LibrettoToken<'a>>) -> String {
  let content = lex.slice();
  content[1..content.len()-1].to_string()
}

fn as_logic_for_top<'a>(lex : &mut Lexer<'a, LibrettoToken<'a>>) -> Lexer<'a, LibrettoLogicToken> {
  let content = lex.slice();
  let content = &content[1..content.len()-1];
  let logic_lex = LibrettoLogicToken::lexer(content);
  logic_lex
}

#[derive(Debug, Logos)]
pub enum LibrettoToken<'a> {

  #[regex("#([^ \t\n]*)", content_after_first)]
  Tag(String),
  
  #[regex(":([^ \t\n]*)", content_after_first)]
  Speaker(String),

  #[regex("<([^><]*)>", as_logic_for_top)]
  Logic(Lexer<'a, LibrettoLogicToken>),

  #[regex("\"([^\"]*)\"", content_except_first_and_last)]
  Quote(String),

  #[token("|")]
  Bar,

  #[token("{")]
  LeftCurlyBracket,

  #[token("}")]
  RightCurlyBracket,

  #[token("->")]
  Arrow,

  #[token("--")]
  Dash,

  #[token("request")]
  Request,

  #[regex(r"//[^\n\r]+(?:\*\)|[\n\r])", logos::skip)]
  Comment,

  #[regex(r"[ \t\n\f]+", logos::skip)]
  Whitespace,

  #[error]
  Error
}

//==================================================================================================
//          Libretto Logic Token - Logic Level Lexing
//==================================================================================================

fn lex_string(lex : &mut Lexer<LibrettoLogicToken>) -> String {
  let content = lex.slice();
  content[1..content.len()-1].to_string()
}

fn lex_int(lex : &mut Lexer<LibrettoLogicToken>) -> i64 {
  let content = lex.slice().to_string();
  content.parse::<i64>().unwrap()
}

fn lex_float(lex : &mut Lexer<LibrettoLogicToken>) -> f64 {
  let content = lex.slice().to_string();
  content.parse::<f64>().unwrap()
}

fn lex_bool(lex : &mut Lexer<LibrettoLogicToken>) -> bool {
  let content = lex.slice().to_string();
  content.as_str() == "true"
}

#[derive(Debug, Logos, PartialEq)]
pub enum LibrettoLogicToken {

  #[regex("[a-zA-Z0-9_]+", priority=1)]
  Text,

  #[regex("[0-9]+", lex_int, priority=2)]
  IntLiteral(i64),

  #[regex("[0-9]+.[0-9]+", lex_float, priority=3)]
  FloatLiteral(f64),

  #[regex("(true|false)", lex_bool)]
  BoolLiteral(bool),

  #[regex("\"([^\"]*)\"", lex_string)]
  StringLiteral(String),

  #[token("function")]
  Function,

  #[token("if")]
  If,
  
  #[token("else")]
  Else,

  #[token("for")]
  For,

  #[token("in")]
  In,

  #[token("let")]
  Let,

  #[token("const")]
  Const,

  #[token("int")]
  Int,

  #[token("float")]
  Float,

  #[token("string")]
  String,

  #[token("bool")]
  Bool,

  #[token("{")]
  LeftCurlyBracket,

  #[token("}")]
  RightCurlyBracket,

  #[token("[")]
  LeftBracket,

  #[token("]")]
  RightBracket,

  #[token("(")]
  LeftParen,

  #[token(")")]
  RightParen,

  #[token(".")]
  Period,

  #[token(",")]
  Comma,

  #[token(":")]
  Colon,

  #[token("==", priority=2)]
  Equality,

  #[token("<=", priority=2)]
  LessThanEquality,

  #[token(">=", priority=2)]
  GreaterThanEquality,

  #[token("<")]
  LessThan,

  #[token(">")]
  GreaterThan,

  #[token("+")]
  Add,

  #[token("-")]
  Sub,

  #[token("*")]
  Mult,

  #[token("/")]
  Div,

  #[token("=")]
  Equals,

  #[regex(r"[ \t\n\f]+", logos::skip)]
  Whitespace,

  #[error]
  Error
}

//==================================================================================================
//          Libretto Quote Token - Quote Level Lexing
//==================================================================================================

fn as_logic_for_quote<'a>(lex : &mut Lexer<'a, LibrettoQuoteToken>) -> Vec<LibrettoLogicToken> {
  let content = lex.slice();
  let content = &content[1..content.len()-1];
  let logic_lex = LibrettoLogicToken::lexer(content);
  let tokens = logic_lex.collect::<Vec<LibrettoLogicToken>>();
  tokens
}

fn start_tag<'a>(lex: &mut Lexer<'a, LibrettoQuoteToken>) -> String {
  let content = lex.slice();
  let content = &content[1..content.len()-1];
  content.to_string()
}

fn end_tag<'a>(lex: &mut Lexer<'a, LibrettoQuoteToken>) -> String {
  let content = lex.slice();
  let content = &content[2..content.len()-1];
  content.to_string()
}

#[derive(Debug, Logos, PartialEq)]
pub enum LibrettoQuoteToken {

  #[regex(r"\[[a-zA-Z]*\]", start_tag, priority=2)] 
  StartTag(String),

  #[regex(r"\[/[a-zA-Z]*\]", end_tag, priority=1)]
  EndTag(String), 

  //select any character and space combination ending with either a period, colon, exclamation mark, or question mark.
  #[regex(r"[\w\s(.|!|?|:|;)]+", priority=2)]
  Text,

  #[regex(r"<([^><]*)>", as_logic_for_quote)]
  Logic(Vec::<LibrettoLogicToken>),

  #[error]
  Error
}

#[cfg(test)]
mod tests {
  use crate::LibrettoQuoteToken;
  use crate::LibrettoLogicToken;
  use logos::Logos;

  #[test]
  fn quote_text_test() {
    let mut lex = LibrettoQuoteToken::lexer("Go away Brigand!");
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::Text));
    assert_eq!(lex.slice(), "Go away Brigand!");
    assert_eq!(lex.next(), None);
  }

  #[test]
  fn quote_tag_test() {
    let mut lex = LibrettoQuoteToken::lexer("[Welcoming]Hello World![/Welcoming]");
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::StartTag("Welcoming".to_string())));
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::Text));
    assert_eq!(lex.slice(), "Hello World!");
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::EndTag("Welcoming".to_string())));
    assert_eq!(lex.next(), None);
  }
  

  #[test]
  fn quote_logic_test() {
    let mut lex = LibrettoQuoteToken::lexer("My logic is: <if status.guild_member == False> end.");
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::Text));
    assert_eq!(lex.slice(), "My logic is: ");
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::Logic(vec![
      LibrettoLogicToken::If,
      LibrettoLogicToken::Text("status".to_string()),
      LibrettoLogicToken::Period,
      LibrettoLogicToken::Text("guild_member".to_string()),
      LibrettoLogicToken::Equality,
      LibrettoLogicToken::Text("False".to_string())
    ])));
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::Text));
    assert_eq!(lex.slice(), " end.");
    assert_eq!(lex.next(), None);
  }

  #[test]
  fn quote_complex_test() {
    let mut lex = LibrettoQuoteToken::lexer("[yelling]Go away Brigand![/yelling]None named <player.name> are welcome here.");
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::StartTag("yelling".to_string())));
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::Text));
    assert_eq!(lex.slice(), "Go away Brigand!");
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::EndTag("yelling".to_string())));
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::Text));
    assert_eq!(lex.slice(), "None named ");
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::Logic(vec![LibrettoLogicToken::Text("player".to_string()), LibrettoLogicToken::Period, LibrettoLogicToken::Text("name".to_string())])));
    assert_eq!(lex.next(), Some(LibrettoQuoteToken::Text));
    assert_eq!(lex.slice(), " are welcome here.");
    assert_eq!(lex.next(), None);
  }
}
