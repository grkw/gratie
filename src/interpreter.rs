// ./gratie <image file> # start interpreter
//
// repl commands:
// - step: execute one command
// - run: run entire program and print result
//

// The interpreter finds the edge of the current colour block which is furthest in the direction of the DP. (This edge may be disjoint if the block is of a complex shape.)
// The interpreter finds the codel of the current colour block on that edge which is furthest to the CC's direction of the DP's direction of travel. (Visualise this as standing on the program and walking in the direction of the DP; see table at right.)
// The interpreter travels from that codel into the colour block containing the codel immediately in the direction of the DP.

use crate::grid::{Color, Grid};

pub(crate) struct Interpreter {
    pos: (usize, usize),
    block_color: Color,
    block_integer: u16,
    hue_change_steps: u8, // tell the command controller
    grid: Grid,
    dp: DP,
    cc: CC,
}

// Color block
// A set of codels (each codel is a struct with a position and color)
// struct Codel {
//     color: Color,
//     pos: (usize, usize),
// }

// Program grid

impl Interpreter {
    pub(crate) fn new(grid: Grid) -> Self {
        let init_pos = (0, 0);
        let init_color = grid.get_color(init_pos);
        // let init_block_integer = grid.find_codel_block(init_pos);
        Interpreter {
            pos: init_pos,
            block_color: init_color,
            block_integer: 0,
            hue_change_steps: 0,
            grid: grid,
            dp: DP::RIGHT,
            cc: CC::LEFT,
        }
    }

    fn step() {
        todo!()
    }

    //TODO: use hashmap or hashset?
    //TODO: decide if this should belong to Grid instead, and Interpreter can just query Grid for the # of codels in the block and the next codel according to DP and CC
    fn find_edge_codels(&self, mut block_codels: Vec<(usize, usize)>) -> Vec<(usize, usize)> { // TODO: decide ownership of block_codels -- maybe it should be a reference slice and the only thing that owns codels is Grid
        let mut edge_codels: Vec<(usize, usize)> = Vec::new();
        if block_codels.is_empty() {
            panic!(); //TODO: find good way of error handling
        }
        match self.dp { // TODO: eliminate repeated code for readability
            DP::RIGHT => {
                block_codels.sort_by_key(|tuple| tuple.0);
                let edge_coord = block_codels.last().unwrap().0;
                for c in block_codels.iter().rev() {
                    if c.0 == edge_coord {
                        edge_codels.push(*c);
                    } else {
                        break;
                    }
                }
            }
            DP::LEFT => {
                block_codels.sort_by_key(|tuple| tuple.0);
                let edge_coord = block_codels.first().unwrap().0;
                for c in block_codels.iter() {
                    if c.0 == edge_coord {
                        edge_codels.push(*c);
                    } else {
                        break;
                    }
                }
            }
            DP::UP => {
                block_codels.sort_by_key(|tuple| tuple.1);
                let edge_coord = block_codels.first().unwrap().1;
                for c in block_codels.iter() {
                    if c.1 == edge_coord {
                        edge_codels.push(*c);
                    } else {
                        break;
                    }
                }
            }
            DP::DOWN => {
                block_codels.sort_by_key(|tuple| tuple.1);
                let edge_coord = block_codels.last().unwrap().1;
                for c in block_codels.iter().rev() {
                    if c.1 == edge_coord {
                        edge_codels.push(*c);
                    } else {
                        break;
                    }
                }
            }
        }
        
        edge_codels

    }

    fn find_corner_codel(&self, mut edge_codels: Vec<(usize, usize)>) -> (usize, usize) {
        if edge_codels.is_empty() {
            panic!();
        }

        todo!();
        // match self.dp {
        //     DP::RIGHT => {
        //         if self.cc == CC::LEFT {

        //         } else {

        //         }
        //     }
        //     DP::DOWN => {
        //         if self.cc == CC::LEFT {

        //         } else {
                    
        //         }
        //     }
        //     DP::LEFT => {
        //         if self.cc == CC::LEFT {

        //         } else {
                    
        //         }
        //     }
        //     DP::UP => {
        //         if self.cc == CC::LEFT {

        //         } else {
                    
        //         }
        //     }
        // }
    }
    /*
     * interpreter loop:
     *
     * state to track across iterations:
     * - index of current codel (pre: new index inside a color block)
     * - DP direction
     * - CC direction
     * - stack
    step
        get color of current codel
        find all codels in current color block (floodfill)
            find block integer

        find edge (DP): returns a few codel options

        find codel (CC): returns one codel option

        move: update color_block
            handle white block
            handle black block or edge

    */
}

#[derive(PartialEq)]
enum DP {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}
#[derive(PartialEq)]
enum CC {
    LEFT,
    RIGHT,
}

#[cfg(test)] 
mod tests {
    use super::Interpreter;
    use super::Grid;
    use crate::interpreter::{DP, CC};
    
