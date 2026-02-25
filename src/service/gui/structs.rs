#[derive(Clone)]
pub struct Counter {
    n: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

impl Counter {
    fn new() -> Counter {
        Self { n: 0 }
    }
    fn next(&mut self) -> u64 {
        self.n += 1;
        self.n
    }
}

#[derive(Clone)]
pub struct IdCounter {
    counter: Counter,
}
impl IdCounter {
    pub fn new() -> Self {
        Self {
            counter: Counter::new(),
        }
    }
    pub fn next(&mut self) -> TaskId {
        TaskId(self.counter.next())
    }
}