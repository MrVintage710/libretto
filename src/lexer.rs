use logos::{Logos, Lexer};

//==================================================================================================
//          Libretto Token - Top Level Lexing
//==================================================================================================

fn content_after_first(lex : &mut Lexer<LibrettoToken>) -> String {
  lex.slice()[1..].to_string()
}

fn content_except_first_and_last(lex : &mut Lexer<LibrettoToken>) -> String {
  let content = lex.slice();
  content[1..content.len()-1].to_string()
}

#[derive(Debug, Logos)]
pub enum LibrettoToken {

  #[regex("#([^ \t\n]*)", content_after_first)]
  Tag(String),
  
  #[regex(":([^ \t\n]*)", content_after_first)]
  Speaker(String),

  #[regex("<([^><]*)>", content_except_first_and_last)]
  Logic(String),

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

fn lex_text(lex : &mut Lexer<LibrettoLogicToken>) -> String {
  lex.slice().to_string()
}

fn lex_int(lex : &mut Lexer<LibrettoLogicToken>) -> i64 {
  let content = lex.slice().to_string();
  content.parse::<i64>().unwrap()
}

fn lex_float(lex : &mut Lexer<LibrettoLogicToken>) -> f64 {
  let content = lex.slice().to_string();
  content.parse::<f64>().unwrap()
}

#[derive(Debug, Logos)]
pub enum LibrettoLogicToken {

  #[regex("[a-zA-Z0-9_]+", lex_text, priority=1)]
  Text(String),

  #[regex("[0-9]+", lex_int, priority=2)]
  IntLiteral(i64),

  #[regex("[0-9]+.[0-9]+", lex_float, priority=3)]
  FloatLiteral(f64),

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