mod stack; // Lets the compiler know about the `stack` module
mod interpreter;
mod grid;

use stack::Stack; // Brings the `Stack` struct into scope, so you can use `Stack` directly without needing to prefix it with `stack::`.

fn main() {
    let mut interpreter_pos = (0,0);
    let mut s = Stack::<i32>::new();
    // let program p = Vec<Vec::new()>;
}
