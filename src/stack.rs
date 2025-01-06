use image::codecs::qoi;
use num::Integer;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Stack<T: Integer+Copy+Debug> {
    pub stack: Vec<T>,
}

impl<T: Integer+Copy+Debug> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn pop(&mut self) -> Option<T> {
        println!("POP");
        self.stack.pop() // If the vector is empty, you get a None
    }

    pub fn push(&mut self, item: T) {
        println!("PUSH");
        self.stack.push(item)
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn length(&self) -> usize {
        self.stack.len()
    }

    pub fn peek(&self) -> Option<T> {
       if !self.stack.is_empty() {
        return Some(self.stack[self.length()-1]); // makes a copy
       } else {
            return None;
       }
    }

    pub fn add(&mut self) {
        println!("+");
        let a1 = self.stack.pop();
        let a2 = self.stack.pop();
        if a1.is_some() && a2.is_some() {
            let sum = a1.unwrap() + a2.unwrap();
            self.stack.push(sum);
        } else {
            panic!("Add command needs to have at least 2 values on the stack.");
        }
    }

    pub fn subtract(&mut self) {
        println!("-");
        let s1 = self.stack.pop();
        let s2 = self.stack.pop();
        if s1.is_some() && s2.is_some() {
            let sub = s1.unwrap() - s2.unwrap();
            self.stack.push(sub);
        } else {
            panic!("Subtract command needs to have at least 2 values on the stack."); 
        }
    }

    pub fn multiply(&mut self) {
        println!("*");
        let m1 = self.stack.pop();
        let m2 = self.stack.pop();
        if m1.is_some() && m2.is_some() {
            let prod = m1.unwrap() - m2.unwrap();
            self.stack.push(prod);
        } else {
            panic!("Multiply command needs to have at least 2 values on the stack."); 
        } 
    }

    pub fn divide(&mut self) {
        println!("/");
        let d1 = self.stack.pop();
        let d2 = self.stack.pop();
        if d1.is_some() && d2.is_some() {
            let div = d1.unwrap() - d2.unwrap();
            self.stack.push(div);
        } else {
            panic!("Divide command needs to have at least 2 values on the stack."); 
        }
    }

    pub fn duplicate(&mut self) {
        println!("DUPLICATE");
        let top = self.peek();
      
        if top.is_some() {
            self.stack.push(top.unwrap());
        } else {
            panic!("Duplicate command needs to have at least 1 value on the stack."); 
        }
    }

    pub fn read_in(&mut self, is_char: bool) {
        println!("READIN");
        // if is_char {

        // } else {

        // }
        // let input;//read in
        // self.stack.push(input)
    }

    pub fn write_out(&mut self, is_char: bool) {
        println!("WRITEOUT");
        let top = self.stack.pop();
        if top.is_some() { //char::fromi32
            print!("{:?}", top.unwrap());
        } else {
            panic!("Write command needs to have at least one value on the stack.");
        }
    }
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
