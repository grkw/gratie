#[derive(Debug)]
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

impl Color {
    pub(crate) fn parse(s: &str) -> Option<Color> {
        let c = match s.to_lowercase().as_str() {
            "yellow" => Self::Yellow,
            "yellowgreen" => Self::YellowGreen,
            "green" => Self::Green,
            "bluegreen" => Self::BlueGreen,
            "blue" => Self::Blue,
            "blueviolet" => Self::BlueViolet,
            "violet" => Self::Violet,
            "redviolet" => Self::RedViolet,
            "red" => Self::Red,
            "redorange" => Self::RedOrange,
            "orange" => Self::Orange,
            "yelloworange" => Self::YellowOrange,
            "white" => Self::White,
            "black" => Self::Black,
            _ => return None,
        };

        Some(c)
    }
}

#[derive(Debug)]
pub(crate) struct Grid {
    cells: Vec<Vec<Color>>,
    size: (usize, usize),
}

impl Grid {
    pub(crate) fn new(grid: Vec<Vec<Color>>, height: usize, width: usize) -> Self {
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

impl Default for Grid {
    fn default() -> Self {
        let height = 10;
        let width = 10;

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
}
