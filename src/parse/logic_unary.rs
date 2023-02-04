use crate::lexer::{LibrettoLogicToken, LibrettoTokenQueue};

use super::{logic_value::LogicValue, LibrettoParsable, ParseResult};

pub struct LogicUnaryStatement {
    is_nagative: bool,
    value: LogicValue,
}

impl<'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicUnaryStatement {
    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> ParseResult<Self> {
        ParseResult::Failure
    }
    // fn parse(lexer : &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> Option<Self> {
    //   if let logic_value = LogicValue::parse(lexer) {

    //   } else {
    //     let token = lexer.peek();
    //   }

    //   None
    // }
}
