use nom::{combinator::complete, Parser};
use thiserror::Error;

use crate::tuples::snd;

pub type ParsingResult<'a, T> = Result<(&'a str, T), nom::Err<NomError<'a>>>;
pub type NomError<'a> = nom::error::VerboseError<&'a str>;

#[derive(Debug, Error)]
#[error("{0}")]
pub struct ParsingError(String);

pub trait Parsable<'a>: Sized {
    fn parse(input: &'a str) -> ParsingResult<'a, Self>;
}

pub fn parse<'a, T>(input: &'a str) -> Result<T, ParsingError> where
    T: Parsable<'a>
{
    run_parser(T::parse, input)
}

pub fn run_parser<'a, T, P>(parser: P, input: &'a str) -> Result<T, ParsingError> where
    P: Parser<&'a str, T, NomError<'a>>
{
    complete(parser)(input)
        .map(snd)
        .map_err(|err| ParsingError(err.to_string()))
}