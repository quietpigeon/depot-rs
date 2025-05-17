use nom::character::complete::{alphanumeric1, multispace0};
use nom::character::complete::{char, space0};
use nom::combinator::recognize;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::{IResult, Parser, multi::separated_list1};

/// A parser that recognizes strings that contain `_` as a word.
pub fn alphanumeric1_with_hyphen(s: &str) -> IResult<&str, &str> {
    recognize(separated_list1(char('-'), alphanumeric1)).parse(s)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace (excluding `\n`), returning the output of `inner`.
pub fn ws2<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(space0, inner, space0)
}

#[cfg(test)]
mod tests {
    use super::{alphanumeric1_with_hyphen, ws2};
    use crate::parser::ws;
    use nom::{IResult, Parser, bytes::complete::tag};

    fn ws_wrapper<'a>(s1: &'a str, s2: &'a str) -> IResult<&'a str, &'a str> {
        ws(tag(s1)).parse(s2)
    }

    fn ws2_wrapper<'a>(s1: &'a str, s2: &'a str) -> IResult<&'a str, &'a str> {
        ws2(tag(s1)).parse(s2)
    }

    #[test]
    fn parse_alphanumeric1_with_hyphen() {
        assert_eq!(
            alphanumeric1_with_hyphen("hello-world").unwrap().1,
            "hello-world"
        );
        assert_eq!(
            alphanumeric1_with_hyphen("hello-world-world").unwrap().1,
            "hello-world-world"
        );
        assert_eq!(alphanumeric1_with_hyphen("hello").unwrap().1, "hello");
    }

    #[test]
    fn parse_with_ws() {
        assert_eq!(ws_wrapper("foo", " \nfoo ").unwrap().1, "foo")
    }

    #[test]
    fn parse_with_ws2() {
        assert_eq!(ws2_wrapper("foo", "   foo\n").unwrap().0, "\n")
    }
}
