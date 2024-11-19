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

    fn peek(&self) -> Option<&T> { // Option<&T> is a reference to an item in the stack because if we return the item itself, that would mean removing it.
        self.stack.last()
    }
}
