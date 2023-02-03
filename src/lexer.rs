use std::{iter::Peekable, fmt::Debug, ops::Range};
use peekmore::{PeekMoreIterator, PeekMore};
use logos::{Logos, Lexer, SpannedIter};
use strum::EnumDiscriminants;

//==================================================================================================
//          Libretto Token - Top Level Lexing
//==================================================================================================

pub struct LibrettoTokenQueue<'a, T> where T : Logos<'a> + PartialEq{
  iterator : PeekMoreIterator<SpannedIter<'a, T>>
}

impl <'a, T> From<Lexer<'a, T>> for LibrettoTokenQueue<'a, T> where T : Logos<'a> + PartialEq {
  fn from(value: Lexer<'a, T>) -> Self {
    LibrettoTokenQueue { iterator: value.spanned().peekmore() }
  }
}

impl <'a, T> PartialEq for LibrettoTokenQueue<'a, T> where T : Logos<'a> + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.iterator.eq(other.iterator)
    }
}

impl <'a, T> Debug for LibrettoTokenQueue<'a, T> where T : Logos<'a> + PartialEq{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[Libretto Token Queue]")
    }
}

impl <'a, T> LibrettoTokenQueue<'a, T> where T : Logos<'a> + PartialEq {
  pub fn next_is<D>(&self, token : D) -> bool where {
    let next = self.iterator.peek();
    if next.is_none() { return false }
    let (t, range) = next.unwrap();
    token == *t
  }

  pub fn next_nth_is(&self, token : T, n : usize) -> bool {
    let next = self.iterator.peek_nth(n);
    if next.is_none() { return false }
    let (t, range) = next.unwrap();
    token == *t
  }

  pub fn pop(&mut self) -> Option<(T, Range<usize>)> {
    self.iterator.next()
  }

  pub fn pop_if(&mut self, token : T) -> Option<(T, Range<usize>)> {
    if self.next_is(token) {
      self.pop()
    } else {
      None
    }
  }
}

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

fn as_logic_for_top<'a>(lex : &mut Lexer<'a, LibrettoToken<'a>>) -> LibrettoTokenQueue<'a, LibrettoLogicToken> {
  let content = lex.slice();
  let content = &content[1..content.len()-1];
  let logic_lex = LibrettoLogicToken::lexer(content);
  logic_lex.into()
}

#[derive(Logos, PartialEq, EnumDiscriminants)]
pub enum LibrettoToken<'a> {

  #[regex("#([^ \t\n]*)", content_after_first)]
  Tag(String),
  
  #[regex(":([^ \t\n]*)", content_after_first)]
  Speaker(String),

  #[regex("<([^><]*)>", as_logic_for_top)]
  Logic(LibrettoTokenQueue<'a, LibrettoLogicToken>),

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

fn lex_bool(lex : &mut Lexer<LibrettoLogicToken>) -> bool {
  let content = lex.slice().to_string();
  content.as_str() == "true"
}



#[derive(Debug, Logos, PartialEq)]
pub enum LibrettoLogicToken {

  #[regex("[a-zA-Z0-9_]+", lex_text, priority=1)]
  Identifier(String),

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

fn as_logic_for_quote<'a>(lex : &mut Lexer<'a, LibrettoQuoteToken<'a>>) -> LibrettoTokenQueue<'a, LibrettoLogicToken> {
  let content = lex.slice();
  let content = &content[1..content.len()-1];
  let logic_lex = LibrettoLogicToken::lexer(content);
  logic_lex.into()
}

#[derive(Debug, Logos, PartialEq)]
pub enum LibrettoQuoteToken<'a> {

  // #[regex("[^\\]?[")]
  // LeftBracket,

  #[token("]")]
  RightBracket,

  #[regex("<([^><]*)>", as_logic_for_quote)]
  Logic(LibrettoTokenQueue<'a, LibrettoLogicToken>),

  #[error]
  Error
}