use logos::Logos;

#[derive(Debug, Logos)]
enum LibrettoToken {

  #[regex("#([^ \t\n]*)")]
  Tag,
  
  #[regex(":([^ \t\n]*)")]
  Speaker,

  #[regex("<([^><]*)>")]
  Logic,

  #[regex("\"([^\"]*)\"")]
  Quote,

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