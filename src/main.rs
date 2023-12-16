use std::fmt::Debug;

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

    //menambah data
    fn enqueue(&mut self, item: T) {
        self.items.push(item);
    }

    //menghapus data
    fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }

    // cek apakah kosong
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
    
    //queue
    let mut queue = Queue::new();

    queue.enqueue(3);
    queue.enqueue(1);
    queue.enqueue(2);

    queue.print_all();
    
    while let Some(item) = queue.dequeue() {
        println!("Dequeue from queue: {}", item);
    }
}
