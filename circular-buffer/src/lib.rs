use std::collections::VecDeque;

pub struct CircularBuffer<T> {
    items: VecDeque<T>,
    size: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Clone> CircularBuffer<T> {
    pub fn new(size: usize) -> Self {
        CircularBuffer {
            items: VecDeque::<T>::with_capacity(size + 1),
            size: size,
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if let Some(item) = self.items.pop_front() {
            Ok(item.clone())
        } else {
            Err(Error::EmptyBuffer)
        }
    }

    pub fn write(&mut self, item: T) -> Result<(), Error> {
        if self.items.len() == self.size {
            Err(Error::FullBuffer)
        } else {
            self.items.push_back(item);
            Ok(())
        }
    }

    pub fn overwrite(&mut self, item: T) {
        if self.items.len() == self.size {
            self.items.pop_front();
        }
        self.items.push_back(item);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}
