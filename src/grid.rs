enum Color {
    White,
    Black
}

struct Grid {
    cells: Vec<Vec<Color>>,
    size: (usize, usize)
}

// Color: white, black, dark green, green,
// Color: hue_value, lightnes_value

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
            size: (height, width)
        }
    }
}
