use std::fmt::Debug;
use std::io::{self, Write};

struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.elements.push(item);
        println!("Pushed Item");
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn size(&self) -> usize {
        self.elements.len()
    }
}

impl<T: Debug> Stack<T> {
    fn print(&self) {
        println!("{:?}", self.elements);
    }
}

fn main() {
    let mut stack = Stack::new();

    loop {
        println!("\n Generic Stack Menu :");
        println!("1. Push");
        println!("2. Pop");
        println!("3. Peek");
        println!("4. Is Empty");
        println!("5. Size");
        println!("6. Print Stack");
        println!("7. Exit");

        let choice = input("Enter your choice: ");
        match choice.as_str() {
            "1" => {
                let item = input("Enter item to push: ");
                stack.push(item);
            }
            "2" => {
                if let Some(item) = stack.pop() {
                    println!("Popped Item: {}", item);
                } else {
                    println!("Stack is empty");
                }
            }
            "3" => {
                if let Some(item) = stack.peek() {
                    println!("Peeked Item: {}", item);
                } else {
                    println!("Stack is empty");
                }
            }
            "4" => {
                println!("Is Empty: {}", stack.is_empty());
            }
            "5" => {
                println!("Size: {}", stack.size());
            }
            "6" => {
                stack.print();
            }
            "7" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
