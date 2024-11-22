use anyhow::Result;
use std::fs::File;
use thiserror::Error;

use crate::grid::Grid;

/// Gratie input program parsers
///
/// Currently supported input formats:
/// * simple text file of comma-separated color values, each color on a different row
///
/// Possible extensions:
/// * real image files (e.g., PNGs or JPEGs)
///

pub(crate) trait GratieParse {
    fn parse(&self, f: File) -> Result<Grid>;
}

pub(crate) struct SimpleText {
    col_delimiter: char,
    row_delimiter: char,
}

impl Default for SimpleText {
    fn default() -> Self {
        Self {
            col_delimiter: ',',
            row_delimiter: '\n',
        }
    }
}

#[derive(Error, Debug)]
pub(crate) enum SimpleTextParseError {
    /// Found color not supported by the gratie command set
    #[error("Invalid color \"{color:?}\" (row {row:?}, col {col:?})")]
    InvalidColor {
        color: String,
        row: u32,
        col: u32,
        msg: String,
    },

    /// Number of columns does not match for every row
    #[error(
        "Invalid number of cols on row {row:?}: expected {expected_cols:?}, but found {ncols:?}"
    )]
    InvalidColCount {
        expected_cols: u32,
        ncols: u32,
        row: u32,
    },

    #[error("Invalid row delimiter on row {row:?}, col {col:?}: expected \"{expected:?}\", but found \"{found:?}\"")]
    UnexpectedRowDelimiter {
        expected: char,
        found: char,
        row: u32,
        col: u32,
    },

    #[error("Invalid col delimiter on row {row:?}, col {col:?}: expected \"{expected:?}\", but found \"{found:?}\"")]
    UnexpectedColDelimiter {
        expected: char,
        found: char,
        row: u32,
        col: u32,
    },
}

impl GratieParse for SimpleText {
    fn parse(&self, f: File) -> Result<Grid> {
        todo!()
    }
}
