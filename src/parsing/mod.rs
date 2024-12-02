use nom::{combinator::complete, Parser};
use thiserror::Error;

use crate::tuples::snd;

pub mod combinators;

pub type ParsingResult<'a, O> = Result<(&'a str, O), nom::Err<NomError<'a>>>;
pub type NomError<'a> = nom::error::VerboseError<&'a str>;

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
    P: Parser<&'a str, O, NomError<'a>>
{
    complete(parser)(input)
        .map(snd)
        .map_err(|err| ParsingError(err.to_string()))
}

pub trait ParserExt<'a, O> where
    Self: Parser<&'a str, O, NomError<'a>> + Sized
{
    fn run(self, input: &'a str) -> Result<O, ParsingError> {
        run_parser(self, input)
    }
}

impl<'a, P, O> ParserExt<'a, O> for P where
    P: Parser<&'a str, O, NomError<'a>>,
{}