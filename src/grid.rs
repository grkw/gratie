#[derive(Debug, Clone, PartialEq)]
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
    cells: Vec<Vec<Color>>, // indexed like a 2d array [row index][col index], *not* like x-y coords
    size: (usize, usize),
}

impl Grid {
    pub(crate) fn new(grid: Vec<Vec<Color>>, height: usize, width: usize) -> Self {
        Grid {
            cells: grid,
            size: (height, width),
        }
    }

    pub(crate) fn print(&self) {
        println!("{:?}", self.cells);
    }

    pub(crate) fn get_color(&self, pos: (usize, usize)) -> Color {
        self.cells[pos.0][pos.1].clone()
    }

    pub(crate) fn find_codel_block(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        //at current position
        let mut q = Vec::new();
        let mut results: Vec<(usize, usize)> = Vec::new();
        
        let codel_color = self.cells[pos.0][pos.1].clone();
        // println!("Codel color: {:?}", codel_color);

        let deltas = [
                   (-1, 0), 
            (0, -1),     (0, 1),
                   (1, 0), 
            ];

        q.push(pos);
        
        while !q.is_empty() {
            let n = q.pop().unwrap();
            if self.cells[n.0][n.1] == codel_color {
                if !results.contains(&(n.0, n.1)) {
                    results.push((n.0, n.1));
                }
 
                for (dx, dy) in deltas {
                     // Check if the cell coords are within bounds
                    let row = n.0 as isize + dx; // Convert to isize to allow for negative values
                    let col = n.1 as isize + dy;
                    let x_valid = row >= 0 && row < self.size.0 as isize;
                    let y_valid = col >= 0 && col < self.size.1 as isize;
                    
                    if x_valid && y_valid {
                        if !q.contains(&(row as usize, col as usize)) && !results.contains(&(row as usize, col as usize)) {
                            q.push((row as usize, col as usize));
                        }
                    }
                }
            }
        }

        // println!("Codels: {:?}", results);
        // println!("Codel block size: {}", results.len());

        results
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

#[cfg(test)]
mod test {

    use crate::grid::{Color, Grid};

    use crate::parsers::{GratieParse, SimpleText};
    use std::{
        fs::{self, File},
        io,
        path::PathBuf,
    };

    #[test]
    fn default_grid() {
        let grid = Grid::default(); // 10x10 white cells
        let codels_in_block = grid.find_codel_block((0,0)); //white
        assert_eq!(codels_in_block.len(), 100);
    }

    #[test]
    fn square_shaped_block() {
        let f = File::open("./tests/txt/valid/square_color_block.txt").expect("could not open input program file");

        // TODO(jph): check file extension to determine parse type; for now, just create a text parser
        let parser = SimpleText::default();
        let grid = parser.parse(f).unwrap();
        let codels_in_block = grid.find_codel_block((0,1)); //blue
        assert_eq!(codels_in_block.len(), 4);
    }

    #[test]
    fn irregular_shaped_blocks() {

        let f = File::open("./tests/txt/valid/irregular_shaped_blocks.txt").expect("could not open input program file");

        // TODO(jph): check file extension to determine parse type; for now, just create a text parser
        let parser = SimpleText::default();
        let grid = parser.parse(f).unwrap();
        let codels_in_block = grid.find_codel_block((2,1)); //yellow
        assert_eq!(codels_in_block.len(), 7);
        
        let codels_in_block = grid.find_codel_block((9,4)); //green
        assert_eq!(codels_in_block.len(), 9);
        
        let codels_in_block = grid.find_codel_block((8,0)); //black
        assert_eq!(codels_in_block.len(), 1);
    }
}