    //TODO: decide if I wanna keep these tests. Before writing them, they seemed to be a good idea. But now that I've written them, they seem silly and unnecessary. Maybe the exercise of writing them out has been the valuable part (rather than their existence).
    //TODO: decide if I wanna split up test functions by "function under test" (rather than input type, which is what I'm currently doing)
    #[test]
    fn vertical_colorblock() {
        let mut interp = Interpreter::new(Grid::default());
        let block_codels = vec![(0, 9),
                                                (1, 9), 
                                                (2, 9), 
                                                (3, 9)];
        interp.dp = DP::RIGHT;
        // assert_eq!(interp.find_edge_codels(&block_codels), block_codels);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(&block_codels), (0,9));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(&block_codels), (3,9));

        interp.dp = DP::DOWN;
        // assert_eq!(interp.find_edge_codels(&block_codels), vec![(3,9)]);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(vec![(3,9)]), (3,9));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(vec![(3,9)]), (3,9));

        interp.dp = DP::LEFT;
        // assert_eq!(interp.find_edge_codels(&block_codels), block_codels);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(&block_codels), (3,9));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(&block_codels), (0,9));

        interp.dp = DP::UP;
        // assert_eq!(interp.find_edge_codels(&block_codels), vec![(0,9)]);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(vec![(0,9)), (0,9));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(vec![(0,9)), (0,9));
    }

    #[test]
    fn horizontal_colorblock() {
        let mut interp = Interpreter::new(Grid::default());
        let block_codels = vec![(1,4), (1,5), (1,6), (1,7), (1,8)];
        
        interp.dp = DP::RIGHT;
        // assert_eq!(interp.find_edge_codels(&block_codels), vec![(1, 8)]);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(vec![(1, 8)]), (1,8));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(vec![(1, 8)]), (1,8));

        interp.dp = DP::DOWN;
        // assert_eq!(interp.find_edge_codels(&block_codels), block_codels);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(&block_codels), (1,8));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(&block_codels), (1,4));

        interp.dp = DP::LEFT;
        // assert_eq!(interp.find_edge_codels(&block_codels), vec![(1,4)]);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(vec![(1, 4)]), (1,4));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(vec![(1, 4)]), (1,4));

        interp.dp = DP::UP;
        // assert_eq!(interp.find_edge_codels(&block_codels), block_codels);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(&block_codels), (1,4));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(&block_codels), (1,8));
    }

    #[test]
    fn square_colorblock() {
        let mut interp = Interpreter::new(Grid::default());
        let block_codels = vec![(3,4), (3,5), 
                                                 (4,4), (4,5)];
        interp.dp = DP::RIGHT;
        // assert_eq!(interp.find_edge_codels(&block_codels), vec![(3,5), (4, 5)]);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(vec![(3,5), (4, 5)]), (3,5));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(vec![(3,5), (4, 5)]), (4,5));

        interp.dp = DP::DOWN;
        // assert_eq!(interp.find_edge_codels(&block_codels), vec![(4, 4), (4, 5)]);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(vec![(4, 4), (4, 5)]), (4,5));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(vec![(4, 4), (4, 5)]), (4,4));

        interp.dp = DP::LEFT;
        // assert_eq!(interp.find_edge_codels(&block_codels), vec![(3,4), (4,4)]);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(vec![(3,4), (4,4)), (4,4));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(vec![(3,4), (4,4)), (3,4));

        interp.dp = DP::UP;
        // assert_eq!(interp.find_edge_codels(&block_codels), vec![(3,4), (3,5)]);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(vec![(3,4), (3,5)]), (3,4));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(vec![(3,4), (3,5)]), (3,5));
    }

    #[test]
    fn irregular_shaped_block() {
        let mut interp = Interpreter::new(Grid::default());
        let block_codels = vec![(2,0), (2,1),         (2,3),
                                                 (3,0), (3, 1), (3,2), (3,3),
                                                 (4,0), (4, 1),
                                                        (5, 1) ];
        interp.dp = DP::RIGHT;
        // assert_eq!(interp.find_edge_codels(&block_codels), vec![(2,3), (3,3)]);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(vec![(2,3), (3,3)], (2,3));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(vec![(2,3), (3,3)], (3,3));

        interp.dp = DP::DOWN;
        // assert_eq!(interp.find_edge_codels(&block_codels), vec![(5, 1)]);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(vec![(5, 1)], (5, 1));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(vec![(5, 1)], (5,1));

        interp.dp = DP::LEFT;
        // assert_eq!(interp.find_edge_codels(&block_codels), vec![(2,0), (3,0), (4,0)]);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(vec![(2,0), (3,0), (4,0)], (4,0));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(vec![(2,0), (3,0), (4,0)], (2,0));

        interp.dp = DP::UP;
        // assert_eq!(interp.find_edge_codels(&block_codels), vec![(2,0), (2,1), (2,3)]);
        interp.cc = CC::LEFT;
        // assert_eq!(interp.find_corner_codel(vec![(2,0), (2,1), (2,3)]), (2,0));
        interp.cc = CC::RIGHT;
        // assert_eq!(interp.find_corner_codel(vec![(2,0), (2,1), (2,3)]), (2,3));
    }

}