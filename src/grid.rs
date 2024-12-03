use image::{RgbImage, Rgb, imageops};

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
    pub(crate) fn get_rgb(e: &Color) -> Option<[u8; 3]> {
        match e {
            Color::Yellow => Some([252, 242, 80]),
            Color::YellowGreen => Some([189, 211, 82]),
            Color::Green => Some([97, 179, 88]),
            Color::BlueGreen => Some([68, 152, 144]),
            Color::Blue => Some([47, 5, 196]),
            Color::BlueViolet => Some([46, 44, 113]),
            Color::Violet => Some([98, 59, 123]),
            Color::RedViolet => Some([178, 36, 112]),
            Color::Red => Some([228, 50, 48]),
            Color::RedOrange => Some([222, 103, 54]),
            Color::Orange => Some([241, 158, 56]),
            Color::YellowOrange => Some([247, 206, 70]),
            Color::White => Some([255, 255, 255]),
            Color::Black => Some([0, 0, 0]),
        }
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

    fn generate_image(&self, fname: &str) {

        // a default (black) image containing Rgb values
        let mut image = RgbImage::new(self.size.0 as u32, self.size.1 as u32);

        // set a central pixel to white
        for r in 0..self.size.0 {
            for c in 0..self.size.1 {
                image.put_pixel(r as u32, c as u32, Rgb(Color::get_rgb(&self.cells[r][c]).unwrap()));
            }
        }
        // TODO: scale image up. probably need to convert to DynamicImage. https://docs.rs/image/latest/image/enum.DynamicImage.html#method.resize
        // resize(image, 100, 100)
        
        // write it out to a file
        image.save(fname).unwrap();

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

    use crate::grid::Grid;

    #[test]
    fn generate_default_img() { //TODO: this shouldn't be its own test, I don't think? But handy to have this code somewhere. Probably include it in the other tests.
        let g = Grid::default();
        g.generate_image("default.png");
    }
}