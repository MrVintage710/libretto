use super::{LibrettoCompileResult, LibrettoParsable, ParseResult};
use crate::{
    lexer::{LibrettoLogicToken, LibrettoTokenQueue, LogicOrdinal, Ordinal},
    parse_ast,
};
use logos::Logos;
use std::{fmt::Debug, marker::PhantomData};

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

impl<'a, P> LibrettoParsable<'a, LibrettoLogicToken>
    for ParseCommaSeparatedList<'a, P, LibrettoLogicToken>
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

    fn validate(&self, errors: &mut Vec<super::LibrettoCompileError>) {
        for i in self.values.iter() {
            i.validate(errors)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::{LibrettoLogicToken, LibrettoTokenQueue},
        logic::lson::Lson,
        parse::{
            logic_value::LogicObjectKeyValue, util::ParseCommaSeparatedList, LibrettoParsable,
        },
    };
    use logos::Logos;

    fn check_expr<'a, P: LibrettoParsable<'a, LibrettoLogicToken> + Sized>(
        source: &'a str,
        number_of_tokens: usize,
    ) {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let check = ParseCommaSeparatedList::<'a, P, LibrettoLogicToken>::check(&mut queue);
        assert!(check);
        assert_eq!(queue.cursor(), 0);
        queue.reset();
        let check = ParseCommaSeparatedList::<'a, P, LibrettoLogicToken>::raw_check(&mut queue);
        assert!(check);
        assert_eq!(queue.cursor(), number_of_tokens)
    }

    fn parse_expr<'a, P: LibrettoParsable<'a, LibrettoLogicToken> + Sized>(
        source: &'a str,
    ) -> ParseCommaSeparatedList<'a, P, LibrettoLogicToken> {
        let mut queue = LibrettoTokenQueue::from(LibrettoLogicToken::lexer(source));
        let ast = ParseCommaSeparatedList::<'a, P, LibrettoLogicToken>::checked_parse(
            &mut queue,
            &mut Vec::new(),
        );
        assert!(ast.is_some());
        ast.unwrap()
    }

    #[test]
    fn check_logic_list() {
        check_expr::<Lson>("false", 1);
        check_expr::<Lson>("false, 3.14", 3);
        check_expr::<Lson>("false, 3.14, 3", 5);
        check_expr::<Lson>("false, 3.14, 3, \"test\"", 7);

        check_expr::<LogicObjectKeyValue>("key : false, another : false", 7)
    }

    #[test]
    fn parse_logic_list() {
        let ast = parse_expr::<Lson>("false, 3");
        assert_eq!(ast.values.len(), 2);
        assert_eq!(ast.values[0], Lson::Bool(false));
        assert_eq!(ast.values[1], Lson::Int(3));

        let ast = parse_expr::<LogicObjectKeyValue>("key : false, value : false");
        assert_eq!(ast.values.len(), 2);
        let first = &ast.values[0];
        let second = &ast.values[1];
        assert_eq!(first.key(), "key");
        assert_eq!(first.value(), &Lson::Bool(false));
        assert_eq!(second.key(), "value");
        assert_eq!(second.value(), &Lson::Bool(false));
    }
}
