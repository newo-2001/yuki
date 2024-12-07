use thiserror::Error;

/// An error for when a puzzle has no valid solutions for a given input
#[derive(Debug, Error)]
#[error("The puzzle has no valid solution for the given input")]
pub struct NoSolution;

/// An error for when the solution to the puzzle is ambiguous
#[derive(Debug, Error)]
#[error("Multiple valid solutions possible, the answer is ambiguous")]
pub struct MultipleSolutions;

/// An error for when the input to a puzzle is empty
#[derive(Debug, Error)]
#[error("The puzzle input is empty")]
pub struct NoInput;