mod stack; // Lets the compiler know about the `stack` module

use stack::Stack; // Brings the `Stack` struct into scope, so you can use `Stack` directly without needing to prefix it with `stack::`.

fn main() {
    let mut s = Stack::<i32>::new();
}
