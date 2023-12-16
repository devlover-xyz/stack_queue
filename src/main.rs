use std::fmt::Debug;

use tabled::{Tabled, Table};

#[derive(Debug, Tabled)]
struct Mahasiswa {
    nim: String,
    nama: String,
    prodi: String,
}

#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T>
where
    T: Debug,
{
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

#[derive(Debug)]
struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T>
where
    T: Debug + Tabled,
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
        // for item in &self.items {
        //     // println!("Queue item: {:?}", item);
        //     rows.push(item);
        // }

        let table = Table::new(&self.items);
        println!("{}", table);

    }
}

fn main() {
    //stack
    let mut stack = Stack::new();

    let mahasiswa_1 = Mahasiswa {
        nim: String::from("12345"),
        nama: String::from("Abdul"),
        prodi: String::from("TI"),
    };

    let mahasiswa_2 = Mahasiswa {
        nim: String::from("45678"),
        nama: String::from("Zaki"),
        prodi: String::from("MI"),
    };

    stack.push(mahasiswa_1);
    stack.push(mahasiswa_2);

    // Display the data in the stack before popping
    if let Some(top) = stack.pop() {
        println!("Top of the stack: {:?}", top);
    }

    while let Some(item) = stack.pop() {
        println!("Proped from stack: {:?}", item);
    }

    //queue
    let mut queue = Queue::new();

    let mahasiswa_1 = Mahasiswa {
        nim: String::from("12345"),
        nama: String::from("Abdul"),
        prodi: String::from("TI"),
    };

    let mahasiswa_2 = Mahasiswa {
        nim: String::from("45678"),
        nama: String::from("Zaki"),
        prodi: String::from("MI"),
    };

    queue.enqueue(mahasiswa_1);
    queue.enqueue(mahasiswa_2);

    // Print all data in the queue using tabled
    println!("Queue:");
    queue.print_all();

    while let Some(item) = queue.dequeue() {
        println!("Dequeue from queue: {:?}", item);
    }
}
