// from rust-nomincon book  and claude code, refer to learn

use std::alloc::{self, Layout};
use std::cmp::Ordering;

use std::fmt;
use std::ptr;


pub struct BinaryHeap<T, F>
where
    F: for<'a, 'b> Fn(&'a T, &'b T) -> Ordering
{
    ptr: *mut T,
    len: usize,
    cap: usize,
    cmp: F,
}

impl<T: Ord> BinaryHeap<T, fn(&T, &T) -> Ordering> {

    pub fn new() -> Self {
        Self::with_comparator(|a: &T, b: &T| a.cmp(b))
    }

    pub fn new_min() -> Self {
        Self::with_comparator(|a: &T, b: &T| b.cmp(a))
    }
}

impl<T, F: Fn(&T, &T) -> Ordering> BinaryHeap<T, F> {
    pub fn with_comparator(cmp: F) -> Self {
        BinaryHeap 
        { ptr: ptr::NonNull::dangling().as_ptr(),
          len: 0, 
          cap: 0, 
          cmp 
        }
    }

    fn layout(cap: usize) -> Layout {
        Layout::array::<T>(cap).expect("layout overflow") 
    }

    unsafe fn grow(&mut self, min_cap: usize)
    {
        let new_cap = min_cap.max(self.cap.saturating_mul(2)).max(4);

        let new_ptr = if self.cap == 0 {
            let layout = Self::layout(self.cap);
            let raw = alloc::alloc(layout);

            if raw.is_null(){
                alloc::handle_alloc_error(layout);
            }

            raw as *mut T

        }else {
            let old_layout = Self::layout(self.cap);
            let new_layout = Self::layout(new_cap);

            let raw = alloc::realloc(self.ptr as *mut u8, old_layout, new_layout.size());

            if raw.is_null(){
                alloc::handle_alloc_error(new_layout);
            }

            raw as *mut T
        };


        self.ptr = new_ptr;
        self.cap = new_cap;
    }

    unsafe fn dealloc(&mut self)
    {
        if self.cap != 0 {
            for i in 0..self.len {
                ptr::drop_in_place(self.ptr.add(i));
            }
            alloc::dealloc(self.ptr as *mut u8, Self::layout(self.cap));

            self.len = 0;
            self.cap = 0;
        }
    }

    #[inline]
    unsafe fn get(&self, i: usize) -> &T {
        &*self.ptr.add(i)
    }

    #[inline]
    unsafe fn swap(&mut self, a: usize, b: usize)
    {
        ptr::swap(self.ptr.add(a), self.ptr.add(b))
    }

    #[inline]
    fn compare(&self, a: usize, b:usize) -> Ordering {

        unsafe {
            (self.cmp)(self.get(a), self.get(b))
        }
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i-1) / 2;

            if self.compare(i, parent) == Ordering::Greater {
                unsafe {self.swap(i, parent)};
                i = parent ;
            }
            else {
                break
            }
        }
    }

    fn sift_down(&mut self, mut i: usize) {

        loop {
            let left = (i << 1) + 1;
            let right = (i << 1) + 2;

            let mut top = i ;

            if left < self.len && self.compare(left, top) == Ordering::Greater {
                top = left;
            }

            if right < self.len && self.compare(right, top) == Ordering::Greater {
                top = right;
            }

            if top == i {
                break;
            }

            unsafe { self.swap(i, top)};

            i = top;

        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            unsafe {self.grow(self.len + 1)}
        }

        unsafe {
            ptr::write(self.ptr.add(self.len), value);
        }

        self.len += 1;

        self.sift_up(self.len - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 
        {
            return None;
        }

        unsafe {
            let top = ptr::read(self.ptr);

            self.len -= 1;

            if self.len > 0 {
                let last = ptr::read(self.ptr.add(self.len));

                ptr::write(self.ptr, last);
                self.sift_down(0);
            }

            Some(top)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.len == 0 {
            None
        }
        else {
            unsafe {Some(self.get(0))}
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn from_slice(data: &[T], cmp: F) -> Self 
    where 
        T: Clone
    {
        let mut heap = Self::with_comparator(cmp);

        if data.is_empty(){
            return heap;
        }

        unsafe {heap.grow(data.len())};

        for (i, item) in data.iter().enumerate() {
            unsafe {ptr::write(heap.ptr.add(i), item.clone())};
        }

        heap.len = data.len();

        let last_parent = (heap.len - 1).saturating_sub(1) / 2;

        for i in (0..=last_parent).rev()
        {
            heap.sift_down(i);
        }

        heap

    }

    pub fn assert_valid(&self) {
        for i in 1..self.len {
            let parent = (i-1) / 2;

            assert!(
                self.compare(parent, i) != Ordering::Less, 
                "heap violation: parent[{parent}] < child[{i}]"
            )
        }
    }
}


impl<T, F: for<'a, 'b> Fn(&'a T, &'b T) -> Ordering> Drop for BinaryHeap<T, F> {
    fn drop(&mut self) {
        unsafe { self.dealloc() }
    }
}

unsafe impl<T: Send, F: Send + Fn(&T, &T) -> Ordering> Send for BinaryHeap<T, F> {}
unsafe impl<T: Sync, F: Sync + Fn(&T, &T) -> Ordering> Sync for BinaryHeap<T, F> {}

impl<T: fmt::Debug, F: Fn(&T, &T) -> Ordering> fmt::Debug for BinaryHeap<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let slice = unsafe { std::slice::from_raw_parts(self.ptr, self.len) };
        f.debug_struct("BinaryHeap")
            .field("len", &self.len)
            .field("cap", &self.cap)
            .field("data", &slice)
            .finish()
    }
}

pub struct IntoIter<T, F: Fn(&T, &T) -> Ordering>(BinaryHeap<T, F>);
 
impl<T, F: Fn(&T, &T) -> Ordering> Iterator for IntoIter<T, F> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.0.pop()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len, Some(self.0.len))
    }
}
 
impl<T, F: Fn(&T, &T) -> Ordering> IntoIterator for BinaryHeap<T, F> {
    type Item = T;
    type IntoIter = IntoIter<T, F>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}