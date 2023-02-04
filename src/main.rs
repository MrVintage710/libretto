mod lexer;
mod logic;
mod parse;
mod runtime;

use lexer::LibrettoLogicToken;
use logos::{Logos, SpannedIter};

use crate::lexer::LibrettoToken;

fn main() {
    // let mut libretto_lex = LibrettoToken::lexer("| :Mark <let x = 1 + 1.2> \"Line\" #test_tag");

    // for token in libretto_lex {
    //     match token {
    //         LibrettoToken::Tag(value) => println!("Tag: {}", value),
    //         LibrettoToken::Speaker(value) => println!("Speaker: {}", value),
    //         LibrettoToken::Logic(value) => {
    //             for logic_token in value {
    //                 println!("  {:?}", logic_token)
    //             }
    //         },
    //         LibrettoToken::Quote(value) => println!("Quote: {}", value),
    //         _ => println!("{:?}", token)
    //     }

    // }

    let mut logic_lex = LibrettoLogicToken::lexer("false");

    for token in logic_lex {
        println!("{:?}", token);
    }
}
