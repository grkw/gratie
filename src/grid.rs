use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Color {
    Yellow,
    YellowGreen,
    Green,
    BlueGreen,
    Blue,
    BlueViolet,
    Violet,
    RedViolet,
    Red,
    RedOrange,
    Orange,
    YellowOrange,
    White,
    Black,
}

#[derive(Debug)]
pub(crate) struct Grid {
    cells: Vec<Vec<Color>>,
    size: (usize, usize),
}

impl Grid {
    fn new(height: usize, width: usize) -> Self {
        let mut grid = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Color::White)
            }
            grid.push(row);
        }
        Grid {
            cells: grid,
            size: (height, width),
        }
    }

    fn print(&self) {
        println!("{:?}", self.cells);
    }
}
