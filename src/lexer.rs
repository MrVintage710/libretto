use logos::Logos;

#[derive(Debug, Logos)]
enum LibrettoToken {

  #[regex("(\"[^\"]*\")")]
  Quote,

  #[token("|")]
  Bar,

  #[token("request")]
  Request,

  #[regex(r"[ \t\n\f]+", logos::skip)]
  Whitespace,

  #[error]
  Error
}