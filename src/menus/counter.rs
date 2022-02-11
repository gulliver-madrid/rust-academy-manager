pub struct Counter {
    count: u32,
    limit: u32,
}

impl Counter {
    pub fn new(limit: u32) -> Counter {
        Counter { count: 0, limit }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.limit {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
