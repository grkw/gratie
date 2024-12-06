// ./gratie <image file> # start interpreter
//
// repl commands:
// - step: execute one command
// - run: run entire program and print result
//

// The interpreter finds the edge of the current colour block which is furthest in the direction of the DP. (This edge may be disjoint if the block is of a complex shape.)
// The interpreter finds the codel of the current colour block on that edge which is furthest to the CC's direction of the DP's direction of travel. (Visualise this as standing on the program and walking in the direction of the DP; see table at right.)
// The interpreter travels from that codel into the colour block containing the codel immediately in the direction of the DP.

use crate::grid::Color;

struct Interpreter {
    pos: (usize, usize),
    block_color: Color,
    block_integer: u16,
    hue_change_steps: u8, // tell the command controller
}

// Color block
// A set of codels (each codel is a struct with a position and color)

// Program grid

impl Interpreter {
    fn new() -> Self {
        //Interpreter {}
        todo!()
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

            find codel (CC): returns one code option
    DP_CC_dir = ([right, left], [right, right], [down, right], [down, left])
            move: update color_block
                handle white block
                handle black block or edge

        */
}

enum DP {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

enum CC {
    LEFT,
    RIGHT,
}
