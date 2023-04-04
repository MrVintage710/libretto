use super::{LibrettoParsable};
use crate::{
    lexer::{LibrettoLogicToken, LogicOrdinal, Ordinal, LibrettoTokenQueue},
    logic::lson::{LsonType},
    parse_ast,
};
use logos::Logos;
use std::{fmt::Debug, marker::PhantomData, collections::HashMap};

pub struct ParseCommaSeparatedList<'a, P, T>
where
    P: LibrettoParsable<'a, T> + Sized,
    T: Logos<'a> + PartialEq + Ordinal + Clone + Debug + 'a,
    T::Extras: Clone,
    Self: Sized,
{
    values: Vec<P>,
    _phantom: &'a PhantomData<T>,
}

impl<'a, P, T> ParseCommaSeparatedList<'a, P, T>
where
    P: LibrettoParsable<'a, T> + Sized,
    T: Logos<'a> + PartialEq + Ordinal + Clone + Debug + 'a,
    T::Extras: Clone,
    Self: Sized,
{
    pub fn values(&self) -> &Vec<P> {
        &self.values
    }
}

impl<'a, P, T> Debug for ParseCommaSeparatedList<'a, P, T>
where
    P: LibrettoParsable<'a, T> + Sized + Debug,
    T: Logos<'a> + PartialEq + Ordinal + Clone + Debug + 'a,
    T::Extras: Clone,
    Self: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParseCommaSeparatedList")
            .field("values", &self.values)
            .finish()
    }
}

impl<'a, P> LibrettoParsable<'a, LibrettoLogicToken> for ParseCommaSeparatedList<'a, P, LibrettoLogicToken>
where
    P: LibrettoParsable<'a, LibrettoLogicToken> + Sized,
{
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if !P::raw_check(queue) {
            return false;
        };
        loop {
            if !(queue.next_is(LogicOrdinal::Comma) && P::raw_check(queue)) {
                break;
            }
        }
        true
    }

    fn parse(
        queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>,
        errors: &mut Vec<super::LibrettoCompileError>,
    ) -> Option<Self> {
        let mut values = Vec::new();

        queue.reset();
        values.push(parse_ast!(P, queue, errors));
        loop {
            if queue.next_is(LogicOrdinal::Comma) && P::raw_check(queue) {
                queue.pop();
                values.push(parse_ast!(P, queue, errors))
            } else {
                break;
            }
        }

        if values.is_empty() {
            Option::None
        } else {
            Some(Self {
                values,
                _phantom: &PhantomData,
            })
        }
    }

    fn validate(&self, errors: &mut Vec<super::LibrettoCompileError>, type_map : &mut HashMap<String, LsonType>) -> LsonType{
        if self.values.is_empty() {return LsonType::None;}
        let expected_type = self.values.first().unwrap().validate(errors, type_map);
        let mut return_expected = true;
        
        for i in 1..self.values.len() {
            let element = &self.values[i];
            return_expected = element.validate(errors, type_map) == expected_type;  
        }

        if return_expected {
            expected_type
        } else {
            LsonType::None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{LibrettoLogicToken, LibrettoTokenQueue},
        logic::lson::Lson,
        parse::{
            logic_value::LogicObjectKeyValue, util::ParseCommaSeparatedList, LibrettoParsable, test_util::*
        },
    };
    use logos::Logos;

    #[test]
    fn check_logic_list() {
        check_expr::<ParseCommaSeparatedList<Lson, LibrettoLogicToken>>("false", 1);
        check_expr::<ParseCommaSeparatedList<Lson, LibrettoLogicToken>>("false, 3.14", 3);
        check_expr::<ParseCommaSeparatedList<Lson, LibrettoLogicToken>>("false, 3.14, 3", 5);
        check_expr::<ParseCommaSeparatedList<Lson, LibrettoLogicToken>>("false, 3.14, 3, \"test\"", 7);

        check_expr::<ParseCommaSeparatedList<LogicObjectKeyValue, LibrettoLogicToken>>("key : false, another : false", 7)
    }

    #[test]
    fn parse_logic_list() {
        let ast = parse_expr::<ParseCommaSeparatedList<Lson, LibrettoLogicToken>>("false");
    }
}
