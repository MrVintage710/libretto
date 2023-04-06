use logos::{Lexer, Logos};
use crate::lson::LsonType;
use peekmore::{PeekMore, PeekMoreIterator};
use std::{fmt::Debug, marker::PhantomData};
use strum::EnumDiscriminants;

//==================================================================================================
//          Libretto Token Queue
//==================================================================================================

#[derive(Clone)]
pub struct LibrettoTokenQueue<'a, T>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal + Debug,
    T::Extras: Clone,
{
    iterator: PeekMoreIterator<Lexer<'a, T>>,
    cursor: usize,
}

impl<'a, T> From<Lexer<'a, T>> for LibrettoTokenQueue<'a, T>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal + Debug,
    T::Extras: Clone,
{
    fn from(value: Lexer<'a, T>) -> Self {
        LibrettoTokenQueue {
            iterator: value.peekmore(),
            cursor: 0,
        }
    }
}

impl<'a, T> PartialEq for LibrettoTokenQueue<'a, T>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal + Debug,
    T::Extras: Clone,
{
    fn eq(&self, other: &Self) -> bool {
        //Filthy hack
        true
    }
}

impl<'a, T> Debug for LibrettoTokenQueue<'a, T>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal + Debug,
    T::Extras: Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let proxy = self.iterator.clone();
        let mut formater = f.debug_list();
        for token in proxy {
            formater.entry(&token);
        }
        formater.finish()
    }
}

impl<'a, T> LibrettoTokenQueue<'a, T>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal + Debug + Debug + 'a,
    T::Extras: Clone,
{
    pub fn rewind(&mut self) {
        self.cursor = 0;
    }

    pub fn mark(&mut self) {
        self.iterator.advance_cursor_by(self.cursor);
        self.cursor = 0
    }

    pub fn reset(&mut self) {
        self.iterator.reset_cursor();
        self.cursor = 0;
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Gives the count of the queue. WARNING: This clones the iterator. Very Slow
    pub fn length(&self) -> usize {
        self.iterator.clone().count()
    }

    pub fn next_is<D: From<T> + PartialEq + Copy>(
        &mut self,
        ordinal_group: impl Into<OrdinalGroup<'a, T, D>>,
    ) -> bool {
        let next = self.iterator.peek_nth(self.cursor);
        if next.is_none() {
            return false;
        }
        let t = next.unwrap();
        let ordinal_group: OrdinalGroup<'a, T, D> = ordinal_group.into();
        let next_is = ordinal_group.check_ordinal(t);
        if next_is {
            self.cursor += 1
        };
        next_is
    }

    pub fn next_nth_is<D: From<T> + PartialEq + Copy>(
        &mut self,
        ordinal_group: impl Into<OrdinalGroup<'a, T, D>>,
        n: usize,
    ) -> bool {
        let next = self.iterator.peek_nth(self.cursor + n);
        if next.is_none() {
            return false;
        }
        let t = next.unwrap();
        let ordinal_group: OrdinalGroup<'a, T, D> = ordinal_group.into();
        let next_is = ordinal_group.check_ordinal(t);
        if next_is {
            self.cursor += 1
        };
        next_is
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.cursor != 0 {
            self.cursor -= 1
        };
        self.iterator.next()
    }

    pub fn pop_if_next_is<D: From<T> + PartialEq + Copy>(
        &mut self,
        ordinal_group: impl Into<OrdinalGroup<'a, T, D>>,
    ) -> Option<T> {
        self.reset();
        if self.next_is(ordinal_group) {
            self.pop()
        } else {
            None
        }
    }

    pub fn pop_and_check_if<D: From<T> + PartialEq + Copy>(
        &mut self,
        ordinal_group: impl Into<OrdinalGroup<'a, T, D>>,
    ) -> bool {
        if let Some(token) = self.pop() {
            let ordinal_group: OrdinalGroup<'a, T, D> = ordinal_group.into();
            ordinal_group.check_ordinal(&token)
        } else {
            false
        }
    }

    pub fn pop_until<D: From<T> + PartialEq + Copy>(
        &mut self,
        ordinal_group: impl Into<OrdinalGroup<'a, T, D>>,
    ) -> Vec<T> {
        let mut tokens = Vec::new();
        let ordinal_group: OrdinalGroup<'a, T, D> = ordinal_group.into();
        while (!self.next_is(ordinal_group.clone())) {
            if let Some(token) = self.pop() {
                tokens.push(token)
            }
        }
        tokens
    }
}

//==================================================================================================
//          Ordinal - For Checking Enums
//==================================================================================================

pub trait Ordinal: Sized + Clone {
    fn check_ordinal<T>(&self, ordinal: T) -> bool
    where
        T: PartialEq + From<Self>,
    {
        let other = T::from(self.clone());
        ordinal == other
    }
}

//==================================================================================================
//          Ordinal Groups
//==================================================================================================

#[derive(Clone, PartialEq)]
pub struct OrdinalGroup<'a, T, D>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal,
    T::Extras: Clone,
    D: From<T> + PartialEq + Copy,
{
    tokens: Vec<D>,
    _phantom: &'a PhantomData<T>,
}

impl<'a, D, T, const COUNT: usize> From<[D; COUNT]> for OrdinalGroup<'a, T, D>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal,
    T::Extras: Clone,
    D: From<T> + PartialEq + Copy,
{
    fn from(value: [D; COUNT]) -> Self {
        OrdinalGroup {
            tokens: Vec::from(value),
            _phantom: &PhantomData,
        }
    }
}

impl<'a, D, T> From<D> for OrdinalGroup<'a, T, D>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal,
    T::Extras: Clone,
    D: From<T> + PartialEq + Copy,
{
    fn from(value: D) -> Self {
        let mut tokens = Vec::new();
        tokens.push(value);
        OrdinalGroup {
            tokens,
            _phantom: &PhantomData,
        }
    }
}

impl<'a, D, T> OrdinalGroup<'a, T, D>
where
    T: Logos<'a> + PartialEq + Clone + Ordinal,
    T::Extras: Clone,
    D: From<T> + PartialEq + Copy,
{
    fn check_ordinal(&self, token: &T) -> bool {
        for inner in self.tokens.iter() {
            if token.check_ordinal(*inner) {
                return true;
            }
        }

        false
    }
}

//==================================================================================================
//          Libretto Token - Top Level Lexing
//==================================================================================================

fn content_after_first<'a>(lex: &mut Lexer<'a, LibrettoToken<'a>>) -> String {
    lex.slice()[1..].to_string()
}

