struct Queue<T> {
    queue: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    fn add(&mut self, item: T) {
        self.queue.push(item)
    }

    fn pop(&mut self) -> T {
        self.queue.remove(0)
    }

    fn length(&self) -> usize {
        self.queue.len()
    }
    
    fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}
