use std::{cmp::min};

struct PriorityQueue<T>
where T: Ord,
{
    capacity: usize,
    size: usize,
    data: Vec<T>,
}


impl <T> PriorityQueue<T>
where T: Ord + Copy,
{
    pub fn new() -> Self {
        PriorityQueue::with_capacity(10_000 as usize)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self {
            capacity,
            size: 0,
            data: Vec::with_capacity(capacity),
        }
    }

    fn extend_capacity(&mut self){
        self.capacity += 10_000;
        let mut new_data = Vec::with_capacity(self.capacity);
        new_data.append(&mut self.data);
        self.data = new_data;
    }

    #[inline]
    fn left_down(i: usize) -> usize
    {
        i << 1
    }

    #[inline]
    fn right_down(i: usize) -> usize 
    {
        (i << 1) + 1
    }

    #[inline]
    fn climb_up(i: usize) -> usize 
    {
        i >> 1
    }
    pub fn push(&mut self, item: T){

        if self.size == self.capacity {
            self.extend_capacity();
        }

        self.data.insert(self.size, item);
        self.size += 1;

        let mut cur_p = self.size;

        while cur_p > 1
        {
            let par_p = Self::climb_up(cur_p);

            if self.data[par_p] <= self.data[cur_p]{
                break;
            }

            self.data.swap(par_p, cur_p);

            cur_p = par_p;

        }
    }

    pub fn pop(&mut self) -> Option<T> {

        if self.size == 0 {
            return None;
        }

        self.data.swap(0, self.size);

        let res = self.data[self.size];

        self.size -= 1;

        let mut cur_p = 1;

        while Self::right_down(cur_p) <= self.size 
        {
            let left_p = Self::left_down(cur_p);
            let right_p = Self::right_down(cur_p);

            let cur_min = min(self.data[cur_p], min(self.data[left_p], self.data[right_p]));


            if cur_min == self.data[cur_p]{
                break;
            }

            if cur_min == self.data[left_p]{
                self.data.swap(cur_p, left_p);
                cur_p = left_p;
            }
            else {
                self.data.swap(cur_p, right_p);
                cur_p = right_p;
            }
        }

        if Self::left_down(cur_p) <= self.size && self.data[Self::left_down(cur_p)] < self.data[cur_p] {
            self.data.swap(cur_p, Self::left_down(cur_p));
        }

        Some(res)

    }


    pub fn peek(&self) -> Option<T> {
        if self.size == 0 {
            return None;
        }

        Some(self.data[1])
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn heapify(&mut self, dat: Vec<T>){

        for item in dat.into_iter() {
            self.push(item);
        }
    }

    pub fn from(dat: Vec<T>) -> Self {
        let mut res = Self::new();
        res.heapify(dat);
        res
    }


}

impl<T> Default for PriorityQueue<T>
where T: Ord + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}


impl<T> Clone for PriorityQueue<T>
where T: Ord + Copy,
{
    fn clone(&self) -> Self {
        Self {
            capacity: self.capacity,
            size: self.size,
            data: self.data.clone(),
        }
    }
}

impl<T> IntoIterator for PriorityQueue<T>
where T: Ord + Copy,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {

        let mut res = Vec::with_capacity(self.size);

        let mut replicate = self.clone();


        while let Some(item) = replicate.pop()
        {
            res.push(item);
        }

        res.into_iter()
    }
}


fn main() {
    println!("Hello, world!");
}