fn content_except_first_and_last<'a>(lex: &mut Lexer<'a, LibrettoToken<'a>>) -> String {
    let content = lex.slice();
    content[1..content.len() - 1].to_string()
}

fn as_logic_for_top<'a>(
    lex: &mut Lexer<'a, LibrettoToken<'a>>,
) -> LibrettoTokenQueue<'a, LibrettoLogicToken> {
    let content = lex.slice();
    let content = &content[1..content.len() - 1];
    let logic_lex = LibrettoLogicToken::lexer(content);
    logic_lex.into()
}

impl<'a> Ordinal for LibrettoToken<'a> {}

#[derive(Logos, PartialEq, EnumDiscriminants, Clone)]
#[strum_discriminants(name(TokenOrdinal))]
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
    Error,
}

//==================================================================================================
//          Libretto Logic Token - Logic Level Lexing
//==================================================================================================

fn lex_string(lex: &mut Lexer<LibrettoLogicToken>) -> String {
    let content = lex.slice();
    content[1..content.len() - 1].to_string()
}

fn lex_text(lex: &mut Lexer<LibrettoLogicToken>) -> String {
    lex.slice().to_string()
}

fn lex_int(lex: &mut Lexer<LibrettoLogicToken>) -> i64 {
    let content = lex.slice().to_string();
    content.parse::<i64>().unwrap()
}

fn lex_float(lex: &mut Lexer<LibrettoLogicToken>) -> f64 {
    let content = lex.slice().to_string();
    content.parse::<f64>().unwrap()
}

fn lex_bool(lex: &mut Lexer<LibrettoLogicToken>) -> bool {
    let content = lex.slice().to_string();
    println!("{}", content);
    content.as_str() == "true"
}

fn lex_type(lex: &mut Lexer<LibrettoLogicToken>) -> LsonType {
    match lex.slice() {
        "float" => LsonType::Float,
        "int" => LsonType::Int,
        "string" => LsonType::String,
        "bool" => LsonType::Bool,
        "struct" => LsonType::Struct,
        "array" => LsonType::Array,
        _ => LsonType::None
    }
}

impl<'a> Ordinal for LibrettoLogicToken {}

#[derive(Debug, Logos, PartialEq, Clone, EnumDiscriminants)]
#[strum_discriminants(name(LogicOrdinal))]
pub enum LibrettoLogicToken {
    #[regex("[a-zA-Z0-9_]+", lex_text, priority = 1)]
    Identifier(String),

    #[regex("[0-9]+", lex_int, priority = 2)]
    IntLiteral(i64),

    #[regex(r"([0-9]+)\.([0-9]+)", lex_float, priority = 3)]
    FloatLiteral(f64),

    #[regex("(true|false)", lex_bool, priority=4)]
    BoolLiteral(bool),

    #[regex("(float|int|string|bool|struct|array)", lex_type, priority=4)]
    Type(LsonType),

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

    #[token("!")]
    Bang,

    #[token("?")]
    Question,

    #[token(",")]
    Comma,

    #[token(":")]
    Colon,

    #[token("!=", priority = 2)]
    InverseEquality,

    #[token("==", priority = 2)]
    Equality,

    #[token("<=", priority = 2)]
    LessThanEquality,

    #[token(">=", priority = 2)]
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
    Error,
}

//==================================================================================================
//          Libretto Quote Token - Quote Level Lexing
//==================================================================================================

fn as_logic_for_quote<'a>(
    lex: &mut Lexer<'a, LibrettoQuoteToken<'a>>,
) -> LibrettoTokenQueue<'a, LibrettoLogicToken> {
    let content = lex.slice();
    let content = &content[1..content.len() - 1];
    let logic_lex = LibrettoLogicToken::lexer(content);
    logic_lex.into()
}

impl<'a> Ordinal for LibrettoQuoteToken<'a> {}

#[derive(Debug, Logos, PartialEq, Clone, EnumDiscriminants)]
#[strum_discriminants(name(QuoteOrdinal))]
pub enum LibrettoQuoteToken<'a> {
    // #[regex("[^\\]?[")]
    // LeftBracket,
    #[token("]")]
    RightBracket,

    #[regex(r"<([^><]*)>", as_logic_for_quote)]
    Logic(LibrettoTokenQueue<'a, LibrettoLogicToken>),

    #[error]
    Error,
}