use std::ops::{Range, RangeFrom, RangeTo};

use nom::{character::complete::{char, line_ending}, combinator::map, error::ParseError, multi::separated_list0, sequence::delimited, AsChar, Compare, IResult, InputIter, InputLength, Slice};

pub fn lines<I, O, E, F>(parser: F) -> impl FnMut(I) -> IResult<I, Vec<O>, E> where
    F: FnMut(I) -> IResult<I, O, E>,
    E: ParseError<I>,
    I: Clone + InputLength + InputIter + Compare<&'static str> +
       Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>
{
    separated_list0(
        line_ending,
        parser,
    )
}

pub fn parens<I, O, E, F>(parser: F) -> impl FnMut(I) -> IResult<I, O, E> where
    F: FnMut(I) -> IResult<I, O, E>,
    E: ParseError<I>,
    I: InputIter + Slice<RangeFrom<usize>>,
    <I as InputIter>::Item: AsChar
{
    delimited(char('('), parser, char(')'))
}

pub fn square_brackets<I, O, E, F>(parser: F) -> impl FnMut(I) -> IResult<I, O, E> where
    F: FnMut(I) -> IResult<I, O, E>,
    E: ParseError<I>,
    I: InputIter + Slice<RangeFrom<usize>>,
    <I as InputIter>::Item: AsChar
{
    delimited(char('['), parser, char(']'))
}

pub fn curly_brackets<I, O, E, F>(parser: F) -> impl FnMut(I) -> IResult<I, O, E> where
    F: FnMut(I) -> IResult<I, O, E>,
    E: ParseError<I>,
    I: InputIter + Slice<RangeFrom<usize>>,
    <I as InputIter>::Item: AsChar
{
    delimited(char('{'), parser, char('}'))
}

pub fn angle_brackets<I, O, E, F>(parser: F) -> impl FnMut(I) -> IResult<I, O, E> where
    F: FnMut(I) -> IResult<I, O, E>,
    E: ParseError<I>,
    I: InputIter + Slice<RangeFrom<usize>>,
    <I as InputIter>::Item: AsChar
{
    delimited(char('<'), parser, char('>'))
}

pub fn quoted<I, O, E, F>(parser: F) -> impl FnMut(I) -> IResult<I, O, E> where
    F: FnMut(I) -> IResult<I, O, E>,
    E: ParseError<I>,
    I: InputIter + Slice<RangeFrom<usize>>,
    <I as InputIter>::Item: AsChar
{
    delimited(char('"'), parser, char('"'))
}

pub fn map2<I, O, E, F, M, O1, O2>(parser: F, mapper: M) -> impl FnMut(I) -> IResult<I, O, E>
    where F: FnMut(I) -> IResult<I, (O1, O2), E>,
          M: Fn(O1, O2) -> O,
          E: ParseError<I>
{
    map(parser, move |(a, b)| mapper(a, b))
}

pub trait Map2<I, O1, O2, E> where
    Self: FnMut(I) -> IResult<I, (O1, O2), E> + Sized,
    E: ParseError<I>
{
    fn map2<O, M>(self, mapper: M) -> impl FnMut(I) -> IResult<I, O, E> where
        M: Fn(O1, O2) -> O
    {
        map2(self, mapper)
    }
}

impl<I, O1, O2, E, F> Map2<I, O1, O2, E> for F where
    F: FnMut(I) -> IResult<I, (O1, O2), E>,
    E: ParseError<I>
{}

pub fn map3<I, O, E, F, M, O1, O2, O3>(parser: F, mapper: M) -> impl FnMut(I) -> IResult<I, O, E> where
    F: FnMut(I) -> IResult<I, (O1, O2, O3), E>,
    M: Fn(O1, O2, O3) -> O,
    E: ParseError<I>
{
    map(parser, move |(a, b, c)| mapper(a, b, c))
}

pub trait Map3<I, O1, O2, O3, E> where
    Self: FnMut(I) -> IResult<I, (O1, O2, O3), E> + Sized,
    E: ParseError<I>
{
    fn map3<O, M>(self, mapper: M) -> impl FnMut(I) -> IResult<I, O, E> where
        M: Fn(O1, O2, O3) -> O
    {
        map3(self, mapper)
    }
}

impl<I, O1, O2, O3, E, F> Map3<I, O1, O2, O3, E> for F where
    F: FnMut(I) -> IResult<I, (O1, O2, O3), E>,
    E: ParseError<I>
{}