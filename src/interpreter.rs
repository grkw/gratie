// ./gratie <image file> # start interpreter
//
// repl commands:
// - step: execute one command
// - run: run entire program and print result
//

use std::thread::current;

use crate::grid::{CodelIndex, Color, Grid};
use crate::stack::Stack;

#[derive(Clone)]
pub(crate) struct Interpreter {
    pos: CodelIndex,
    pub(crate) grid: Grid,
    dp: DP,
    cc: CC,
    stack: Stack<i32>,
}

impl Interpreter {
    pub(crate) fn new(grid: Grid) -> Self {
        let init_pos = (0, 0);
        let init_color = grid.get_color(init_pos);
        // let init_block_integer = grid.find_codel_block(init_pos);
        Interpreter {
            pos: init_pos,
            grid: grid,
            dp: DP::RIGHT,
            cc: CC::LEFT,
            stack: Stack::new(),
        }
    }

    fn step() {
        todo!()
    }

    //TODO: use hashmap or hashset?
    fn find_edge_codels(&self, block_codels: &Vec<CodelIndex>) -> Vec<CodelIndex> {
        assert!(!block_codels.is_empty(), "block codels must not be empty"); // the first argument should be true; if it is not, raise an error that says ""

        // Create a local copy which will change the order of block_codels
        let mut block_codels = block_codels.clone();

        // println!("self.dp: {:?}", self.dp);

        // Sort block_codels by the coordinate corresponding to DP
        let sort_key: Box<dyn Fn(&CodelIndex) -> usize> = match self.dp {
            DP::RIGHT | DP::LEFT => {
                block_codels.sort_by_key(|tuple| tuple.1);
                Box::new(|tuple: &CodelIndex| tuple.1)
            }
            DP::UP | DP::DOWN => {
                block_codels.sort_by_key(|tuple| tuple.0);
                Box::new(|tuple: &CodelIndex| tuple.0)
            }
        };
        // println!("block codels: {:?}", block_codels);

        // Determine furthest-edge coordinate to filter block_codels by
        let edge_coord = match self.dp {
            DP::RIGHT => block_codels.last().unwrap().1,
            DP::DOWN => block_codels.last().unwrap().0,
            DP::LEFT => block_codels.first().unwrap().1,
            DP::UP => block_codels.first().unwrap().0,
        };
        // println!("edge coord: {:?}", edge_coord);

        let edge_codels: Vec<CodelIndex> = block_codels
            .into_iter()
            .filter(|c| sort_key(c) == edge_coord)
            .collect();

        edge_codels
    }

    fn find_corner_codel(&self, edge_codels: &Vec<CodelIndex>) -> CodelIndex {
        assert!(!edge_codels.is_empty(), "edge codels cannot be empty");

        // Create a local copy which will change the order of edge_codels
        let mut edge_codels = edge_codels.clone();

        // println!("self.dp: {:?}", self.dp);
        // println!("self.cc: {:?}", self.cc);

        // Sort edge_codels by the coordinate *not* corresponding to DP (the coordinate that CC will use to pick one codel).
        match self.dp {
            DP::RIGHT | DP::LEFT => {
                edge_codels.sort_by_key(|tuple| tuple.0);
            }
            DP::UP | DP::DOWN => {
                edge_codels.sort_by_key(|tuple| tuple.1);
            }
        };
        // println!("block codels: {:?}", edge_codels);

        // Select the corner codel according to CC, from DP's frame of reference.
        let corner_codel = match (self.dp, self.cc) {
            (DP::RIGHT, CC::RIGHT) | (DP::LEFT, CC::LEFT) => edge_codels.last().unwrap(), // bottom-most
            (DP::DOWN, CC::LEFT) | (DP::UP, CC::RIGHT) => edge_codels.last().unwrap(), // right-most
            (DP::DOWN, CC::RIGHT) | (DP::UP, CC::LEFT) => edge_codels.first().unwrap(), // left-most
            (DP::RIGHT, CC::LEFT) | (DP::LEFT, CC::RIGHT) => edge_codels.first().unwrap(), // top-most
        };
        // println!("corner codel: {:?}", *corner_codel);

        *corner_codel
    }
//
// let next_codel = helper function with find_edge_codels + find_corner_codels;
// look at next_codel; if it is not black, an edge, or white, then you're done
// compute command and execute
//
// if next_codel is black or an edge, this is a potential termination condition
// but we need to exhaust all options first
// steps:
// - toggle CC, and try again
// - toggle DP, and try again
// - repeat until a non-terminating codel is found, or you have tried all 8 combinations
//
// if the chosen codel is a white codel, follow DP until a codel is found that is not-white, or an
// edge
//
//
//
//

// The interpreter finds the edge of the current colour block which is furthest in the direction of the DP. (This edge may be disjoint if the block is of a complex shape.)
// The interpreter finds the codel of the current colour block on that edge which is furthest to the CC's direction of the DP's direction of travel. (Visualise this as standing on the program and walking in the direction of the DP; see table at right.)
// The interpreter travels from that codel into the colour block containing the codel immediately in the direction of the DP.

