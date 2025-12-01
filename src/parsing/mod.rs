use nom::{IResult, Parser, character::complete::{i8, i16, i32, i64, i128, u8, u16, u32, u64, u128}, combinator::all_consuming};
use thiserror::Error;

use crate::tuples::snd;

pub mod combinators;

pub type ParsingResult<'a, O> = IResult<&'a str, O, NomError<'a>>;
pub type NomError<'a> = nom_language::error::VerboseError<&'a str>;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct ParsingError(String);

pub trait Parsable<'a>: Sized {
    fn parse(input: &'a str) -> ParsingResult<'a, Self>;
}

pub fn parse<'a, O>(input: &'a str) -> Result<O, ParsingError> where
    O: Parsable<'a>
{   
    run_parser(O::parse, input)
}

pub fn parse_lines<'a, O>(input: &'a str) -> Result<Vec<O>, ParsingError> where
    O: Parsable<'a>
{
    input
        .lines()
        .map(parse::<O>)
        .collect()
}

pub fn run_parser<'a, O, P>(parser: P, input: &'a str) -> Result<O, ParsingError> where
    P: Parser<&'a str, Output = O, Error = NomError<'a>>
{
    all_consuming(parser)
        .parse(input)
        .map(snd)
        .map_err(|err| ParsingError(err.to_string()))
}

pub trait ParserExt<'a, O> where
    Self: Parser<&'a str, Output = O, Error = NomError<'a>> + Sized
{
    fn run(self, input: &'a str) -> Result<O, ParsingError> {
        run_parser(self, input)
    }
}

impl<'a, P, O> ParserExt<'a, O> for P where
    P: Parser<&'a str, Output = O, Error = NomError<'a>>,
{}

macro_rules! impl_parsable {
    ($type: ty, $parser: expr) => {
        impl<'a> Parsable<'a> for $type {
            fn parse(input: &'a str) -> ParsingResult<'a, Self> {
                $parser(input)
            }
        }
    }
}

impl_parsable!(u8, u8);
impl_parsable!(i8, i8);
impl_parsable!(u16, u16);
impl_parsable!(i16, i16);
impl_parsable!(u32, u32);
impl_parsable!(i32, i32);
impl_parsable!(u64, u64);
impl_parsable!(i64, i64);
impl_parsable!(u128, u128);
impl_parsable!(i128, i128);