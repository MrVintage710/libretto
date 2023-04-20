use super::LibrettoParsable;
use crate::{
    lexer::{LibrettoLogicToken, LogicOrdinal, Ordinal, LibrettoTokenQueue},
    lson::LsonType,
    parse_ast, compiler::LibrettoCompiletime,
};
use logos::Logos;
use std::{fmt::Debug, marker::PhantomData};

//==================================================================================================
//          Logic Typed Identifier
//==================================================================================================

#[derive(Debug, PartialEq, Eq)]
pub struct TypedIdentifier {
    implicit_type: Option<LsonType>,
    ident: String
}

impl TypedIdentifier {
    pub fn implicit_type(&self) -> Option<LsonType> {
        self.implicit_type
    }

    pub fn ident(&self) -> &str {
        self.ident.as_ref()
    }
}

impl <'a> LibrettoParsable<'a, LibrettoLogicToken> for TypedIdentifier {
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if queue.next_is(LogicOrdinal::Identifier) {
            queue.next_is(LogicOrdinal::Colon) && LsonType::raw_check(queue);
            true
        } else {
            false
        }
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
        if let Some(LibrettoLogicToken::Identifier(ident)) = queue.pop() {
            let implicit_type = {
                if queue.pop_if_next_is(LogicOrdinal::Colon).is_some() {
                    LsonType::parse(queue, compile_time)
                } else {
                    None
                }
            };

            Some(TypedIdentifier { implicit_type, ident })
        } else {
            None
        }
    }

    fn validate(&self, compile_time : &mut LibrettoCompiletime) -> LsonType {
        self.implicit_type.unwrap_or(LsonType::None)
    }
}

//==================================================================================================
//          Key Value Pair
//==================================================================================================

#[derive(Debug, PartialEq)]
pub struct KeyValuePair<'a, P, T>
where
    P: LibrettoParsable<'a, T> + Sized,
    T: Logos<'a> + PartialEq + Ordinal + Clone + Debug + 'a,
    T::Extras: Clone,
    Self: Sized,
{
    key: String,
    value: P,
    _phantom : &'a PhantomData<T>
}

impl <'a, P, T> KeyValuePair<'a, P, T>
where
    P: LibrettoParsable<'a, T> + Sized,
    T: Logos<'a> + PartialEq + Ordinal + Clone + Debug + 'a,
    T::Extras: Clone,
    Self: Sized,
{
    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &P {
        &self.value
    }
}

impl <'a, P> LibrettoParsable<'a, LibrettoLogicToken> for KeyValuePair<'a, P, LibrettoLogicToken>
where
    P: LibrettoParsable<'a, LibrettoLogicToken> + Sized,
    Self: Sized,
{
    fn raw_check(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>) -> bool {
        if !queue.next_is(LogicOrdinal::Identifier) {
            return false;
        }
        if !queue.next_is(LogicOrdinal::Colon) {
            return false;
        }
        if !P::raw_check(queue) {
            return false;
        }
        true
    }

    fn parse(queue: &mut LibrettoTokenQueue<'a, LibrettoLogicToken>, compile_time : &mut LibrettoCompiletime) -> Option<Self> {
        queue.reset();
        let ident = queue.pop_if_next_is(LogicOrdinal::Identifier).unwrap();
        queue.pop_if_next_is(LogicOrdinal::Colon);
        let value = parse_ast!(P, queue, compile_time);

        if let LibrettoLogicToken::Identifier(key) = ident {
            Some(KeyValuePair { key, value, _phantom: &PhantomData })
        } else {
            None
        }
    }

    fn validate(&self, compile_time : &mut LibrettoCompiletime) -> LsonType {
        self.value.validate(compile_time)
    }
}

//==================================================================================================
//          Comma Seporated List
//==================================================================================================

pub struct CommaSeparatedList<'a, P, T>
where
    P: LibrettoParsable<'a, T> + Sized,
    T: Logos<'a> + PartialEq + Ordinal + Clone + Debug + 'a,
    T::Extras: Clone,
    Self: Sized,
{
    values: Vec<P>,
    _phantom: &'a PhantomData<T>,
}

impl<'a, P, T> CommaSeparatedList<'a, P, T>
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

impl<'a, P, T> Debug for CommaSeparatedList<'a, P, T>
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

impl<'a, P> LibrettoParsable<'a, LibrettoLogicToken> for CommaSeparatedList<'a, P, LibrettoLogicToken>
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
        compile_time : &mut LibrettoCompiletime,
    ) -> Option<Self> {
        let mut values = Vec::new();

        queue.reset();
        values.push(parse_ast!(P, queue, compile_time));
        loop {
            if queue.next_is(LogicOrdinal::Comma) && P::raw_check(queue) {
                queue.pop();
                values.push(parse_ast!(P, queue, compile_time))
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

    fn validate(&self, compile_time : &mut LibrettoCompiletime) -> LsonType{
        if self.values.is_empty() {return LsonType::None;}
        let expected_type = self.values.first().unwrap().validate(compile_time);
        let mut return_expected = true;
        
        for i in 1..self.values.len() {
            let element = &self.values[i];
            return_expected = element.validate(compile_time) == expected_type;
        }

        if return_expected {
            expected_type
        } else {
            LsonType::None
        }
    }
}

//==================================================================================================
//         Tests
//==================================================================================================

#[cfg(test)]
pub mod tests {
    use crate::{
        lexer::LibrettoLogicToken,
        lson::Lson,
        parse::{
            util::CommaSeparatedList, test_util::*
        },
    };
    use super::*;

    #[test]
    fn check_logic_list() {
        check_expr::<CommaSeparatedList<Lson, LibrettoLogicToken>>("false", 1);
        check_expr::<CommaSeparatedList<Lson, LibrettoLogicToken>>("false, 3.14", 3);
        check_expr::<CommaSeparatedList<Lson, LibrettoLogicToken>>("false, 3.14, 3", 5);
        check_expr::<CommaSeparatedList<Lson, LibrettoLogicToken>>("false, 3.14, 3, \"test\"", 7);
    }

    #[test]
    fn parse_logic_list() {
        let ast = parse_expr::<CommaSeparatedList<Lson, LibrettoLogicToken>>("false");
    }

    #[test]
    fn check_logic_key_value_pair() {
        check_expr::<KeyValuePair<Lson, LibrettoLogicToken>>("key : \"value\"", 3);
    }

    #[test]
    fn check_typed_ident() {
        check_expr::<TypedIdentifier>("test", 1);
        check_expr::<TypedIdentifier>("some_bool : bool", 3);
        check_expr::<TypedIdentifier>("some_float : float", 3);
        check_expr::<TypedIdentifier>("some_string : string", 3);
    }

    #[test]
    fn parse_typed_ident() {
        let ast = parse_expr::<TypedIdentifier>("test");
        assert_eq!(ast.ident, "test".to_string());
        assert_eq!(ast.implicit_type, None);

        let ast = parse_expr::<TypedIdentifier>("some_bool : bool");
        assert_eq!(ast.ident, "some_bool".to_string());
        assert_eq!(ast.implicit_type, Some(LsonType::Bool));
    }

    #[test]
    fn validate_typed_ident() {
        validate_expr::<TypedIdentifier>("test", 0, LsonType::None);
        validate_expr::<TypedIdentifier>("some_bool : bool", 0, LsonType::Bool);
    }
}
