pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop() // If the vector is empty, you get a None
    }

    fn push(&mut self, item: T) {
        self.stack.push(item)
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    fn length(&self) -> usize {
        self.stack.len()
    }

    fn peek(&self) -> Option<&T> {
        // Option<&T> is a reference to an item in the stack because if we return the item itself, that would mean removing it.
        self.stack.last()
    }

    fn add(&self) {}

    fn subtract(&self) {}

    fn multiply(&self) {}

    fn divide(&self) {}

    fn duplicate(&self) {}

    fn read_in(&self, is_char: bool) {}

    fn write_out(&self, is_char: bool) {}
}

/*
-5	out(char)
-4	in(number)
-3	pop
-2	divide
-1	subtract
1	add
2	multiply
3	push
4	in(char)
5	out(number)
6	duplicate
*/
