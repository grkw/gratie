use anyhow::{anyhow, Result};
use std::{fs::File, io::Read};
use thiserror::Error;

use crate::grid::{self, Grid};

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
    #[error("Invalid color {color:?} (row {row:?}, col {col:?})")]
    InvalidColor {
        color: String,
        row: usize,
        col: usize,
    },

    /// Number of columns does not match for every row
    #[error(
        "Invalid number of cols on row {row:?}: expected {expected_cols:?}, but found {ncols:?}"
    )]
    InvalidColCount {
        expected_cols: usize,
        ncols: usize,
        row: usize,
    },

    #[error("Invalid row delimiter on row {row:?}, col {col:?}: expected \"{expected:?}\", but found \"{found:?}\"")]
    _UnexpectedRowDelimiter {
        expected: char,
        found: char,
        row: usize,
        col: usize,
    },

    #[error("Invalid col delimiter on row {row:?}, col {col:?}: expected \"{expected:?}\", but found \"{found:?}\"")]
    _UnexpectedColDelimiter {
        expected: char,
        found: char,
        row: usize,
        col: usize,
    },
}

impl GratieParse for SimpleText { // impl TraitName for TypeName
    fn parse(&self, mut f: File) -> Result<Grid> {
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        // Parse program into a 2D vec, based on the row and column delimiters.
        let parsed_rows: Vec<&str> = contents.split(self.row_delimiter).collect();
        let mut parsed_program: Vec<Vec<&str>> = Vec::new();
        for i in 0..parsed_rows.len() {
            // Skip the last row (if the delimiter, such as a newline, is at the end of the file).
            if i == (parsed_rows.len() - 1) && parsed_rows[i] == "" {
                break;
            }

            let r: Vec<&str> = parsed_rows[i].split(self.col_delimiter).collect();
            parsed_program.push(r);
        }

        // Do some basic validation:
        // - the file is non-empty
        // - all the rows have the same width
        if parsed_rows.len() == 0 || parsed_rows[0] == "" {
            return Err(anyhow!("empty program file"));
        }

        //println!("parsed_rows: {:?}", parsed_rows);
        //println!("parsed_program: {:?}", parsed_program);

        let width = parsed_program[0].len();
        for i in 0..parsed_program.len() {
            let len = parsed_program[i].len();
            if len != width {
                return Err(anyhow!(SimpleTextParseError::InvalidColCount {
                    expected_cols: width,
                    ncols: len,
                    row: i,
                }));
            }
        }

        // Attempt to convert all the text into valid colors for a grid.
        let mut cells = Vec::new();
        for i in 0..parsed_program.len() {
            let row = &parsed_program[i];

            let mut grid_row = Vec::new();
            for j in 0..row.len() {
                if let Some(color) = grid::Color::parse(row[j]) {
                    grid_row.push(color);
                } else {
                    return Err(anyhow!(SimpleTextParseError::InvalidColor {
                        color: row[j].to_string(),
                        row: i,
                        col: j,
                    }));
                }
            }
            cells.push(grid_row);
        }

        let height = cells.len();
        Ok(Grid::new(cells, height, width))
    }
}

#[cfg(test)]
mod test {
    use crate::parsers::{GratieParse, SimpleText};
    use std::{
        fs::{self, File},
        io,
        path::PathBuf,
    };

    const VALID_TESTS_DIR: &str = "./tests/txt/valid";
    const INVALID_TESTS_DIR: &str = "./tests/txt/invalid";

    #[test]
    fn valid_inputs() {
        let mut paths = fs::read_dir(VALID_TESTS_DIR)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .map(|res| res.unwrap())
            .collect::<Vec<PathBuf>>();

        let stp = SimpleText::default();
        for p in paths {
            println!("testing valid program at path: {:?}", p);
            let f = File::open(p).unwrap();
            assert!(stp.parse(f).is_ok());
        }
    }

    #[test]
    fn invalid_inputs() {
        let mut paths = fs::read_dir(INVALID_TESTS_DIR)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .map(|res| res.unwrap())
            .collect::<Vec<PathBuf>>();

        let stp = SimpleText::default();
        for p in paths {
            println!("testing invalid program at path: {:?}", p);
            let f = File::open(p).unwrap();
            assert!(stp.parse(f).is_err());
        }
    }
}
