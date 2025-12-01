use nom::{AsChar, Compare, Input, Parser, character::complete::{char, line_ending}, combinator::map, error::ParseError, multi::separated_list0, sequence::delimited};

pub fn lines<I, O, E, F>(parser: F) -> impl Parser<I, Output = Vec<O>, Error = E> where
    F: Parser<I, Output = O, Error = E>,
    E: ParseError<I>,
    I: Clone + Input + Compare<&'static str> +
{
    separated_list0(
        line_ending,
        parser,
    )
}

pub fn parens<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E> where
    F: Parser<I, Output = O, Error = E>,
    E: ParseError<I>,
    I: Input,
    I::Item: AsChar
{
    delimited(char('('), parser, char(')'))
}

pub fn square_brackets<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E> where
    F: Parser<I, Output = O, Error = E>,
    E: ParseError<I>,
    I: Input,
    I::Item: AsChar
{
    delimited(char('['), parser, char(']'))
}

pub fn curly_brackets<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E> where
    F: Parser<I, Output = O, Error = E>,
    E: ParseError<I>,
    I: Input,
    I::Item: AsChar
{
    delimited(char('{'), parser, char('}'))
}

pub fn angle_brackets<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E> where
    F: Parser<I, Output = O, Error = E>,
    E: ParseError<I>,
    I: Input,
    I::Item: AsChar
{
    delimited(char('<'), parser, char('>'))
}

pub fn quoted<I, O, E, F>(parser: F) -> impl Parser<I, Output = O, Error = E> where
    F: Parser<I, Output = O, Error = E>,
    E: ParseError<I>,
    I: Input,
    I::Item: AsChar
{
    delimited(char('"'), parser, char('"'))
}

pub fn map2<I, O, E, F, M, O1, O2>(parser: F, mapper: M) -> impl Parser<I, Output = O, Error = E>
    where F: Parser<I, Output = (O1, O2), Error = E>,
          M: Fn(O1, O2) -> O,
          E: ParseError<I>
{
    map(parser, move |(a, b)| mapper(a, b))
}

pub trait Map2<I, O1, O2, E> where
    Self: Parser<I, Output = (O1, O2), Error = E> + Sized,
    E: ParseError<I>
{
    fn map2<O, M>(self, mapper: M) -> impl Parser<I, Output = O, Error = E> where
        M: Fn(O1, O2) -> O
    {
        map2(self, mapper)
    }
}

impl<I, O1, O2, E, F> Map2<I, O1, O2, E> for F where
    F: Parser<I, Output = (O1, O2), Error = E>,
    E: ParseError<I>
{}

pub fn map3<I, O, E, F, M, O1, O2, O3>(parser: F, mapper: M) -> impl Parser<I, Output = O, Error = E> where
    F: Parser<I, Output = (O1, O2, O3), Error = E>,
    M: Fn(O1, O2, O3) -> O,
    E: ParseError<I>
{
    map(parser, move |(a, b, c)| mapper(a, b, c))
}

pub trait Map3<I, O1, O2, O3, E> where
    Self: Parser<I, Output = (O1, O2, O3), Error = E> + Sized,
    E: ParseError<I>
{
    fn map3<O, M>(self, mapper: M) -> impl Parser<I, Output = O, Error = E> where
        M: Fn(O1, O2, O3) -> O
    {
        map3(self, mapper)
    }
}

impl<I, O1, O2, O3, E, F> Map3<I, O1, O2, O3, E> for F where
    F: Parser<I, Output = (O1, O2, O3), Error = E>,
    E: ParseError<I>
{}