    pub(crate) fn run(&mut self) {
        // let mut current_dp = DP::RIGHT;
        // let mut current_cc = CC::LEFT;
        // (row, column) index
        let mut current_codel = (0, 0);
        let mut terminated = false;

        while !terminated {
            println!("\nNew turn");
            let current_block: Vec<CodelIndex> = self.grid.find_codel_block(current_codel);
            println!("current_block: {:?}", current_block);

            let mut i = 0;
            while i < 8 {
                println!("i: {:?}", i);

                let edge = self.find_edge_codels(&current_block);
                let corner = self.find_corner_codel(&edge);
                println!("corner: {:?}", corner);
                let next_codel = self.get_next_codel(corner);
                println!("next_codel: {:?}", next_codel);
                if next_codel.is_some() {
                    self.execute_command(current_codel, next_codel.unwrap());
                    current_codel = next_codel.unwrap();
                    break;
                }
                
                i += 1;

                if i % 2 == 0 {
                    self.dp = self.dp.get_next();
                } else {
                    self.cc = self.cc.get_next();
                }
            }
            if i == 8 {
                println!("terminated");
                terminated = true;
            }

        }
    }

    fn translate_command(&mut self, color_diff: u8, codel_size: i32) {
        match color_diff {
            7 => self.stack.write_out(true),
            8 => self.stack.read_in(false), //not implemented yet
            9 => {self.stack.pop();},
            10 => self.stack.divide(),
            11 => self.stack.subtract(),
            1 => self.stack.add(),
            2 => self.stack.multiply(),
            3 => self.stack.push(codel_size),
            4 => self.stack.read_in(true), //not implemented yet
            5 => self.stack.write_out(false),
            6 => self.stack.duplicate(),
            _ => panic!("Color diff {:?} is invalid", color_diff),
        };
    }

    fn execute_command(&mut self, prev_codel:CodelIndex, current_codel: CodelIndex) {
        let prev_codel_color = self.grid.cells[prev_codel.0][prev_codel.1];
        let current_codel_color =self.grid.cells[current_codel.0][current_codel.1];
        let prev_id = prev_codel_color.get_color_id().unwrap();
        let current_id = current_codel_color.get_color_id().unwrap();
        let color_diff: u8;
        if prev_id > current_id {
            color_diff = prev_id - current_id;
        } else {
            color_diff = current_id - prev_id;
        }
        println!("EXECUTE_COMMAND:");
        println!("prev_codel: {:?}", prev_codel);
        println!("current_codel: {:?}", current_codel);
        println!("prev_codel_color: {:?}", prev_codel_color);
        println!("current_codel_color: {:?}", current_codel_color);
        println!("Color diff: {:?}", color_diff);

        let mut codel_size: i32 = 0;
        if color_diff == 3 {
            let codel_block = self.grid.find_codel_block(prev_codel);
            codel_size = codel_block.len().try_into().unwrap();
        }
        self.translate_command(color_diff, codel_size);
    }

    // Return Some<CodelIndex>
    // Return None if the chosen codel would terminate the program (black or an edge)
    fn get_next_codel(&self, corner: CodelIndex) -> Option<CodelIndex> {
        let mut next_codel_idx: (isize, isize);
        let mut new_codel;

        println!("self.dp: {:?}", self.dp);
        loop {
            next_codel_idx = match self.dp {
                DP::LEFT => (corner.0 as isize, corner.1 as isize - 1),
                DP::RIGHT => (corner.0 as isize, corner.1 as isize + 1),
                DP::UP => (corner.0 as isize - 1, corner.1 as isize),
                DP::DOWN => (corner.0 as isize + 1, corner.1 as isize),
            };

            // Check if codel is out of bounds of the program.
            if next_codel_idx.0 < 0
                || next_codel_idx.0 >= self.grid.size.0 as isize
                || next_codel_idx.1 < 0
                || next_codel_idx.1 >= self.grid.size.1 as isize
            {
                return None;
            }

            new_codel = self.grid.cells[next_codel_idx.0 as usize][next_codel_idx.1 as usize];
            if new_codel == Color::Black {
                return None;
            }

            if new_codel != Color::White {
                break;
            }
        }

        Some((next_codel_idx.0 as usize, next_codel_idx.1 as usize))
    }

}

