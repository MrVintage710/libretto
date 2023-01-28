mod lexer;

use lexer::LibrettoLogicToken;
use logos::Logos;

use crate::lexer::LibrettoToken;

fn main() {
    let mut libretto_lex = LibrettoToken::lexer("| :Mark <Logic> \"Line\" #test_tag");

    for token in libretto_lex {
        match token {
            LibrettoToken::Tag(value) => println!("Tag: {}", value),
            LibrettoToken::Speaker(value) => println!("Speaker: {}", value),
            LibrettoToken::Logic(value) => println!("Logic: {}", value),
            LibrettoToken::Quote(value) => println!("Quote: {}", value),
            _ => println!("{:?}", token)
        }
        
    }

    let mut logic_lex = LibrettoLogicToken::lexer("let float_var = 3.14 + 10");

    for token in logic_lex {
        println!("{:?}", token);
    }
}
