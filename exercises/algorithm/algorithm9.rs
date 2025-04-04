/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default + Clone,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default + Clone,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        self.count += 1;
        let mut cur_index = self.count;
        let mut parent_index = self.parent_idx(cur_index);

        while cur_index != 1 && (self.comparator)(&self.items[cur_index], &self.items[parent_index]) {
            
            let temp = self.items[parent_index].clone();
            self.items[parent_index] = self.items[cur_index].clone();
            self.items[cur_index] = temp;

            cur_index = parent_index;         
            parent_index = self.parent_idx(cur_index);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        assert!(self.children_present(idx));

        let left_child_index = self.left_child_idx(idx);
        let right_child_index = self.right_child_idx(idx);
        if right_child_index <= self.count {
            if (self.comparator)(&self.items[right_child_index], &self.items[left_child_index]) {
                return right_child_index
            }
        }

		left_child_index
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Clone,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> { //相当于是pop
        if self.count > 0 {
            let res = Some(self.items[1].clone());
            if self.count == 1 {
                self.items.pop();
                self.count = 0;
                return res 
            }

            let last = self.items.pop().unwrap();
            self.count -= 1;
            self.items[1] = last;

            let mut cur_index = 1;
            while self.children_present(cur_index) {
                let children_index = self.smallest_child_idx(cur_index);
                if (self.comparator)(&self.items[children_index], &self.items[cur_index]) {
                    let temp = self.items[children_index].clone();
                    self.items[children_index] = self.items[cur_index].clone();
                    self.items[cur_index] = temp;

                    cur_index = children_index;
                } else {
                    break;
                }
            }

            
            return res
        }
		None
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: Clone>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: Clone>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}