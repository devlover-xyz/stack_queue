use std::fmt::Debug;

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T>
where
    T: Debug,
{
    //new
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    //push
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    //pop
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    //print all data
    fn print_all(&self) {
        for item in &self.items {
            println!("Stack item: {:?}", item);
        }
    }
}

fn main() {
    //stack
    let mut stack = Stack::new();

    stack.push(3);
    stack.push(1);
    stack.push(2);

    //Display all data
    stack.print_all();

    // Display the data in the stack before popping
    if let Some(top) = stack.pop() {
        println!("Top of the stack: {}", top);
    }

    while let Some(item) = stack.pop() {
        println!("Proped from stack: {}", item);
    }
}
