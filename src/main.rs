use std::fmt::Debug;

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    // create a new empty stack
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    // push an item onto stack
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}

struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T>
where
    T: Debug,
{
    fn new() -> Self {
        Queue { items: Vec::new() }
    }

    fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn print_all(&self) {
        for item in &self.items {
            println!("Queue item: {:?}", item);
        }
    }
}

fn main() {
    //stack
    let mut stack = Stack::new();

    stack.push(3);
    stack.push(1);
    stack.push(2);

    // Display the data in the stack before popping
    if let Some(top) = stack.pop() {
        println!("Top of the stack: {}", top);
    }

    while let Some(item) = stack.pop() {
        println!("Proped from stack: {}", item);
    }

    //queue
    let mut queue = Queue::new();

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    queue.print_all();
    
    while let Some(item) = queue.dequeue() {
        println!("Dequeue from queue: {}", item);
    }
}