#[derive(PartialEq, Copy, Clone, Debug)]
enum DP {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}
impl DP {
    fn get_next(&self) -> Self {
        match self {
            DP::RIGHT => DP::DOWN,
            DP::DOWN => DP::LEFT,
            DP::LEFT => DP::UP,
            DP::UP => DP::RIGHT,
        }
    }
}
#[derive(PartialEq, Copy, Clone, Debug)]
enum CC {
    LEFT,
    RIGHT,
}
impl CC {
    fn get_next(&self) -> Self {
        match self {
            CC::LEFT => CC::RIGHT,
            CC::RIGHT => CC::LEFT,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use super::Interpreter;
    use crate::interpreter::{CC, DP};
    use crate::parsers::{GratieParse, SimpleText};
    use std::fs::File;

    #[test]
    fn test_get_next_codel() {
        let f =
            File::open("./tests/txt/valid/push3.txt").expect("could not open input program file");

        // TODO(jph): check file extension to determine parse type; for now, just create a text parser
        let parser = SimpleText::default();
        let grid = parser.parse(f).unwrap();
        let interp = Interpreter::new(grid);
        assert_eq!(interp.get_next_codel((0,1)),Some((0,2)));
        assert_eq!(interp.get_next_codel((0,2)),None);
    }

    #[test]
    fn push3() {
        let f =
            File::open("./tests/txt/valid/push3.txt").expect("could not open input program file");

        // TODO(jph): check file extension to determine parse type; for now, just create a text parser
        let parser = SimpleText::default();
        let grid = parser.parse(f).unwrap();
        grid.generate_image("tests/png/push3.png", 50);
        let mut interp = Interpreter::new(grid);
        interp.run(); // this is where a debugger would be awesome, since we could have checks at each turn/step of the interpreter?
        assert_eq!(interp.stack.stack.len(), 1);
        assert_eq!(interp.stack.stack[0], 3);
    }

    #[test]
    fn push6() {
        let f =
            File::open("./tests/txt/valid/push6.txt").expect("could not open input program file");

        // TODO(jph): check file extension to determine parse type; for now, just create a text parser
        let parser = SimpleText::default();
        let grid = parser.parse(f).unwrap();
        grid.generate_image("tests/png/push6.png", 50);
        let mut interp = Interpreter::new(grid);
        interp.run(); // this is where a debugger would be awesome, since we could have checks at each turn/step of the interpreter?
        assert_eq!(interp.stack.stack.len(), 1);
        assert_eq!(interp.stack.stack[0], 6);
    }

    #[test]
    fn print7() {
        let f =
            File::open("./tests/txt/valid/print7.txt").expect("could not open input program file");

        // TODO(jph): check file extension to determine parse type; for now, just create a text parser
        let parser = SimpleText::default();
        let grid = parser.parse(f).unwrap();
        grid.generate_image("tests/png/print7.png", 50);
        let mut interp = Interpreter::new(grid);
        interp.run(); // this is where a debugger would be awesome, since we could have checks at each turn/step of the interpreter?
    }

    //TODO: decide if I wanna keep these tests. Before writing them, they seemed to be a good idea. But now that I've written them, they seem silly and unnecessary. Maybe the exercise of writing them out has been the valuable part (rather than their existence).
    //TODO: decide if I wanna split up test functions by "function under test" (rather than input type, which is what I'm currently doing)
    #[test]
    fn vertical_colorblock() {
        let mut interp = Interpreter::new(Grid::default());
        let block_codels = vec![(0, 9), (1, 9), (2, 9), (3, 9)];
        interp.dp = DP::RIGHT;
        assert_eq!(
            interp.find_edge_codels(&block_codels),
            vec![(0, 9), (1, 9), (2, 9), (3, 9)]
        );
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&block_codels), (0, 9));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&block_codels), (3, 9));

        interp.dp = DP::DOWN;
        assert_eq!(interp.find_edge_codels(&block_codels), vec![(3, 9)]);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&vec![(3, 9)]), (3, 9));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&vec![(3, 9)]), (3, 9));

        interp.dp = DP::LEFT;
        assert_eq!(interp.find_edge_codels(&block_codels), block_codels);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&block_codels), (3, 9));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&block_codels), (0, 9));

        interp.dp = DP::UP;
        assert_eq!(interp.find_edge_codels(&block_codels), vec![(0, 9)]);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&vec![(0, 9)]), (0, 9));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&vec![(0, 9)]), (0, 9));
    }

    #[test]
    fn horizontal_colorblock() {
        let mut interp = Interpreter::new(Grid::default());
        let block_codels = vec![(1, 4), (1, 5), (1, 6), (1, 7), (1, 8)];

        interp.dp = DP::RIGHT;
        assert_eq!(interp.find_edge_codels(&block_codels), vec![(1, 8)]);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&vec![(1, 8)]), (1, 8));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&vec![(1, 8)]), (1, 8));

        interp.dp = DP::DOWN;
        assert_eq!(interp.find_edge_codels(&block_codels), block_codels);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&block_codels), (1, 8));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&block_codels), (1, 4));

        interp.dp = DP::LEFT;
        assert_eq!(interp.find_edge_codels(&block_codels), vec![(1, 4)]);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&vec![(1, 4)]), (1, 4));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&vec![(1, 4)]), (1, 4));

        interp.dp = DP::UP;
        assert_eq!(interp.find_edge_codels(&block_codels), block_codels);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&block_codels), (1, 4));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&block_codels), (1, 8));
    }

    #[test]
    fn square_colorblock() {
        let mut interp = Interpreter::new(Grid::default());
        let block_codels = vec![(3, 4), (3, 5), (4, 4), (4, 5)];
        interp.dp = DP::RIGHT;
        assert_eq!(interp.find_edge_codels(&block_codels), vec![(3, 5), (4, 5)]);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&vec![(3, 5), (4, 5)]), (3, 5));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&vec![(3, 5), (4, 5)]), (4, 5));

        interp.dp = DP::DOWN;
        assert_eq!(interp.find_edge_codels(&block_codels), vec![(4, 4), (4, 5)]);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&vec![(4, 4), (4, 5)]), (4, 5));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&vec![(4, 4), (4, 5)]), (4, 4));

        interp.dp = DP::LEFT;
        assert_eq!(interp.find_edge_codels(&block_codels), vec![(3, 4), (4, 4)]);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&vec![(3, 4), (4, 4)]), (4, 4));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&vec![(3, 4), (4, 4)]), (3, 4));

        interp.dp = DP::UP;
        assert_eq!(interp.find_edge_codels(&block_codels), vec![(3, 4), (3, 5)]);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&vec![(3, 4), (3, 5)]), (3, 4));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&vec![(3, 4), (3, 5)]), (3, 5));
    }

    #[test]
    fn irregular_shaped_block() {
        let mut interp = Interpreter::new(Grid::default());
        let block_codels = vec![
            (2, 0),
            (2, 1),
            (2, 3),
            (3, 0),
            (3, 1),
            (3, 2),
            (3, 3),
            (4, 0),
            (4, 1),
            (5, 1),
        ];
        interp.dp = DP::RIGHT;
        assert_eq!(interp.find_edge_codels(&block_codels), vec![(2, 3), (3, 3)]);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&vec![(2, 3), (3, 3)]), (2, 3));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&vec![(2, 3), (3, 3)]), (3, 3));

        interp.dp = DP::DOWN;
        assert_eq!(interp.find_edge_codels(&block_codels), vec![(5, 1)]);
        interp.cc = CC::LEFT;
        assert_eq!(interp.find_corner_codel(&vec![(5, 1)]), (5, 1));
        interp.cc = CC::RIGHT;
        assert_eq!(interp.find_corner_codel(&vec![(5, 1)]), (5, 1));

        interp.dp = DP::LEFT;
        assert_eq!(
            interp.find_edge_codels(&block_codels),
            vec![(2, 0), (3, 0), (4, 0)]
        );
        interp.cc = CC::LEFT;
        assert_eq!(
            interp.find_corner_codel(&vec![(2, 0), (3, 0), (4, 0)]),
            (4, 0)
        );
        interp.cc = CC::RIGHT;
        assert_eq!(
            interp.find_corner_codel(&vec![(2, 0), (3, 0), (4, 0)]),
            (2, 0)
        );

        interp.dp = DP::UP;
        assert_eq!(
            interp.find_edge_codels(&block_codels),
            vec![(2, 0), (2, 1), (2, 3)]
        );
        interp.cc = CC::LEFT;
        assert_eq!(
            interp.find_corner_codel(&vec![(2, 0), (2, 1), (2, 3)]),
            (2, 0)
        );
        interp.cc = CC::RIGHT;
        assert_eq!(
            interp.find_corner_codel(&vec![(2, 0), (2, 1), (2, 3)]),
            (2, 3)
        );
    }
}
