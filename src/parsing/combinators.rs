use std::ops::{Range, RangeFrom, RangeTo};

use nom::{character::complete::line_ending, error::ParseError, multi::separated_list0, Compare, IResult, InputIter, InputLength, Slice};

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