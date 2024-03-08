use std::cell::Cell;

#[derive(Default)]
pub struct Counter {
    pub(crate) count: Cell<usize>,
}

impl Counter {
    pub const fn new() -> Self {
        Self {
            count: Cell::new(0),
        }
    }

    pub fn increment(&self) {
        let next = self.count.get() + 1;
        self.count.set(next);
    }

    pub fn get(&self) -> usize {
        self.count.get()
    }

    pub fn wrapping_add_signed(&self, rhs: isize) -> usize {
        let count = self.count.get() as isize;
        let result = count.wrapping_add(rhs);
        result as usize
    }
}
