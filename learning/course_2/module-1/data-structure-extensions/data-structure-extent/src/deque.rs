

pub struct Deque<T> {

    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    len: usize
}


impl<T> Deque<T> {

    pub fn with_capacity(capacity: usize) -> Self {

        let mut buffer = Vec::with_capacity(capacity);

        buffer.resize_with(capacity, || None);

        Self {
            buffer,
            head: 0,
            tail: 0,
            len: 0
        }
    }

    fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn push_back(&mut self, value: T) {
        if self.len == self.capacity() {
            panic!("Deque is full");
        }

        self.buffer[self.tail] = Some(value);
        self.tail = (self.tail + 1) % self.capacity();
        self.len += 1;
    }

    pub fn push_front(&mut self, value: T) {
        if self.len == self.capacity() {
            panic!("Deque is full");
        }

        self.head = (self.head + self.capacity() - 1) % self.capacity();
        self.buffer[self.head] = Some(value);
        self.len += 1;
    }


    pub fn pop_back(&mut self) -> Option<T> {

        if self.is_empty() {
            return None;
        }

        let value = self.buffer[self.tail].take();

        self.tail = self.tail.wrapping_sub(1) % self.capacity();
        self.len -= 1;

        value
    }

    pub fn pop_front(&mut self) -> Option<T> {

        if self.is_empty() {
            return None;
        }

        let value = self.buffer[self.head].take();

        self.head = (self.head + 1) % self.capacity();
        self.len -= 1;

        value
    }

    pub fn peek_front(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.buffer[self.head].as_ref()
    }  

    pub fn peek_back(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        let tail_index = (self.tail + self.capacity() - 1) % self.capacity();
        self.buffer[tail_index].as_ref()
    }

    pub fn clear(&mut self) {
        for i in 0..self.capacity() {
            self.buffer[i] = None;
        }
        self.head = 0;
        self.tail = 0;
        self.len = 0;
    }

    pub fn grow(&mut self) {

        let new_capacity = if self.capacity() == 0 { 1 } else { self.capacity() * 2 };

        let mut new_buffer = Vec::with_capacity(new_capacity);

        new_buffer.resize_with(new_capacity, || None);

        for i in 0..self.len {
            let old_index = (self.head + i) % self.capacity();
            new_buffer[i] = self.buffer[old_index].take();
        }

        self.buffer = new_buffer;
        self.head = 0;
        self.tail = self.len;
    }


}


impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.into_iter().filter_map(|x| x).collect::<Vec<_>>().into_iter()
    }
}