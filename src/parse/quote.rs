use crate::{parse::LibrettoParsable, lexer::{LibrettoQuoteToken, QuoteOrdinal}};

// "My name is <player.name>" => "My name is Steven"
// "I am [wavy] very mad <player.name ? "Steven"> [/]" {}

// "[yell] I am [wavy] very [\wavy] mad.[\]"


enum TagGroup {
    Tag(String, Vec<TagGroup>),
    Segment(QuoteSegment)
}

enum QuoteSegment {
    Dialog (String),
    Eval
}

// TODO: impl each of these
impl<'a> LibrettoParsable<'a, LibrettoQuoteToken<'a>> for QuoteSegment {
    fn raw_check(queue: &mut crate::lexer::LibrettoTokenQueue<'a, LibrettoQuoteToken<'a>>) -> bool {
        queue.next_is(QuoteOrdinal::Text) || queue.next_is(QuoteOrdinal::Logic)
    }

    // only runs if you pass the raw check (if used correctly)
    fn parse(
        queue: &mut crate::lexer::LibrettoTokenQueue<'a, LibrettoQuoteToken<'a>>,
        errors: &mut Vec<super::LibrettoCompileError>,
    ) -> Option<Self> {
        if let Some(token) = queue.pop() {
            match token {
                LibrettoQuoteToken::Text(dialogue) => {
                    return Some(QuoteSegment::Dialog(dialogue))
                },
                LibrettoQuoteToken::Logic(logic_tokens) => {
                    // evaluate the logic tokens here >> are the good?
                    // Note : Brooks with plug this in when ready.
                    todo!()
                },
                _ => return None,
            }
        }
        None
    }

    fn validate(&self, errors: &mut Vec<super::LibrettoCompileError>) {}
}

// RAW STRING -> [LEXER] -> TOKENS -> [PARSING] -> (AST,evaluator) ; can do ast.eval()

#[cfg(test)]
mod QuoteParsingTests {
    use crate::{lexer::{LibrettoTokenQueue, LibrettoQuoteToken}, parse::LibrettoParsable};
    use logos::Logos;

    use super::QuoteSegment;

    fn check_expr<'a, T: LibrettoParsable<'a, LibrettoQuoteToken<'a>>>(
        source: &'a str,
        number_of_tokens: usize,
    ) {
        let mut queue = LibrettoTokenQueue::from(LibrettoQuoteToken::lexer(source));
        let check = T::check(&mut queue);
        assert!(check);
        assert_eq!(queue.cursor(), 0);
        queue.reset();
        let check = T::raw_check(&mut queue);
        assert!(check);
        assert_eq!(queue.cursor(), number_of_tokens)
    }

    fn parse_expr<'a, T: LibrettoParsable<'a, LibrettoQuoteToken<'a>>>(source: &'a str) -> T {
        let mut queue = LibrettoTokenQueue::from(LibrettoQuoteToken::lexer(source));
        let result = T::checked_parse(&mut queue, &mut Vec::new());
        assert!(result.is_some());
        result.unwrap()
    }

    #[test]
    fn basic_test() {
        let tokens = LibrettoTokenQueue::from(LibrettoQuoteToken::lexer("My name is <player.name ? \"Kristoff\">. [yell]It's nice to meet you.[/]"));
//        let segments = QuoteSegment
        println!("{:?}", tokens);

    }

    #[test]
    fn check_quote_segment() {
        check_expr::<QuoteSegment>("Hello World!", 1);
        check_expr::<QuoteSegment>("<player.name ? \"Kristoff\">", 1);
    }

    #[test]
    fn parse_quote_segment() {
        let ast = parse_expr::<QuoteSegment>("Hello World!");
        if let QuoteSegment::Dialog(value) = ast {
            assert_eq!("Hello World!".to_string(), value)
        }
    }
}