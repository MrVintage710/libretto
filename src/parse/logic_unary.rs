use crate::lexer::{LibrettoLogicToken, LibrettoTokenQueue};

use super::{logic_value::LogicValue, LibrettoParsable};


pub struct LogicUnaryStatement {
  is_nagative : bool,
  value : LogicValue
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for LogicUnaryStatement {
    fn parse(lexer : &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> Option<Self> {
        todo!()
    }
}